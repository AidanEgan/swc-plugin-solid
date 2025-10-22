/*
 * JSX Opening Element can have a bunch of properties that have custom logic for how to build them.
 * Notably, properties like class/className, ref, and style all have custom logic
 * Additionally there is cusom logic for expressions and spread exprs
 * TODO: SOME CUSTOM COMPONENT LOGIC LIVES IN STD BUILDER BUT SHOULD PROBABLY BE MOVED HERE
 * SINCE OTHER LOGIC WILL LIVE HERE ANYWAY
 */

pub mod class_builder;
mod class_list_builder;
pub mod event_handler_builder;
pub mod helpers;
pub mod ref_statement_builder;
pub mod require_prop_builder;
pub mod standard_prop_builder;
pub mod style_builder;

use swc_core::{
    atoms::Atom,
    common::{Spanned, SyntaxContext, DUMMY_SP},
    ecma::{
        ast::{
            BlockStmtOrExpr, CallExpr, Expr, ExprStmt, Ident, JSXAttrName, JSXAttrOrSpread,
            JSXAttrValue, JSXExpr, Lit, MemberExpr, ObjectLit, Prop, PropName, Stmt, Tpl,
            TplElement,
        },
        visit::VisitMutWith,
    },
};

use crate::{
    builder::{
        client::{
            builder_helpers::wrap_in_empty_arrow,
            effect_builder::{EffectBuilder, EffectMetadata},
            element_properties::helpers::{
                generate_effect_assignment, merge_prop_as_getter, merge_prop_as_kv,
                template_lit_class_expr, EffectOrInlineOrExpression, PossibleEffectStatement,
            },
            jsx_expr_builder_client::{build_js_from_client_jsx, standard_build_res_wrappings},
            jsx_expr_transformer_client::ClientJsxExprTransformer,
            jsx_parser_client::ClientJsxElementVisitor,
        },
        parser_types::JsxOpeningMetadata,
    },
    constants::properties::get_bool_attr,
    helpers::{
        common_into_expressions::{create_double_negated, ident_callee, ident_expr},
        component_helpers::is_falsy,
        generate_var_names::{
            generate_effect_arg, generate_el, generate_merge_props, generate_v, SPREAD, USE,
        },
    },
    transform::{parent_visitor::ParentVisitor, scope_manager::TrackedVariable},
};

struct WrapEffect {
    data: Option<(Atom, Box<Expr>)>,
}

impl WrapEffect {
    fn from(data: Option<(Atom, Box<Expr>)>) -> Self {
        Self { data }
    }
    fn and_then<T: FnOnce()>(self, callback: T) -> Self {
        if self.data.is_none() {
            callback();
        }
        self
    }
    fn or_else<T: FnOnce(Atom, Box<Expr>)>(self, callback: T) {
        if let Some(data) = self.data {
            callback(data.0, data.1);
        }
    }
}

/*
 *
 * Used for opening elements on non-Custom components
 * ref implementation is partially used for custom compoennt
 * due to similar impelementation logic.
 *
 * Take a series of kv pairs with raw values,
 * transforms raw values into final exprs
 * generates a series of stmt values from those exprs
 * stmts are inserted as needed
 */

#[derive(Debug)]
pub enum StatementInserts {
    TextContentData,
}

#[derive(Debug)]
pub enum ClassBuilderVariants {
    Lit(String),
    Computed(Box<Expr>),
    ComputedEffect(Box<Expr>),
}

#[derive(Debug)]
pub struct ClassBuilder {
    class_name: bool,
    is_effect: bool,
    is_lit: bool,
}

#[derive(Debug)]
pub struct ElementPropertiesBuilder<'a, T: ParentVisitor> {
    pub statements: Vec<Stmt>,
    pub direct_template_inserts: Vec<(String, String)>,
    pub statement_inserts: Vec<StatementInserts>,
    pub parent_visitor: &'a mut T,
    effect_builder: &'a mut EffectBuilder,
    tmp_wrap_effect: bool,
    class_builder_data: (Option<bool>, Vec<ClassBuilderVariants>),
}

// Implements many fns for different types of properties
// See 'element properties' folder
impl<'a, T: ParentVisitor> ElementPropertiesBuilder<'a, T> {
    fn check_class_name_in_scope(
        &mut self,
        initial_val: &Atom,
    ) -> Result<String, Option<TrackedVariable>> {
        let mut val = initial_val;
        loop {
            match self.parent_visitor.get_var_if_in_scope(val) {
                Some(TrackedVariable::FunctionIdent(is_const)) => {
                    break Err(Some(TrackedVariable::FunctionIdent(*is_const)))
                }
                Some(TrackedVariable::Literal(l)) => break Ok(l.clone()),
                Some(TrackedVariable::Referred(r)) => {
                    val = r;
                }
                Some(TrackedVariable::StoredConstant) => {
                    break Err(Some(TrackedVariable::StoredConstant))
                }
                Some(TrackedVariable::Imported) => break Err(Some(TrackedVariable::Imported)),
                None => break Err(None),
            }
        }
    }
    fn effect_or_inline_or_expr(
        &mut self,
        data: PossibleEffectStatement,
    ) -> EffectOrInlineOrExpression {
        match data {
            PossibleEffectStatement::Std(expr) => match *expr {
                Expr::Lit(Lit::Str(s)) => {
                    EffectOrInlineOrExpression::InlineRes(s.value.as_str().into())
                }
                Expr::Lit(Lit::Num(n)) => {
                    EffectOrInlineOrExpression::InlineRes(n.value.to_string())
                }
                e => {
                    let inline = if let Some(ident) = e.as_ident() {
                        self.check_class_name_in_scope(&ident.sym).ok()
                    } else {
                        None
                    };
                    if let Some(inline) = inline {
                        EffectOrInlineOrExpression::InlineRes(inline)
                    } else {
                        EffectOrInlineOrExpression::ExpressionRes(e.into())
                    }
                }
            },
            PossibleEffectStatement::Effect(e) => {
                EffectOrInlineOrExpression::EffectRes((generate_effect_assignment(&e), e))
            }
        }
    }

    pub fn new(parent_visitor: &'a mut T, effect_builder: &'a mut EffectBuilder) -> Self {
        Self {
            statements: Vec::new(),
            direct_template_inserts: Vec::new(),
            statement_inserts: Vec::new(),
            parent_visitor,
            effect_builder,
            tmp_wrap_effect: false,
            class_builder_data: (None, Vec::new()),
        }
    }

    pub fn std_transform(
        &mut self,
        attr: JSXAttrOrSpread,
        transform_call_exprs: bool,
        default_val: Option<Box<Expr>>,
    ) -> Box<Expr> {
        match attr {
            JSXAttrOrSpread::JSXAttr(jsxattr) => match jsxattr.value {
                Some(JSXAttrValue::JSXElement(mut jsxe)) => {
                    let mut sub_visitor = ClientJsxElementVisitor::new();
                    jsxe.visit_mut_with(&mut sub_visitor);
                    standard_build_res_wrappings(build_js_from_client_jsx(
                        sub_visitor,
                        self.parent_visitor,
                    ))
                }
                Some(JSXAttrValue::JSXFragment(mut jsxf)) => {
                    let mut sub_visitor = ClientJsxElementVisitor::new();
                    jsxf.visit_mut_with(&mut sub_visitor);
                    standard_build_res_wrappings(build_js_from_client_jsx(
                        sub_visitor,
                        self.parent_visitor,
                    ))
                }
                Some(JSXAttrValue::JSXExprContainer(exp)) => {
                    let has_static_marker = self.parent_visitor.has_static_marker(exp.span_lo());
                    match exp.expr {
                        // Undefined behavior seemingly
                        swc_core::ecma::ast::JSXExpr::JSXEmptyExpr(_) => Box::new(Expr::default()),
                        swc_core::ecma::ast::JSXExpr::Expr(mut expr) => {
                            let mut sub_visitor = ClientJsxExprTransformer::new(
                                self.parent_visitor,
                                transform_call_exprs,
                                false, // Memo bin + cond only in custom compnents I think
                            );
                            if transform_call_exprs {
                                sub_visitor.visit_and_wrap_outer_expr(&mut expr, true);
                            } else {
                                expr.visit_mut_with(&mut sub_visitor);
                            }
                            self.tmp_wrap_effect =
                                !has_static_marker && sub_visitor.should_wrap_in_effect;
                            expr
                        }
                    }
                }
                Some(JSXAttrValue::Lit(lit)) => Box::new(lit.into()),
                None => {
                    // Allows caller to define what 'no value' means
                    // Defaults to empty string
                    if let Some(default_val) = default_val {
                        default_val
                    } else {
                        if let JSXAttrName::Ident(name) = jsxattr.name {
                            if get_bool_attr(&name.sym, None).is_some() {
                                return Box::new(Expr::Lit(Lit::Bool(true.into())));
                            }
                        }
                        Box::new(Expr::Lit(Lit::Str("".into())))
                    }
                }
            },
            JSXAttrOrSpread::SpreadElement(spread_element) => spread_element.expr,
        }
    }

    pub fn build_effect_statement(&mut self, build_for_isolated_effect: bool) -> Option<Stmt> {
        if build_for_isolated_effect {
            self.effect_builder.set_uses_effect(false);
        }
        let mut data = if !build_for_isolated_effect {
            std::mem::take(&mut self.effect_builder.data)
        } else {
            if let Some(eff) = self.effect_builder.isolated_effects.pop() {
                vec![eff]
            } else {
                vec![]
            }
        };
        if data.is_empty() {
            return None;
        } else if data.len() == 1 {
            let raw = data.pop().unwrap();
            match raw.variant {
                super::effect_builder::EffectVariant::Class => {
                    self.class_builder(raw.count, PossibleEffectStatement::Std(raw.expr));
                }
                super::effect_builder::EffectVariant::ClassName => {
                    self.class_builder(raw.count, PossibleEffectStatement::Std(raw.expr));
                }
                super::effect_builder::EffectVariant::Prop => {
                    self.require_prop_builder(
                        raw.name,
                        raw.count,
                        PossibleEffectStatement::Std(raw.expr),
                        false,
                    );
                }
                super::effect_builder::EffectVariant::ClassList(is_toggle) => {
                    self.class_list_builder(
                        raw.count,
                        if is_toggle {
                            Some(raw.name.as_str())
                        } else {
                            None
                        },
                        PossibleEffectStatement::Std(raw.expr),
                        true,
                    );
                }
                super::effect_builder::EffectVariant::Style(key) => match key {
                    Some(key) => {
                        self.individual_style_builder(
                            raw.count,
                            key,
                            PossibleEffectStatement::Std(raw.expr),
                        );
                    }
                    None => {
                        let effect_arg = self.effect_builder.get_effect_arg();
                        self.style_builder(
                            raw.count,
                            PossibleEffectStatement::Std(raw.expr),
                            Some(ident_expr(effect_arg)),
                        );
                    }
                },
                super::effect_builder::EffectVariant::Std(is_bool) => {
                    self.standard_prop_builder(
                        raw.name,
                        raw.count,
                        is_bool,
                        PossibleEffectStatement::Std(raw.expr),
                    );
                }
            }
        } else {
            for datum in data {
                match datum.variant {
                    super::effect_builder::EffectVariant::Class => {
                        self.effect_builder
                            .add_var_decl(datum.v_val.clone(), Some(datum.expr));
                        self.class_builder(
                            datum.count,
                            PossibleEffectStatement::Effect((datum.v_val, datum.obj)),
                        );
                    }
                    super::effect_builder::EffectVariant::ClassName => {
                        self.effect_builder
                            .add_var_decl(datum.v_val.clone(), Some(datum.expr));
                        self.class_name_builder(
                            datum.count,
                            PossibleEffectStatement::Effect((datum.v_val, datum.obj)),
                        );
                    }
                    super::effect_builder::EffectVariant::Prop => {
                        self.effect_builder
                            .add_var_decl(datum.v_val.clone(), Some(datum.expr));
                        self.require_prop_builder(
                            datum.name,
                            datum.count,
                            PossibleEffectStatement::Effect((datum.v_val, datum.obj)),
                            false,
                        );
                    }
                    super::effect_builder::EffectVariant::ClassList(is_toggle) => {
                        self.effect_builder.add_var_decl(
                            datum.v_val.clone(),
                            Some(if is_toggle {
                                create_double_negated(datum.expr)
                            } else {
                                datum.expr
                            }),
                        );
                        self.class_list_builder(
                            datum.count,
                            if is_toggle {
                                Some(datum.name.as_str())
                            } else {
                                None
                            },
                            PossibleEffectStatement::Effect((datum.v_val, datum.obj)),
                            true,
                        );
                    }
                    super::effect_builder::EffectVariant::Style(key) => {
                        self.effect_builder
                            .add_var_decl(datum.v_val.clone(), Some(datum.expr));
                        match key {
                            Some(key) => {
                                self.individual_style_builder(
                                    datum.count,
                                    key,
                                    PossibleEffectStatement::Effect((datum.v_val, datum.obj)),
                                );
                            }
                            None => {
                                self.style_builder(
                                    datum.count,
                                    PossibleEffectStatement::Effect((datum.v_val, datum.obj)),
                                    None,
                                );
                            }
                        }
                    }
                    super::effect_builder::EffectVariant::Std(is_bool) => {
                        self.effect_builder
                            .add_var_decl(datum.v_val.clone(), Some(datum.expr));
                        self.standard_prop_builder(
                            datum.name,
                            datum.count,
                            is_bool,
                            PossibleEffectStatement::Effect((datum.v_val, datum.obj)),
                        );
                    }
                }
            }
        }
        self.effect_builder
            .build_effect(std::mem::take(&mut self.statements))
    }

    fn wrap_effect(
        &mut self,
        expr: Box<Expr>,
        element_count: usize,
        variant: super::effect_builder::EffectVariant,
        name: Atom,
    ) -> WrapEffect {
        if self.tmp_wrap_effect {
            if name.as_str() == "checked" || name.as_str() == "value" {
                self.effect_builder.isolated_effects.push(EffectMetadata {
                    expr,
                    variant,
                    name,
                    v_val: Ident::default(),    // Dummy data not used
                    obj: MemberExpr::default(), // Dummy data not used
                    count: element_count,
                });
                self.tmp_wrap_effect = false;
                return WrapEffect::from(None);
            }
            let assigned_v = self.parent_visitor.v_count();
            let assigned_prop = self.effect_builder.get_new_obj();
            self.effect_builder.data.push(EffectMetadata {
                expr,
                variant,
                name,
                v_val: generate_v(*assigned_v).into(),
                obj: assigned_prop,
                count: element_count,
            });

            *assigned_v += 1;
            self.tmp_wrap_effect = false;
            WrapEffect::from(None)
        } else {
            WrapEffect::from(Some((name, expr)))
        }
    }

    fn build_individual_property_statement(
        &mut self,
        element_count: usize,
        name: Atom,
        attr: JSXAttrOrSpread,
        el_name: &str,
    ) {
        match name.as_str() {
            "class" => {
                let expr = self.std_transform(attr, false, None);
                if self.class_builder_data.0.is_none() {
                    self.class_builder_data.0 = Some(true);
                }
                self.class_delegator(expr);
            }
            "className" => {
                let expr = self.std_transform(attr, false, None);
                if self.class_builder_data.0.is_none() {
                    self.class_builder_data.0 = Some(false);
                }
                self.class_delegator(expr);
            }
            "classList" => {
                self.class_list_delegator(element_count, attr);
            }
            "style" => {
                self.parse_all_styles(element_count, attr);
            }
            "ref" => {
                let expr = self.std_transform(attr, false, None);
                self.ref_builder(Some(element_count), expr);
            }
            "textContent" => {
                let expr = self.std_transform(attr, false, None);
                self.wrap_effect(
                    expr,
                    element_count + 1,
                    super::effect_builder::EffectVariant::Prop,
                    "data".into(),
                )
                .and_then(|| {
                    *(self.parent_visitor.element_count()) += 1;
                    self.statement_inserts
                        .push(StatementInserts::TextContentData);
                })
                .or_else(|_, expr| {
                    self.require_prop_builder(
                        name,
                        element_count,
                        PossibleEffectStatement::Std(expr),
                        false,
                    );
                });
            }
            "innerHTML" | "innerText" | "children" => {
                let expr = self.std_transform(attr, false, None);
                self.wrap_effect(
                    expr,
                    element_count,
                    super::effect_builder::EffectVariant::Prop,
                    name,
                )
                .or_else(|name, expr| {
                    self.require_prop_builder(
                        name,
                        element_count,
                        PossibleEffectStatement::Std(expr),
                        false,
                    );
                });
            }
            prop if prop.starts_with("on") => {
                let expr = self.std_transform(attr, false, None);
                self.event_handler_builder(prop, element_count, expr);
            }
            prop => {
                // Event handling
                if let Some(attr_name) = get_bool_attr(prop, Some(el_name)) {
                    let expr = self.std_transform(attr, false, Some(Lit::Bool(true.into()).into()));
                    self.wrap_effect(
                        expr,
                        element_count,
                        super::effect_builder::EffectVariant::Prop,
                        attr_name.into(),
                    )
                    .or_else(|name, expr| {
                        self.require_prop_builder(
                            name,
                            element_count,
                            PossibleEffectStatement::Std(expr),
                            true,
                        );
                    });
                } else if let Some(suffix) = prop
                    .strip_prefix("style:")
                    .map(|key| PropName::Str(key.into()))
                {
                    let expr = self.std_transform(attr, false, None);
                    if self.tmp_wrap_effect {
                        self.style_and_effect(element_count, expr, Some(suffix));
                        self.tmp_wrap_effect = false;
                    } else {
                        self.individual_style_builder(
                            element_count,
                            suffix,
                            PossibleEffectStatement::Std(expr),
                        );
                    }
                } else if let Some(suffix) = prop.strip_prefix("class:") {
                    let expr = self.std_transform(attr, false, None);
                    self.wrap_effect(
                        expr,
                        element_count,
                        super::effect_builder::EffectVariant::ClassList(true),
                        suffix.into(),
                    )
                    .or_else(|name, expr| {
                        self.class_list_builder(
                            element_count,
                            Some(name.as_str()),
                            PossibleEffectStatement::Std(expr),
                            false,
                        );
                    });
                } else if let Some(suffix) = prop.strip_prefix("use:") {
                    let expr = self.std_transform(attr, false, Some(Lit::Bool(true.into()).into()));
                    self.parent_visitor.add_import(USE.into());
                    let use_expr = CallExpr {
                        span: DUMMY_SP,
                        ctxt: SyntaxContext::empty(),
                        callee: ident_callee(USE.into()),
                        type_args: None,
                        args: vec![
                            ident_expr(suffix.into()).into(),
                            ident_expr(generate_el(element_count)).into(),
                            Expr::from(wrap_in_empty_arrow(BlockStmtOrExpr::Expr(expr).into()))
                                .into(),
                        ],
                    };
                    self.statements.push(Stmt::Expr(ExprStmt {
                        span: DUMMY_SP,
                        expr: use_expr.into(),
                    }));
                } else if let Some(suffix) = prop.strip_prefix("prop:") {
                    let expr = self.std_transform(attr, false, None);
                    let suffix: Atom = suffix.into();
                    self.wrap_effect(
                        expr,
                        element_count,
                        super::effect_builder::EffectVariant::Prop,
                        suffix,
                    )
                    .or_else(|suffix, expr| {
                        self.require_prop_builder(
                            suffix,
                            element_count,
                            PossibleEffectStatement::Std(expr),
                            false,
                        );
                    });
                } else {
                    let mut is_bool = false;
                    // 'attr:' means specify this is an attribute. B/c of the
                    // logic we don't have to check for it, but it needs to be stripped
                    let name: Atom = prop
                        .strip_prefix("attr:")
                        .or_else(|| {
                            let split_bool = prop.strip_prefix("bool:");
                            is_bool = split_bool.is_some();
                            split_bool
                        })
                        .unwrap_or(prop.into())
                        .into();
                    let expr = self.std_transform(
                        attr,
                        false,
                        if is_bool {
                            Some(Lit::Bool(true.into()).into())
                        } else {
                            None
                        },
                    );
                    // Idk why this is an exception
                    if is_bool && expr.is_arrow() {
                        self.tmp_wrap_effect = false;
                    }
                    self.wrap_effect(
                        expr,
                        element_count,
                        super::effect_builder::EffectVariant::Std(is_bool),
                        name,
                    )
                    .or_else(|name, expr| {
                        self.standard_prop_builder(
                            name,
                            element_count,
                            is_bool,
                            PossibleEffectStatement::Std(expr),
                        );
                    });
                }
            }
        }
    }

    fn build_final_class_data(&mut self, element_count: usize) {
        if !self.class_builder_data.1.is_empty() {
            let mut is_effect = false;
            let mut is_computed = false;
            let mut lit_builder = "".to_string();
            let mut quasis: Vec<TplElement> = vec![];
            let mut exprs: Vec<Box<Expr>> = vec![];

            let (is_class, mut class_builder_data) =
                std::mem::replace(&mut self.class_builder_data, (None, vec![]));

            let expr: Box<Expr> = if class_builder_data.len() == 1 {
                match class_builder_data.pop().expect("Vec should not be empty") {
                    ClassBuilderVariants::Lit(l) => Expr::Lit(Lit::Str(l.into())).into(),
                    ClassBuilderVariants::Computed(expr) => {
                        if expr.is_ident() {
                            let id = expr.as_ident().unwrap();
                            let val_in_scope = self.check_class_name_in_scope(&id.sym);
                            if let Ok(val_in_scope) = val_in_scope {
                                Expr::Lit(Lit::Str(val_in_scope.into())).into()
                            } else {
                                expr
                            }
                        } else {
                            expr
                        }
                    }
                    ClassBuilderVariants::ComputedEffect(expr) => {
                        is_effect = true;
                        expr
                    }
                }
            } else {
                for data in class_builder_data {
                    let expr = match data {
                        ClassBuilderVariants::Lit(s) => {
                            lit_builder += s.as_str();
                            lit_builder += " ";
                            continue;
                        }
                        ClassBuilderVariants::Computed(expr) => {
                            is_computed = true;
                            if expr.is_ident() {
                                let id = expr.as_ident().unwrap();
                                let val_in_scope = self.check_class_name_in_scope(&id.sym);
                                if let Ok(val_in_scope) = val_in_scope {
                                    lit_builder += val_in_scope.as_str();
                                    lit_builder += " ";
                                    continue;
                                }
                            }
                            expr
                        }
                        ClassBuilderVariants::ComputedEffect(expr) => {
                            is_computed = true;
                            is_effect = true;
                            expr
                        }
                    };
                    let transformed: Atom =
                        lit_builder.replace("\r\n", "\n").replace('\r', "\n").into();
                    exprs.push(template_lit_class_expr(expr));
                    quasis.push(TplElement {
                        span: DUMMY_SP,
                        tail: false,
                        cooked: Some(transformed.clone()),
                        raw: transformed,
                    });
                    lit_builder = "".to_string();
                }
                let transformed: Atom = lit_builder
                    .trim_end()
                    .replace("\r\n", "\n")
                    .replace('\r', "\n")
                    .into();
                if !is_computed {
                    // All lit strings
                    Expr::Lit(Lit::Str(transformed.into())).into()
                } else {
                    // Has computed values
                    quasis.push(TplElement {
                        span: DUMMY_SP,
                        tail: true,
                        cooked: Some(transformed.clone()),
                        raw: transformed,
                    });
                    Expr::Tpl(Tpl {
                        span: DUMMY_SP,
                        exprs,
                        quasis,
                    })
                    .into()
                }
            };

            self.tmp_wrap_effect = is_effect;
            self.wrap_effect(
                expr,
                element_count,
                if is_class.unwrap_or(true) {
                    super::effect_builder::EffectVariant::Class
                } else {
                    super::effect_builder::EffectVariant::ClassName
                },
                "class".into(),
            )
            .or_else(|_, expr| {
                if is_class.unwrap_or(true) {
                    self.class_builder(element_count, PossibleEffectStatement::Std(expr))
                } else {
                    self.class_name_builder(element_count, PossibleEffectStatement::Std(expr));
                }
            });
        }
    }

    // Needs to take opening metadata and parse it all out
    // Things to keep in mind
    // 1. Refs always apply
    // 2. spreads are weird. Causes a mergeprops but only affects props that come after
    // 3. some props get inlined into template
    pub fn build_el_property_statements(
        &mut self,
        data: JsxOpeningMetadata,
        element_count: usize,
    ) -> (String, bool) {
        let old_el_count = self.effect_builder.data.len();
        let old_isolated_count = self.effect_builder.isolated_effects.len();
        // More complex!
        if data.has_spread {
            let mut seen_first_spread = false;
            let mut merge_props_args: Vec<Box<Expr>> = vec![];
            for (name, attr) in data.attrs {
                if let JSXAttrOrSpread::SpreadElement(spread_el) = attr {
                    merge_props_args.push(spread_el.expr);
                    seen_first_spread = true;
                } else {
                    if let Some(name) = name {
                        let is_need_getter = if let JSXAttrOrSpread::JSXAttr(jsx_a) = &attr {
                            if let Some(JSXAttrValue::JSXExprContainer(ec)) = &jsx_a.value {
                                if let JSXExpr::Expr(e) = &ec.expr {
                                    e.is_call()
                                } else {
                                    false
                                }
                            } else {
                                false
                            }
                        } else {
                            false
                        };
                        // Might need to be more complex
                        let must_build_alone = name.contains(":");

                        if !must_build_alone && (seen_first_spread || is_need_getter) {
                            let res = self.std_transform(attr, false, None);
                            let prop: Box<Prop> = if is_need_getter || self.tmp_wrap_effect {
                                self.tmp_wrap_effect = false;
                                merge_prop_as_getter(name, res)
                            } else {
                                merge_prop_as_kv(name, res)
                            };
                            if let Some(Some(obj)) =
                                merge_props_args.last_mut().map(|x| x.as_mut_object())
                            {
                                obj.props.push(prop.into());
                            } else {
                                merge_props_args.push(
                                    ObjectLit {
                                        span: DUMMY_SP,
                                        props: vec![prop.into()],
                                    }
                                    .into(),
                                );
                            };
                        } else {
                            self.build_individual_property_statement(
                                element_count,
                                name,
                                attr,
                                &data.value,
                            );
                            // Each class data is treated indipendant in the context
                            // of a spread
                            self.build_final_class_data(element_count);
                        }
                    };
                }
            }

            // Done here b/c vec will be consumed after
            if merge_props_args.len() > 1 {
                self.parent_visitor
                    .add_import(generate_merge_props().as_str().into());
            }
            let merge_props_final: Box<Expr> = if merge_props_args.len() == 1 {
                let mut lone_expr = merge_props_args.pop().unwrap();
                if !self.parent_visitor.has_static_marker(lone_expr.span_lo()) {
                    let mut little_visitor =
                        ClientJsxExprTransformer::new(self.parent_visitor, true, false);
                    little_visitor.visit_and_wrap_outer_expr(&mut lone_expr, true);
                    if little_visitor.should_wrap_in_effect {
                        self.parent_visitor
                            .add_import(generate_merge_props().as_str().into());
                        CallExpr {
                            span: DUMMY_SP,
                            ctxt: SyntaxContext::empty(),
                            callee: ident_callee(generate_merge_props()),
                            args: vec![lone_expr.into()],
                            type_args: None,
                        }
                        .into()
                    } else {
                        lone_expr
                    }
                } else {
                    lone_expr
                }
            } else {
                CallExpr {
                    span: DUMMY_SP,
                    ctxt: SyntaxContext::empty(),
                    callee: ident_callee(generate_merge_props()),
                    args: merge_props_args.into_iter().map(|e| e.into()).collect(),
                    type_args: None,
                }
                .into()
            };

            self.statements.push(Stmt::Expr(ExprStmt {
                span: DUMMY_SP,
                expr: CallExpr {
                    span: DUMMY_SP,
                    ctxt: SyntaxContext::empty(),
                    callee: ident_callee(SPREAD.into()),
                    args: vec![
                        ident_expr(generate_el(element_count)).into(),
                        merge_props_final.into(),
                        Expr::Lit(Lit::Bool(data.is_svg.into())).into(),
                        Expr::Lit(Lit::Bool(data.has_children.into())).into(),
                    ],
                    type_args: None,
                }
                .into(),
            }));
            self.parent_visitor.add_import(SPREAD.into());
        } else {
            for (name, attr) in data.attrs {
                self.build_individual_property_statement(
                    element_count,
                    name.expect("All non-spread properties should have a name"),
                    attr,
                    &data.value,
                );
                self.tmp_wrap_effect = false;
            }
            self.build_final_class_data(element_count);
        }
        let all_data_inlined = old_el_count == self.effect_builder.data.len()
            && old_isolated_count == self.effect_builder.isolated_effects.len()
            && self.statements.is_empty()
            && self.statement_inserts.is_empty();

        (data.value, all_data_inlined)
    }
}

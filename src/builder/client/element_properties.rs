/*
 * JSX Opening Element can have a bunch of properties that have custom logic for how to build them.
 * Notably, properties like class/className, ref, and style all have custom logic
 * Additionally there is cusom logic for expressions and spread exprs
 * TODO: SOME CUSTOM COMPONENT LOGIC LIVES IN STD BUILDER BUT SHOULD PROBABLY BE MOVED HERE
 * SINCE OTHER LOGIC WILL LIVE HERE ANYWAY
 */

pub mod class_builder;
pub mod event_handler_builder;
pub mod helpers;
pub mod ref_statement_builder;
pub mod standard_prop_builder;
pub mod style_builder;

use swc_core::{
    atoms::Atom,
    common::{SyntaxContext, DUMMY_SP},
    ecma::{
        ast::{
            CallExpr, Expr, ExprStmt, JSXAttrOrSpread, JSXAttrValue, JSXExpr, Lit, ObjectLit, Prop,
            PropName, Stmt,
        },
        visit::VisitMutWith,
    },
};

use crate::{
    builder::{
        client::{
            effect_builder::{EffectBuilder, EffectMetadata},
            element_properties::helpers::{
                generate_effect_assignment, merge_prop_as_getter, merge_prop_as_kv,
                EffectOrInlineOrExpression, PossibleEffectStatement,
            },
            jsx_expr_builder_client::{build_js_from_client_jsx, standard_build_res_wrappings},
            jsx_expr_transformer_client::ClientJsxExprTransformer,
            jsx_parser_client::ClientJsxElementVisitor,
        },
        parser_types::JsxOpeningMetadata,
    },
    helpers::{
        common_into_expressions::{ident_callee, ident_expr},
        generate_var_names::{generate_el, generate_merge_props, generate_v, SPREAD},
    },
    transform::{parent_visitor::ParentVisitor, scope_manager::TrackedVariable},
};

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
pub struct ElementPropertiesBuilder<'a, T: ParentVisitor> {
    pub statements: Vec<Stmt>,
    pub direct_template_inserts: Vec<(String, String)>,
    pub parent_visitor: &'a mut T,
    effect_builder: &'a mut EffectBuilder,
    tmp_wrap_effect: bool,
}

// Implements many fns for different types of properties
// See 'element properties' folder
impl<'a, T: ParentVisitor> ElementPropertiesBuilder<'a, T> {
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
                        let mut val = &ident.sym;
                        loop {
                            match self.parent_visitor.get_var_if_in_scope(val) {
                                Some(TrackedVariable::FunctionIdent) => break None,
                                Some(TrackedVariable::Literal(l)) => break Some(l.clone()),
                                Some(TrackedVariable::Referred(r)) => {
                                    val = r;
                                }
                                None => break None,
                            }
                        }
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
            statements: vec![],
            direct_template_inserts: Vec::new(),
            parent_visitor,
            effect_builder,
            tmp_wrap_effect: false,
        }
    }

    pub fn std_transform(
        &mut self,
        attr: JSXAttrOrSpread,
        transform_call_exprs: bool,
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
                                sub_visitor.visit_and_wrap_outer_expr(&mut expr);
                            } else {
                                expr.visit_mut_with(&mut sub_visitor);
                            }
                            self.tmp_wrap_effect = sub_visitor.should_wrap_in_effect;
                            expr
                        }
                    }
                }
                Some(JSXAttrValue::Lit(lit)) => Box::new(lit.into()),
                None => Box::new(Expr::Lit(Lit::Str("".into()))),
            },
            JSXAttrOrSpread::SpreadElement(spread_element) => spread_element.expr,
        }
    }

    pub fn build_effect_statement(&mut self) -> Option<Stmt> {
        let mut data = std::mem::take(&mut self.effect_builder.data);
        if data.is_empty() {
            return None;
        } else if data.len() == 1 {
            let raw = data.pop().unwrap();
            match raw.variant {
                super::effect_builder::EffectVariant::Class => {
                    self.class_builder(raw.count, PossibleEffectStatement::Std(raw.expr));
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
                        self.style_builder(raw.count, PossibleEffectStatement::Std(raw.expr));
                    }
                },
                super::effect_builder::EffectVariant::Std => {
                    self.standard_prop_builder(
                        raw.name,
                        raw.count,
                        PossibleEffectStatement::Std(raw.expr),
                    );
                }
            }
        } else {
            for datum in data {
                self.effect_builder
                    .add_var_decl(datum.name.clone(), Some(datum.expr));
                match datum.variant {
                    super::effect_builder::EffectVariant::Class => {
                        self.class_builder(
                            datum.count,
                            PossibleEffectStatement::Effect((datum.v_val, datum.obj)),
                        );
                    }
                    super::effect_builder::EffectVariant::Style(key) => match key {
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
                            );
                        }
                    },
                    super::effect_builder::EffectVariant::Std => {
                        self.standard_prop_builder(
                            datum.name,
                            datum.count,
                            PossibleEffectStatement::Effect((datum.v_val, datum.obj)),
                        );
                    }
                }
            }
        }
        self.effect_builder
            .build_effect(std::mem::take(&mut self.statements))
    }

    fn build_individual_property_statement(
        &mut self,
        element_count: usize,
        name: Atom,
        attr: JSXAttrOrSpread,
    ) {
        match name.as_str() {
            "class" | "className" => {
                let expr = self.std_transform(attr, false);
                if self.tmp_wrap_effect {
                    let assigned_v = self.parent_visitor.v_count();
                    let assigned_prop = self.effect_builder.get_new_obj();
                    self.effect_builder.data.push(EffectMetadata {
                        expr,
                        v_val: generate_v(*assigned_v).into(),
                        obj: assigned_prop,
                        variant: super::effect_builder::EffectVariant::Class,
                        count: element_count,
                        name: "class".into(),
                    });

                    *assigned_v += 1;
                    return;
                };
                self.class_builder(element_count, PossibleEffectStatement::Std(expr));
            }
            "classList" => {
                todo!("Implement class list object")
            }
            "style" => {
                self.parse_all_styles(element_count, attr);
            }
            "ref" => {
                let expr = self.std_transform(attr, false);
                self.ref_builder(Some(element_count), expr);
            }
            prop => {
                let expr = self.std_transform(attr, false);
                // Event handling
                if prop.starts_with("on") {
                    self.event_handler_builder(prop, element_count, expr);
                } else if prop.starts_with("style:") {
                    let key: PropName = PropName::Str(
                        prop.split_once(":")
                            .expect("Wtf I just saw the ':'???")
                            .1
                            .into(),
                    );
                    if self.tmp_wrap_effect {
                        self.style_and_effect(element_count, expr, Some(key));
                        self.tmp_wrap_effect = false;
                    } else {
                        self.individual_style_builder(
                            element_count,
                            key,
                            PossibleEffectStatement::Std(expr),
                        );
                    }
                } else {
                    if self.tmp_wrap_effect {
                        let assigned_v = self.parent_visitor.v_count();
                        let assigned_prop = self.effect_builder.get_new_obj();
                        self.effect_builder.data.push(EffectMetadata {
                            expr,
                            v_val: generate_v(*assigned_v).into(),
                            obj: assigned_prop,
                            variant: super::effect_builder::EffectVariant::Std,
                            count: element_count,
                            name: prop.into(),
                        });

                        *assigned_v += 1;
                        return;
                    };
                    self.standard_prop_builder(
                        prop.into(),
                        element_count,
                        PossibleEffectStatement::Std(expr),
                    );
                }
            }
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
    ) -> String {
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
                        let is_call_expr_prop = if let JSXAttrOrSpread::JSXAttr(jsx_a) = &attr {
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

                        if seen_first_spread || is_call_expr_prop {
                            let prop: Box<Prop> = if is_call_expr_prop {
                                merge_prop_as_getter(name, self.std_transform(attr, false))
                            } else {
                                merge_prop_as_kv(name, self.std_transform(attr, false))
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
                            self.build_individual_property_statement(element_count, name, attr);
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
                merge_props_args.pop().unwrap()
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
                );
                self.tmp_wrap_effect = false;
            }
        }
        data.value
    }
}

use swc_core::{
    atoms::Atom,
    common::{Span, Spanned, SyntaxContext, DUMMY_SP},
    ecma::{
        ast::{
            CallExpr, Callee, Expr, ExprOrSpread, ExprStmt, IdentName, JSXAttr, JSXAttrOrSpread,
            JSXAttrValue, JSXExpr, Lit, MemberExpr, MemberProp, ObjectLit, PropOrSpread, Stmt,
        },
        visit::VisitMutWith,
    },
};

use crate::{
    builder::client::{
        effect_builder::EffectVariant,
        element_properties::{
            helpers::key_to_atom, ClassBuilderVariants, EffectOrInlineOrExpression,
            ElementPropertiesBuilder, PossibleEffectStatement,
        },
        jsx_expr_transformer_client::ClientJsxExprTransformer,
    },
    helpers::{
        common_into_expressions::{create_double_negated, ident_callee, ident_expr},
        component_helpers::is_falsy_lit,
        generate_var_names::{generate_el, CLASS_LIST},
    },
    transform::parent_visitor::ParentVisitor,
};

fn create_class_list(element_count: usize, expr: Box<Expr>, extra_arg: Option<Atom>) -> Box<Expr> {
    let mut args: Vec<ExprOrSpread> =
        vec![ident_expr(generate_el(element_count)).into(), expr.into()];
    if let Some(extra_arg) = extra_arg {
        args.push(ident_expr(extra_arg).into());
    }
    CallExpr {
        span: DUMMY_SP,
        ctxt: SyntaxContext::empty(),
        callee: ident_callee(CLASS_LIST.into()),
        args,
        type_args: None,
    }
    .into()
}

fn create_class_list_toggle(
    element_count: usize,
    key: &str,
    expr: Box<Expr>,
    double_negate: bool,
) -> Box<Expr> {
    CallExpr {
        span: DUMMY_SP,
        ctxt: SyntaxContext::empty(),
        callee: Callee::Expr(
            MemberExpr {
                span: DUMMY_SP,
                obj: MemberExpr {
                    span: DUMMY_SP,
                    obj: ident_expr(generate_el(element_count)),
                    prop: MemberProp::Ident(IdentName {
                        span: DUMMY_SP,
                        sym: "classList".into(),
                    }),
                }
                .into(),
                prop: MemberProp::Ident(IdentName {
                    span: DUMMY_SP,
                    sym: "toggle".into(),
                }),
            }
            .into(),
        ),
        args: vec![
            Expr::from(Atom::from(key)).into(),
            if double_negate {
                create_double_negated(expr).into()
            } else {
                expr.into()
            },
        ],
        type_args: None,
    }
    .into()
}

fn needs_class_list(props: &Vec<PropOrSpread>) -> bool {
    props.iter().any(|prop| {
        if let Some(prop) = prop.as_prop() {
            if let Some(prop) = prop.as_key_value() {
                prop.key.is_computed()
                    || if let Some(prop) = prop.key.as_str() {
                        prop.value.as_str().contains(|c| c == ':' || c == ' ')
                    } else {
                        false
                    }
            } else {
                true
            }
        } else {
            // Spread
            true
        }
    })
}

fn attr_is_obj(attr: &JSXAttrOrSpread) -> (bool, Option<Span>) {
    match attr {
        JSXAttrOrSpread::JSXAttr(a) => {
            if let Some(JSXAttrValue::JSXExprContainer(ec)) = &a.value {
                if let JSXExpr::Expr(e) = &ec.expr {
                    if e.is_object() {
                        return (true, Some(ec.span));
                    }
                }
            }
        }
        _ => {}
    }
    (false, None)
}
fn attr_get_obj(attr: JSXAttrOrSpread) -> Option<ObjectLit> {
    match attr {
        JSXAttrOrSpread::JSXAttr(a) => {
            if let Some(JSXAttrValue::JSXExprContainer(ec)) = a.value {
                if let JSXExpr::Expr(e) = ec.expr {
                    return Some(e.object()?);
                }
            }
        }
        _ => {}
    }
    None
}

impl<'a, T: ParentVisitor> ElementPropertiesBuilder<'a, T> {
    pub fn class_list_builder(
        &mut self,
        element_count: usize,
        key: Option<&str>,
        data: PossibleEffectStatement,
        is_in_effect: bool,
    ) {
        let (data, effect_vars) = match self.effect_or_inline_or_expr(data) {
            EffectOrInlineOrExpression::EffectRes((data, effect_vars)) => (data, Some(effect_vars)),
            EffectOrInlineOrExpression::ExpressionRes(data) => (data, None),
            EffectOrInlineOrExpression::InlineRes(ir) => {
                if let Some(key) = key {
                    // Falsy
                    if key.is_empty() {
                        return;
                    }
                    self.class_builder_data
                        .1
                        .push(ClassBuilderVariants::Lit(key.into()));
                } else {
                    self.direct_template_inserts.push(("classlist".into(), ir));
                }
                return;
            }
        };
        let expr: Box<Expr> = if let Some(key) = key {
            create_class_list_toggle(element_count, key, data, effect_vars.is_none())
        } else {
            self.parent_visitor.add_import(CLASS_LIST.into());
            let effect_arg = if is_in_effect {
                Some(self.effect_builder.get_effect_arg())
            } else {
                None
            };
            create_class_list(element_count, data, effect_arg)
        };
        if let Some(effect_vars) = effect_vars {
            self.statements
                .push(super::helpers::generate_effect_statement(effect_vars, expr));
        } else {
            self.statements.push(Stmt::Expr(ExprStmt {
                span: DUMMY_SP,
                expr,
            }));
        }
    }

    fn class_list_parser(
        &mut self,
        element_count: usize,
        mut obj_expr: ObjectLit,
        should_effect: bool,
    ) {
        if needs_class_list(&obj_expr.props) {
            let mut little_visitor =
                ClientJsxExprTransformer::new(self.parent_visitor, false, false);
            obj_expr.visit_mut_with(&mut little_visitor);
            if should_effect && little_visitor.should_wrap_in_effect {
                let v_cnt = *self.parent_visitor.v_count();
                self.effect_builder.add_data(
                    Box::new(Expr::Object(obj_expr)),
                    EffectVariant::ClassList(false),
                    v_cnt,
                    element_count,
                    "classList",
                );
                *self.parent_visitor.v_count() += 1;
            } else {
                self.class_list_builder(
                    element_count,
                    None,
                    PossibleEffectStatement::Std(Box::new(Expr::Object(obj_expr))),
                    false,
                );
            }
        } else {
            for prop in obj_expr.props {
                let prop = prop
                    .prop()
                    .expect("prop should not be spread")
                    .key_value()
                    .expect("prop should be kv");
                let (key, mut val) = (
                    key_to_atom(prop.key).unwrap_or("classList".into()),
                    prop.value,
                );
                let mut little_visitor =
                    ClientJsxExprTransformer::new(self.parent_visitor, false, false);
                val.visit_mut_with(&mut little_visitor);
                if should_effect && little_visitor.should_wrap_in_effect {
                    let v_cnt = *self.parent_visitor.v_count();
                    self.effect_builder.add_data(
                        val,
                        EffectVariant::ClassList(true),
                        v_cnt,
                        element_count,
                        key.as_str(),
                    );
                    *self.parent_visitor.v_count() += 1;
                } else {
                    if let Some(l) = val.as_lit() {
                        if !is_falsy_lit(l) {
                            // Hacky way to inline it
                            self.class_list_builder(
                                element_count,
                                Some(key.as_str()),
                                PossibleEffectStatement::Std(Expr::Lit(Lit::Str("".into())).into()),
                                false,
                            )
                        }
                    } else {
                        self.class_list_builder(
                            element_count,
                            Some(key.as_str()),
                            PossibleEffectStatement::Std(val),
                            false,
                        )
                    }
                }
            }
        }
    }

    pub fn class_list_delegator(&mut self, element_count: usize, attr: JSXAttrOrSpread) {
        let (attr_is_obj, span_data) = attr_is_obj(&attr);

        let should_effect = if let Some(span_data) = span_data {
            !self.parent_visitor.has_static_marker(span_data.span_lo())
        } else {
            true
        };
        if attr_is_obj {
            // Should always works
            if let Some(obj) = attr_get_obj(attr) {
                self.class_list_parser(element_count, obj, should_effect);
            }
        } else {
            let transformed = self.std_transform(attr, false, None);
            if self.tmp_wrap_effect || (should_effect && transformed.is_ident()) {
                let v_cnt = *self.parent_visitor.v_count();
                self.effect_builder.add_data(
                    transformed,
                    EffectVariant::ClassList(false),
                    v_cnt,
                    element_count,
                    "classList",
                );
                *self.parent_visitor.v_count() += 1;
                self.tmp_wrap_effect = false;
            } else {
                self.class_list_builder(
                    element_count,
                    None,
                    PossibleEffectStatement::Std(transformed),
                    false,
                );
            }
        }
    }
}

use std::os::unix::raw;

use swc_core::{
    atoms::Atom,
    common::{Spanned, SyntaxContext, DUMMY_SP},
    ecma::{
        ast::{
            CallExpr, Expr, ExprOrSpread, ExprStmt, JSXAttrOrSpread, JSXAttrValue, JSXExpr,
            KeyValueProp, Lit, ObjectLit, Prop, PropName, PropOrSpread, Stmt,
        },
        utils::number::ToJsString,
        visit::{VisitMutWith, VisitWith},
    },
};

use crate::{
    builder::client::{
        effect_builder::{EffectMetadata, EffectVariant},
        element_properties::{
            helpers::key_to_atom, EffectOrInlineOrExpression, ElementPropertiesBuilder,
            PossibleEffectStatement,
        },
        jsx_expr_transformer_client::ClientJsxExprTransformer,
    },
    helpers::{
        common_into_expressions::{ident_callee, ident_expr},
        component_helpers::is_undefined,
        generate_var_names::{generate_el, SET_STYLE_PROPERTY, STYLE},
    },
    transform::parent_visitor::ParentVisitor,
};

fn build_style(args: Vec<ExprOrSpread>) -> Box<Expr> {
    CallExpr {
        span: DUMMY_SP,
        ctxt: SyntaxContext::empty(),
        callee: ident_callee(STYLE.into()),
        args,
        type_args: None,
    }
    .into()
}

fn build_set_style_call(el: usize, key: Atom, val: Box<Expr>) -> Box<Expr> {
    CallExpr {
        span: DUMMY_SP,
        ctxt: SyntaxContext::empty(),
        callee: ident_callee(SET_STYLE_PROPERTY.into()),
        args: vec![
            ident_expr(generate_el(el)).into(),
            Expr::from(key).into(),
            val.into(),
        ],
        type_args: None,
    }
    .into()
}

fn build_style_call(el: usize, arg: Box<Expr>) -> Box<Expr> {
    CallExpr {
        span: DUMMY_SP,
        ctxt: SyntaxContext::empty(),
        callee: ident_callee(STYLE.into()),
        args: vec![ident_expr(generate_el(el)).into(), arg.into()],
        type_args: None,
    }
    .into()
}

impl<'a, T: ParentVisitor> ElementPropertiesBuilder<'a, T> {
    pub fn individual_style_builder(
        &mut self,
        element_count: usize,
        key: PropName,
        value: PossibleEffectStatement,
    ) {
        let inline = key.is_str();
        let Some(key) = key_to_atom(key) else {
            return;
        };
        let (data, effect_vars) = match self.effect_or_inline_or_expr(value) {
            EffectOrInlineOrExpression::EffectRes((data, effect_vars)) => (data, Some(effect_vars)),
            EffectOrInlineOrExpression::ExpressionRes(data) => (data, None),
            EffectOrInlineOrExpression::InlineRes(ir) => {
                if inline {
                    self.direct_template_inserts
                        .push(("style".into(), format!("{0}:{1}", key.as_str(), ir)));
                    return;
                }
                (Expr::Lit(Lit::Str(ir.into())).into(), None)
            }
        };
        let res = build_set_style_call(element_count, key, data);
        self.parent_visitor.add_import(SET_STYLE_PROPERTY.into());
        if let Some(effect_vars) = effect_vars {
            self.statements
                .push(super::helpers::generate_effect_statement(effect_vars, res));
        } else {
            self.statements.push(Stmt::Expr(ExprStmt {
                span: DUMMY_SP,
                expr: res,
            }));
        }
    }

    pub fn style_builder(&mut self, element_count: usize, data: PossibleEffectStatement) {
        let (data, effect_vars) = match self.effect_or_inline_or_expr(data) {
            EffectOrInlineOrExpression::EffectRes((data, effect_vars)) => (data, Some(effect_vars)),
            EffectOrInlineOrExpression::ExpressionRes(data) => (data, None),
            EffectOrInlineOrExpression::InlineRes(ir) => {
                // Shouldn't be possible??
                self.direct_template_inserts.push(("style".into(), ir));
                return;
            }
        };
        let res = build_style_call(element_count, data);
        self.parent_visitor.add_import(STYLE.into());
        if let Some(effect_vars) = effect_vars {
            self.statements
                .push(super::helpers::generate_effect_statement(effect_vars, res));
        } else {
            self.statements.push(Stmt::Expr(ExprStmt {
                span: DUMMY_SP,
                expr: res,
            }));
        }
    }

    // TODO: Effect can be 'turned off' so handle that here later
    pub fn style_and_effect(
        &mut self,
        element_count: usize,
        data: Box<Expr>,
        key: Option<PropName>,
    ) {
        let v_cnt = *self.parent_visitor.v_count();
        self.effect_builder.add_data(
            data,
            EffectVariant::Style(key),
            v_cnt,
            element_count,
            "style",
        );
        *self.parent_visitor.v_count() += 1;
    }

    pub fn parse_all_styles(&mut self, element_count: usize, mut raw_data: JSXAttrOrSpread) {
        match &mut raw_data {
            JSXAttrOrSpread::JSXAttr(jsxattr) => {
                match &mut jsxattr.value {
                    Some(JSXAttrValue::JSXExprContainer(ec)) => match &mut ec.expr {
                        JSXExpr::Expr(e) => {
                            if let Some(obj_expr) = e.to_owned().object() {
                                if obj_expr
                                    .props
                                    .iter()
                                    .any(|i| matches!(i, PropOrSpread::Spread(_)))
                                {
                                    if !self.parent_visitor.has_static_marker(ec.span_lo()) {
                                        // Effect wrap whole thing if has spread
                                        self.style_and_effect(
                                            element_count,
                                            Expr::Object(obj_expr).into(),
                                            None,
                                        );
                                    } else {
                                        self.style_builder(
                                            element_count,
                                            PossibleEffectStatement::Std(
                                                Expr::Object(obj_expr).into(),
                                            ),
                                        );
                                    }
                                    return;
                                } else {
                                    let mut style_obj_builder = ObjectLit::default();
                                    for mut prop in obj_expr.props {
                                        // Should not run if there are any spreads
                                        match &mut **prop.as_mut_prop().expect("Spread prop in style element should have already been found") {
                                            Prop::Shorthand(ident) => {
                                                // Treat { key } as { key: 'key' }
                                                self.individual_style_builder(
                                                    element_count,
                                                    PropName::Str(ident.sym.as_str().into()),
                                                    PossibleEffectStatement::Std(prop.expect_prop().expect_shorthand().into()),
                                                );
                                            }
                                            Prop::KeyValue(key_value_prop) => {
                                                if key_value_prop.value.is_null() || is_undefined(&key_value_prop.value){
                                                    // Null and undefined values are removed
                                                    continue;
                                                }
                                                if key_value_prop.key.is_computed() {
                                                    // Effect it -> in glob
                                                    style_obj_builder.props.push(prop);
                                                } else {
                                                    let mut visitor = ClientJsxExprTransformer::new(self.parent_visitor, false, false);
                                                    key_value_prop.value.visit_mut_with(&mut visitor);
                                                    // Should always succeed
                                                    let kv = prop.expect_prop().expect_key_value();
                                                    if visitor.should_wrap_in_effect {
                                                        self.style_and_effect(element_count, kv.value, Some(kv.key));
                                                    } else {
                                                        self.individual_style_builder(
                                                            element_count,
                                                            kv.key,
                                                            PossibleEffectStatement::Std(kv.value),
                                                        );
                                                    }
                                                }
                                            }
                                            _ => {
                                                // Style + effect bin
                                                style_obj_builder.props.push(prop);
                                            }
                                        };
                                    }
                                    /* handle obj */
                                    if !style_obj_builder.props.is_empty() {
                                        let mut little_visitor = ClientJsxExprTransformer::new(
                                            self.parent_visitor,
                                            false,
                                            false,
                                        );
                                        for p in style_obj_builder.props.iter_mut() {
                                            p.visit_mut_with(&mut little_visitor);
                                        }
                                        if little_visitor.should_wrap_in_effect {
                                            self.style_and_effect(
                                                element_count,
                                                style_obj_builder.into(),
                                                None,
                                            );
                                        } else {
                                            self.style_builder(
                                                element_count,
                                                PossibleEffectStatement::Std(
                                                    style_obj_builder.into(),
                                                ),
                                            );
                                        }
                                    }
                                }
                                // Dont do logic below
                                return;
                            }
                            /* std trans */
                        }
                        _ => { /* std trans */ }
                    },
                    _ => { /* std trans */ }
                }
            }
            /* Spread */
            _ => { /* std trans */ }
        }
        let transformed = self.std_transform(raw_data, false);
        if self.tmp_wrap_effect {
            self.style_and_effect(element_count, transformed, None);
        } else {
            self.style_builder(element_count, PossibleEffectStatement::Std(transformed));
        }
        self.tmp_wrap_effect = false;
    }
}

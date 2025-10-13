use swc_core::{
    atoms::Atom,
    common::{Spanned, SyntaxContext, DUMMY_SP},
    ecma::{
        ast::{
            AssignExpr, AssignTarget, CallExpr, Expr, ExprOrSpread, ExprStmt, JSXAttrOrSpread,
            JSXAttrValue, JSXExpr, Lit, ObjectLit, Prop, PropName, PropOrSpread,
            SimpleAssignTarget, Stmt,
        },
        visit::VisitMutWith,
    },
};

use crate::{
    builder::client::{
        effect_builder::EffectVariant,
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

fn build_style_call(el: usize, arg: Box<Expr>, extra_arg: Option<Box<Expr>>) -> Box<Expr> {
    let mut args: Vec<ExprOrSpread> = vec![ident_expr(generate_el(el)).into(), arg.into()];
    if let Some(extra_arg) = extra_arg {
        args.push(extra_arg.into());
    }
    CallExpr {
        span: DUMMY_SP,
        ctxt: SyntaxContext::empty(),
        callee: ident_callee(STYLE.into()),
        args,
        type_args: None,
    }
    .into()
}

fn split_out_object_lits(obj: ObjectLit) -> (Vec<PropOrSpread>, Vec<String>) {
    let mut res_props: Vec<PropOrSpread> = vec![];
    let mut res_str: Vec<String> = vec![];
    for prop in obj.props {
        if let Some(p) = prop.as_prop() {
            if let Some(kv) = p.as_key_value() {
                if kv.key.is_str() {
                    if let Some(lit_v) = kv.value.as_lit() {
                        if lit_v.is_str() {
                            let prop = prop.expect_prop().expect_key_value();
                            let key = prop.key.expect_str().value;
                            let value = prop.value.expect_lit().expect_str().value;
                            res_str.push(format!("{0}:{1}", key.as_str(), value.as_str()));
                            continue;
                        }
                    }
                }
            }
        }
        res_props.push(prop);
    }
    (res_props, res_str)
}

impl<'a, T: ParentVisitor> ElementPropertiesBuilder<'a, T> {
    fn add_style_inline(&mut self, style: String) {
        let pos = self
            .direct_template_inserts
            .iter()
            .position(|(key, _)| key == "style");
        if let Some(pos) = pos {
            self.direct_template_inserts[pos].1 += ";";
            self.direct_template_inserts[pos].1 += style.as_str();
        } else {
            self.direct_template_inserts
                .push(("style".to_string(), style));
        }
    }

    fn remove_obj_inline_styles(&mut self, obj: ObjectLit) -> Box<Expr> {
        let (props, inlines) = split_out_object_lits(obj);
        if !inlines.is_empty() {
            self.add_style_inline(inlines.join(";"));
        }
        ObjectLit {
            span: DUMMY_SP,
            props,
        }
        .into()
    }

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
                    self.add_style_inline(format!("{0}:{1}", key.as_str(), ir));
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

    pub fn style_builder(
        &mut self,
        element_count: usize,
        data: PossibleEffectStatement,
        extra_arg: Option<Box<Expr>>,
    ) {
        let (data, effect_vars) = match self.effect_or_inline_or_expr(data) {
            EffectOrInlineOrExpression::EffectRes((data, effect_vars)) => (data, Some(effect_vars)),
            EffectOrInlineOrExpression::ExpressionRes(data) => (data, None),
            EffectOrInlineOrExpression::InlineRes(ir) => {
                // Shouldn't be possible??
                self.add_style_inline(ir);
                return;
            }
        };
        self.parent_visitor.add_import(STYLE.into());
        if let Some(effect_vars) = effect_vars {
            let style_stmt = AssignExpr {
                span: DUMMY_SP,
                op: swc_core::ecma::ast::AssignOp::Assign,
                left: AssignTarget::Simple(SimpleAssignTarget::Member(effect_vars.1.clone())),
                right: build_style_call(
                    element_count,
                    effect_vars.0.into(),
                    Some(effect_vars.1.into()),
                ),
            };
            self.statements.push(Stmt::Expr(ExprStmt {
                span: DUMMY_SP,
                expr: style_stmt.into(),
            }));
        } else {
            self.statements.push(Stmt::Expr(ExprStmt {
                span: DUMMY_SP,
                expr: build_style_call(element_count, data, extra_arg),
            }));
        }
    }

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
                    Some(JSXAttrValue::JSXExprContainer(ec)) => {
                        let has_static_marker = self.parent_visitor.has_static_marker(ec.span_lo());
                        match &mut ec.expr {
                            JSXExpr::Expr(e) => {
                                if let Some(obj_expr) = e.to_owned().object() {
                                    if obj_expr
                                        .props
                                        .iter()
                                        .any(|i| matches!(i, PropOrSpread::Spread(_)))
                                    {
                                        let expr = self.remove_obj_inline_styles(obj_expr);
                                        if !has_static_marker {
                                            // Effect wrap whole thing if has spread
                                            self.style_and_effect(element_count, expr, None);
                                        } else {
                                            self.style_builder(
                                                element_count,
                                                PossibleEffectStatement::Std(expr),
                                                None,
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
                                                    if !has_static_marker && visitor.should_wrap_in_effect {
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
                                            if !has_static_marker
                                                && little_visitor.should_wrap_in_effect
                                            {
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
                                                    None,
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
                        }
                    }
                    _ => { /* std trans */ }
                }
            }
            /* Spread */
            _ => { /* std trans */ }
        }
        let transformed = self.std_transform(raw_data, false, None);
        if self.tmp_wrap_effect {
            self.style_and_effect(element_count, transformed, None);
        } else {
            self.style_builder(
                element_count,
                PossibleEffectStatement::Std(transformed),
                None,
            );
        }
        self.tmp_wrap_effect = false;
    }
}

use swc_core::{
    atoms::Atom,
    common::{SyntaxContext, DUMMY_SP},
    ecma::ast::{CallExpr, ExprStmt, Stmt},
};

use crate::{
    builder::client::element_properties::{
        EffectOrInlineOrExpression, ElementPropertiesBuilder, PossibleEffectStatement,
    },
    helpers::{
        common_into_expressions::{ident_callee, ident_expr},
        generate_var_names::{generate_el, SET_ATTRIBUTE, SET_BOOLEAN_ATTRIBUTE},
    },
    transform::parent_visitor::ParentVisitor,
};

impl<'a, T: ParentVisitor> ElementPropertiesBuilder<'a, T> {
    pub fn standard_prop_builder(
        &mut self,
        prop_name: Atom,
        element_count: usize,
        is_bool_attr: bool,
        data: PossibleEffectStatement,
    ) {
        let (data, effect_vars) = match self.effect_or_inline_or_expr(data) {
            EffectOrInlineOrExpression::EffectRes((data, effect_vars)) => (data, Some(effect_vars)),
            EffectOrInlineOrExpression::ExpressionRes(data) => (data, None),
            EffectOrInlineOrExpression::InlineRes(ir) => {
                self.direct_template_inserts
                    .push((prop_name.as_str().into(), ir));
                return;
            }
        };
        let callee = if is_bool_attr {
            SET_BOOLEAN_ATTRIBUTE
        } else {
            SET_ATTRIBUTE
        };
        self.parent_visitor.add_import(callee.into());
        let set_attribute = CallExpr {
            span: DUMMY_SP,
            ctxt: SyntaxContext::empty(),
            callee: ident_callee(callee.into()),
            type_args: None,
            args: vec![
                ident_expr(generate_el(element_count)).into(),
                ident_expr(prop_name).into(),
                data.into(),
            ],
        };
        if let Some(effect_vars) = effect_vars {
            self.statements
                .push(super::helpers::generate_effect_statement(
                    effect_vars,
                    set_attribute.into(),
                ));
        } else {
            self.statements.push(Stmt::Expr(ExprStmt {
                span: DUMMY_SP,
                expr: set_attribute.into(),
            }));
        }
    }
}

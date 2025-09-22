use swc_core::{
    atoms::Atom,
    common::{SyntaxContext, DUMMY_SP},
    ecma::ast::{CallExpr, Expr, ExprStmt, Lit, Stmt},
};

use crate::{
    builder::client::element_properties::{
        generate_effect_statement, EffectOrInlineOrExpression, ElementPropertiesBuilder,
        PossibleEffectStatement,
    },
    helpers::{
        common_into_expressions::{ident_callee, ident_expr},
        generate_var_names::{generate_el, generate_set_attribute},
    },
    transform::parent_visitor::ParentVisitor,
};

impl<'a, T: ParentVisitor> ElementPropertiesBuilder<'a, T> {
    pub fn standard_prop_builder(
        &mut self,
        prop_name: Atom,
        element_count: usize,
        data: PossibleEffectStatement,
    ) {
        self.used_events
            .insert(generate_set_attribute().as_str().into());
        let (data, effect_vars) = match self.effect_or_inline_or_expr(data) {
            EffectOrInlineOrExpression::EffectRes((data, effect_vars)) => (data, Some(effect_vars)),
            EffectOrInlineOrExpression::ExpressionRes(data) => (data, None),
            EffectOrInlineOrExpression::InlineRes(ir) => {
                self.direct_template_inserts
                    .insert(prop_name.as_str().into(), ir);
                return;
            }
        };
        let set_attribute = CallExpr {
            span: DUMMY_SP,
            ctxt: SyntaxContext::empty(),
            callee: ident_callee(generate_set_attribute()),
            type_args: None,
            args: vec![
                ident_expr(generate_el(element_count)).into(),
                ident_expr(prop_name).into(),
                data.into(),
            ],
        };
        if let Some(effect_vars) = effect_vars {
            self.statements
                .push(generate_effect_statement(effect_vars, set_attribute.into()));
        } else {
            self.statements.push(Stmt::Expr(ExprStmt {
                span: DUMMY_SP,
                expr: set_attribute.into(),
            }));
        }
    }
}

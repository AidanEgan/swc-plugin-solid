use swc_core::{
    atoms::Atom,
    common::DUMMY_SP,
    ecma::ast::{AssignExpr, Expr, ExprStmt, ParenExpr, Stmt},
};

use crate::{
    builder::client::element_properties::{
        helpers::generate_effect_assignment, ElementPropertiesBuilder, PossibleEffectStatement,
    },
    helpers::{common_into_expressions::simple_member_expression, generate_var_names::generate_el},
    transform::parent_visitor::ParentVisitor,
};

impl<'a, T: ParentVisitor> ElementPropertiesBuilder<'a, T> {
    pub fn require_prop_builder(
        &mut self,
        prop_name: Atom,
        element_count: usize,
        data: PossibleEffectStatement,
    ) {
        let (data, effect_vars) = match data {
            PossibleEffectStatement::Std(std) => (std, None),
            PossibleEffectStatement::Effect(effect_data) => {
                let pe = ParenExpr {
                    span: DUMMY_SP,
                    expr: generate_effect_assignment(&effect_data),
                };
                (pe.into(), Some(effect_data))
            }
        };

        let is_text_content = prop_name.as_str() == "textContent";
        let prop_name = if is_text_content && effect_vars.is_some() {
            "data".into()
        } else {
            prop_name
        };

        let expr: Box<Expr> = AssignExpr {
            span: DUMMY_SP,
            op: swc_core::ecma::ast::AssignOp::Assign,
            left: simple_member_expression(generate_el(element_count), prop_name).into(),
            right: data,
        }
        .into();
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
}

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
        is_bool_attr: bool,
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

        if is_bool_attr && data.is_lit() {
            if data.as_lit().unwrap().is_num() || data.as_lit().unwrap().is_str() {
                let data = data.expect_lit();
                match data {
                    swc_core::ecma::ast::Lit::Str(s) => self
                        .direct_template_inserts
                        .push((prop_name.to_string(), s.value.to_string())),
                    swc_core::ecma::ast::Lit::Num(n) => self
                        .direct_template_inserts
                        .push((prop_name.to_string(), n.value.to_string())),
                    swc_core::ecma::ast::Lit::Bool(b) => {
                        if b.value == true {
                            self.direct_template_inserts
                                .push((prop_name.to_string(), "".to_string()))
                        }
                    }
                    _ => { /* Not actually reachable :) */ }
                }
                return;
            }
        }

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

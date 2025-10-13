use swc_core::{
    common::{SyntaxContext, DUMMY_SP},
    ecma::ast::{AssignExpr, CallExpr, Expr, ExprStmt, Lit, ParenExpr, Stmt},
};

use crate::{
    builder::client::element_properties::{
        ClassBuilderVariants, EffectOrInlineOrExpression, ElementPropertiesBuilder,
        PossibleEffectStatement,
    },
    helpers::{
        common_into_expressions::{ident_callee, ident_expr, simple_member_expression},
        generate_var_names::{generate_class_name, generate_el},
    },
    transform::parent_visitor::ParentVisitor,
};

const CLASS: &str = "class";

impl<'a, T: ParentVisitor> ElementPropertiesBuilder<'a, T> {
    pub fn class_builder(&mut self, element_count: usize, data: PossibleEffectStatement) {
        let (data, effect_vars) = match self.effect_or_inline_or_expr(data) {
            EffectOrInlineOrExpression::EffectRes((data, effect_vars)) => (data, Some(effect_vars)),
            EffectOrInlineOrExpression::ExpressionRes(data) => (data, None),
            EffectOrInlineOrExpression::InlineRes(ir) => {
                self.direct_template_inserts.push((CLASS.into(), ir));
                return;
            }
        };
        let class_name = generate_class_name();
        self.parent_visitor.add_import(class_name.as_str().into());
        let class_name = CallExpr {
            span: DUMMY_SP,
            ctxt: SyntaxContext::empty(),
            callee: ident_callee(generate_class_name()),
            type_args: None,
            args: vec![ident_expr(generate_el(element_count)).into(), data.into()],
        };
        if let Some(effect_vars) = effect_vars {
            self.statements
                .push(super::helpers::generate_effect_statement(
                    effect_vars,
                    class_name.into(),
                ));
        } else {
            self.statements.push(Stmt::Expr(ExprStmt {
                span: DUMMY_SP,
                expr: class_name.into(),
            }));
        }
    }

    pub fn class_name_builder(&mut self, element_count: usize, data: PossibleEffectStatement) {
        let (data, effect_vars) = match self.effect_or_inline_or_expr(data) {
            EffectOrInlineOrExpression::EffectRes((data, effect_vars)) => {
                let pe = ParenExpr {
                    span: DUMMY_SP,
                    expr: data,
                };
                (pe.into(), Some(effect_vars))
            }
            EffectOrInlineOrExpression::ExpressionRes(data) => (data, None),
            EffectOrInlineOrExpression::InlineRes(ir) => {
                self.direct_template_inserts.push((CLASS.into(), ir));
                return;
            }
        };
        let expr: Box<Expr> = AssignExpr {
            span: DUMMY_SP,
            op: swc_core::ecma::ast::AssignOp::Assign,
            left: simple_member_expression(generate_el(element_count), "className".into()).into(),
            right: data,
        }
        .into();
        if let Some(effect_vars) = effect_vars {
            self.statements
                .push(super::helpers::generate_effect_statement(
                    effect_vars,
                    expr.into(),
                ));
        } else {
            self.statements.push(Stmt::Expr(ExprStmt {
                span: DUMMY_SP,
                expr: expr.into(),
            }));
        }
    }

    pub fn class_delegator(&mut self, transformed_data: Box<Expr>) {
        if self.tmp_wrap_effect {
            self.class_builder_data
                .1
                .push(ClassBuilderVariants::ComputedEffect(transformed_data));
            self.tmp_wrap_effect = false;
        } else if let Some(l) = transformed_data.as_lit() {
            match l {
                Lit::Str(s) => self
                    .class_builder_data
                    .1
                    .push(ClassBuilderVariants::Lit(s.value.as_str().into())),
                Lit::Num(n) => {
                    self.class_builder_data
                        .1
                        .push(ClassBuilderVariants::Lit(n.value.to_string()));
                }
                _ => self
                    .class_builder_data
                    .1
                    .push(ClassBuilderVariants::Computed(transformed_data)),
            }
        } else {
            self.class_builder_data
                .1
                .push(ClassBuilderVariants::Computed(transformed_data));
        }
    }
}

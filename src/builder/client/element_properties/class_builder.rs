use swc_core::{
    common::{SyntaxContext, DUMMY_SP},
    ecma::ast::{CallExpr, ExprStmt, Stmt},
};

use crate::{
    builder::client::element_properties::{
        generate_effect_statement, EffectOrInlineOrExpression, ElementPropertiesBuilder,
        PossibleEffectStatement,
    },
    helpers::{
        common_into_expressions::{ident_callee, ident_expr},
        generate_var_names::{generate_class_name, generate_el},
    },
    transform::parent_visitor::ParentVisitor,
};

const CLASS: &str = "class";

impl<'a, T: ParentVisitor> ElementPropertiesBuilder<'a, T> {
    pub fn class_builder(&mut self, element_count: usize, data: PossibleEffectStatement) {
        let class_name = generate_class_name();
        self.parent_visitor.add_import(class_name.as_str().into());
        let (data, effect_vars) = match self.effect_or_inline_or_expr(data) {
            EffectOrInlineOrExpression::EffectRes((data, effect_vars)) => (data, Some(effect_vars)),
            EffectOrInlineOrExpression::ExpressionRes(data) => (data, None),
            EffectOrInlineOrExpression::InlineRes(ir) => {
                self.direct_template_inserts.push((CLASS.into(), ir));
                return;
            }
        };
        let class_name = CallExpr {
            span: DUMMY_SP,
            ctxt: SyntaxContext::empty(),
            callee: ident_callee(generate_class_name()),
            type_args: None,
            args: vec![ident_expr(generate_el(element_count)).into(), data.into()],
        };
        if let Some(effect_vars) = effect_vars {
            self.statements
                .push(generate_effect_statement(effect_vars, class_name.into()));
        } else {
            self.statements.push(Stmt::Expr(ExprStmt {
                span: DUMMY_SP,
                expr: class_name.into(),
            }));
        }
    }
}

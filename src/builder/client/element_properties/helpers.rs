use swc_core::{
    atoms::Atom,
    common::{SyntaxContext, DUMMY_SP},
    ecma::{
        ast::{
            AssignExpr, AssignTarget, BinExpr, BlockStmt, CallExpr, Expr, ExprOrSpread, ExprStmt,
            GetterProp, Ident, IdentName, KeyValueProp, MemberExpr, ParenExpr, Prop, PropName,
            ReturnStmt, SimpleAssignTarget, Stmt,
        },
        utils::number::ToJsString,
    },
};

use crate::helpers::{
    common_into_expressions::{ident_callee, ident_name_safe},
    generate_var_names::USE,
};

pub fn key_to_atom(key: PropName) -> Option<Atom> {
    match key {
        PropName::Ident(ident_name) => Some(ident_name.sym),
        PropName::Str(s) => Some(s.value),
        PropName::Num(n) => Some(n.value.to_js_string().into()),
        // Uncommon I believe
        PropName::BigInt(big_int) => big_int.raw,
        // Should not be possible
        PropName::Computed(_) => None,
    }
}

pub type Effect = (Ident, MemberExpr);
pub enum PossibleEffectStatement {
    Std(Box<Expr>),
    Effect(Effect),
}

pub enum EffectOrInlineOrExpression {
    EffectRes((Box<Expr>, Effect)),
    InlineRes(String),
    ExpressionRes(Box<Expr>),
}

pub fn generate_effect_assignment(data: &Effect) -> Box<Expr> {
    let assign = AssignExpr {
        span: DUMMY_SP,
        op: swc_core::ecma::ast::AssignOp::Assign,
        left: AssignTarget::Simple(SimpleAssignTarget::Member(data.1.clone())),
        right: data.0.clone().into(),
    };
    // Swc may remove parens if not needed
    Expr::Paren(ParenExpr {
        span: DUMMY_SP,
        expr: assign.into(),
    })
    .into()
}

pub fn generate_effect_statement(data: Effect, expr: Box<Expr>) -> Stmt {
    let expr = BinExpr {
        span: DUMMY_SP,
        op: swc_core::ecma::ast::BinaryOp::LogicalAnd,
        left: BinExpr {
            span: DUMMY_SP,
            op: swc_core::ecma::ast::BinaryOp::NotEqEq,
            left: data.0.into(),
            right: data.1.into(),
        }
        .into(),
        right: expr,
    };
    Stmt::Expr(ExprStmt {
        span: DUMMY_SP,
        expr: expr.into(),
    })
}

pub fn generate_use_expr(args: Vec<ExprOrSpread>) -> Box<Expr> {
    Box::new(Expr::Call(CallExpr {
        span: DUMMY_SP,
        ctxt: SyntaxContext::empty(),
        callee: ident_callee(USE.into()),
        args,
        type_args: None,
    }))
}

pub fn merge_prop_as_getter(key: Atom, value: Box<Expr>) -> Box<Prop> {
    Prop::Getter(GetterProp {
        span: DUMMY_SP,
        key: PropName::Ident(IdentName {
            span: DUMMY_SP,
            sym: key,
        }),
        type_ann: None,
        body: Some(BlockStmt {
            span: DUMMY_SP,
            ctxt: SyntaxContext::empty(),
            stmts: vec![Stmt::Return(ReturnStmt {
                span: DUMMY_SP,
                arg: Some(value),
            })],
        }),
    })
    .into()
}

fn merge_prop_ident_if_possible(key: Atom) -> PropName {
    match ident_name_safe(key, false) {
        Ok(ident) => PropName::Ident(ident.into()),
        Err(key) => PropName::Str(key.as_str().into()),
    }
}

pub fn merge_prop_as_kv(key: Atom, value: Box<Expr>) -> Box<Prop> {
    Prop::KeyValue(KeyValueProp {
        key: merge_prop_ident_if_possible(key),
        value,
    })
    .into()
}

pub fn template_lit_class_expr(expr: Box<Expr>) -> Box<Expr> {
    BinExpr {
        span: DUMMY_SP,
        op: swc_core::ecma::ast::BinaryOp::LogicalOr,
        left: expr,
        right: "".into(),
    }
    .into()
}

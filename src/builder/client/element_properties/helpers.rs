use swc_core::{
    atoms::Atom,
    common::{SyntaxContext, DUMMY_SP},
    ecma::{
        ast::{
            AssignExpr, AssignTarget, BinExpr, BlockStmt, CallExpr, Expr, ExprOrSpread, ExprStmt,
            GetterProp, Ident, IdentName, KeyValueProp, MemberExpr, Prop, PropName, ReturnStmt,
            SimpleAssignTarget, Stmt,
        },
        utils::number::ToJsString,
    },
};

use crate::helpers::{common_into_expressions::ident_callee, generate_var_names::generate_use};

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

fn atom_to_key(atom: Atom) -> PropName {
    todo!();
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
        left: AssignTarget::Simple(SimpleAssignTarget::Ident(data.0.clone().into())),
        right: data.1.clone().into(),
    };
    Box::new(assign.into())
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
        callee: ident_callee(generate_use()),
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

pub fn merge_prop_as_kv(key: Atom, value: Box<Expr>) -> Box<Prop> {
    Prop::KeyValue(KeyValueProp {
        key: PropName::Str(key.as_str().into()),
        value,
    })
    .into()
}

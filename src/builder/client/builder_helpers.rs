use swc_core::{
    atoms::Atom,
    common::{util::take::Take, SyntaxContext, DUMMY_SP},
    ecma::ast::{
        ArrowExpr, BlockStmt, BlockStmtOrExpr, CallExpr, Callee, Expr, Ident, ParenExpr, Pat,
        UnaryExpr, UnaryOp,
    },
};

use crate::helpers::generate_var_names::generate_memo;

pub fn own_box_expr(old_expr: &mut Box<Expr>) -> Box<Expr> {
    let mut dummy = Box::new(Expr::dummy());
    std::mem::swap(old_expr, &mut dummy);
    dummy
}

pub fn name_as_expr(name: Atom) -> Box<Expr> {
    Box::new(Expr::Ident(Ident {
        span: DUMMY_SP,
        ctxt: SyntaxContext::empty(),
        sym: name,
        optional: false,
    }))
}

pub fn id_to_call_expr(name: Ident) -> CallExpr {
    CallExpr {
        span: DUMMY_SP,
        ctxt: SyntaxContext::empty(),
        callee: Callee::Expr(Box::new(Expr::Ident(name))),
        args: vec![],
        type_args: None,
    }
}

pub fn wrap_in_empty_arrow(bsoe: Box<BlockStmtOrExpr>) -> ArrowExpr {
    ArrowExpr {
        span: DUMMY_SP,
        ctxt: SyntaxContext::empty(),
        params: vec![],
        body: bsoe,
        is_async: false,
        is_generator: false,
        type_params: None,
        return_type: None,
    }
}

pub fn wrap_in_arrow(bsoe: Box<BlockStmtOrExpr>, params: Vec<Pat>) -> ArrowExpr {
    ArrowExpr {
        span: DUMMY_SP,
        ctxt: SyntaxContext::empty(),
        params,
        body: bsoe,
        is_async: false,
        is_generator: false,
        type_params: None,
        return_type: None,
    }
}

pub fn block_to_call_expr(block_stmt: BlockStmt) -> CallExpr {
    // Create arrow function expr and then wrap in iife
    let arrowfn = wrap_in_empty_arrow(BlockStmtOrExpr::BlockStmt(block_stmt).into());
    let paren = ParenExpr {
        span: DUMMY_SP,
        expr: Box::new(Expr::Arrow(arrowfn)),
    };
    CallExpr {
        span: DUMMY_SP,
        ctxt: SyntaxContext::empty(),
        callee: Callee::Expr(Box::new(Expr::Paren(paren))),
        args: vec![],
        type_args: None,
    }
}

pub fn wrap_with_memo(expr: Box<Expr>) -> CallExpr {
    CallExpr {
        span: DUMMY_SP,
        ctxt: SyntaxContext::empty(),
        callee: Callee::Expr(name_as_expr(generate_memo())),
        args: vec![expr.into()],
        type_args: None,
    }
}

pub fn memoize_bin_cond_expr(ex: Box<Expr>) -> Box<Expr> {
    let double_negated = UnaryExpr {
        span: DUMMY_SP,
        op: UnaryOp::Bang,
        arg: Box::new(Expr::Unary(UnaryExpr {
            span: DUMMY_SP,
            op: UnaryOp::Bang,
            arg: ex,
        })),
    };
    let arrowfn = wrap_in_empty_arrow(BlockStmtOrExpr::Expr(double_negated.into()).into());
    let memo = wrap_with_memo(Box::new(arrowfn.into()));
    Box::new(Expr::Call(CallExpr {
        span: DUMMY_SP,
        ctxt: SyntaxContext::empty(),
        callee: Callee::Expr(Box::new(memo.into())),
        args: vec![],
        type_args: None,
    }))
}

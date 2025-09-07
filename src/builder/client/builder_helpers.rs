use swc_core::{
    atoms::Atom,
    common::{SyntaxContext, DUMMY_SP},
    ecma::ast::{
        ArrowExpr, BlockStmt, BlockStmtOrExpr, CallExpr, Callee, Expr, ExprOrSpread, Ident,
        ParenExpr,
    },
};

pub fn expr_or_spread(expr: Box<Expr>) -> ExprOrSpread {
    ExprOrSpread { spread: None, expr }
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

pub fn block_to_call_expr(block_stmt: BlockStmt) -> CallExpr {
    // Create arrow function expr and then wrap in iife
    let arrowfn = ArrowExpr {
        span: DUMMY_SP,
        ctxt: SyntaxContext::empty(),
        params: vec![],
        body: Box::new(BlockStmtOrExpr::BlockStmt(block_stmt)),
        is_async: false,
        is_generator: false,
        type_params: None,
        return_type: None,
    };
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

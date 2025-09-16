use swc_core::{
    atoms::Atom,
    common::{source_map::PURE_SP, SyntaxContext, DUMMY_SP},
    ecma::ast::{
        Callee, Decl, Expr, ExprOrSpread, Ident, Stmt, VarDecl, VarDeclKind, VarDeclarator,
    },
};

pub fn ident_name(name: Atom, is_pure: bool) -> Ident {
    Ident {
        span: if is_pure { PURE_SP } else { DUMMY_SP },
        ctxt: SyntaxContext::empty(),
        sym: name,
        optional: false,
    }
}

pub fn ident_expr(name: Atom) -> Expr {
    Expr::Ident(ident_name(name, false))
}

pub fn ident_callee(name: Atom) -> Callee {
    Callee::Expr(Box::new(ident_expr(name)))
}

pub fn create_var_statement(decls: Vec<VarDeclarator>) -> Stmt {
    Stmt::Decl(Decl::Var(Box::new(VarDecl {
        span: DUMMY_SP,
        ctxt: SyntaxContext::empty(),
        kind: VarDeclKind::Var,
        declare: false,
        decls,
    })))
}

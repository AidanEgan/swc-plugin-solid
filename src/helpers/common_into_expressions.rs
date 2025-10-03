use swc_core::{
    atoms::Atom,
    common::{source_map::PURE_SP, SyntaxContext, DUMMY_SP},
    ecma::ast::{
        Callee, Decl, Expr, Ident, IdentName, Lit, MemberExpr, MemberProp, Stmt, UnaryExpr,
        UnaryOp, VarDecl, VarDeclKind, VarDeclarator,
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

pub fn ident_expr(name: Atom) -> Box<Expr> {
    // Will break
    let ex: Expr = if name.contains("-") {
        // String-ify
        name.into()
    } else {
        // As ident
        ident_name(name, false).into()
    };
    ex.into()
}

pub fn ident_callee(name: Atom) -> Callee {
    Callee::Expr(ident_expr(name))
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

pub fn create_lit_str_expr(val: &str) -> Box<Expr> {
    Expr::Lit(Lit::Str(val.into())).into()
}

pub fn create_double_negated(expr: Box<Expr>) -> Box<Expr> {
    UnaryExpr {
        span: DUMMY_SP,
        op: UnaryOp::Bang,
        arg: UnaryExpr {
            span: DUMMY_SP,
            op: UnaryOp::Bang,
            arg: expr,
        }
        .into(),
    }
    .into()
}

pub fn simple_member_expression(obj_name: Atom, prop_name: Atom) -> MemberExpr {
    MemberExpr {
        span: DUMMY_SP,
        obj: ident_expr(obj_name),
        prop: MemberProp::Ident(IdentName {
            span: DUMMY_SP,
            sym: prop_name,
        }),
    }
}

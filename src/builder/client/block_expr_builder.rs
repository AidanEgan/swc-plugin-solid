use swc_core::{
    common::{SyntaxContext, DUMMY_SP},
    ecma::ast::{
        BindingIdent, Decl, Expr, Ident, IdentName, MemberExpr, MemberProp, Pat, ReturnStmt, Stmt,
        VarDecl, VarDeclKind, VarDeclarator,
    },
};

use crate::{
    builder::{
        builder_types::{ElDeclKinds, Kind},
        client::builder_helpers::{id_to_call_expr, name_as_expr},
    },
    helpers::generate_var_names::{generate_el, generate_template_name},
};

pub struct BlockExprBuilder {
    element_declarations: Vec<(usize, ElDeclKinds)>,
    pub prev_kind: Kind,
    final_stmts: Vec<Stmt>,
    pub last_used_decl: Option<usize>,
}

impl BlockExprBuilder {
    pub fn add_decl(&mut self, count: usize) {
        match self.prev_kind {
            Kind::None => {
                self.element_declarations.push((count, ElDeclKinds::Tmpl));
            }
            Kind::Close(at) => {
                self.element_declarations
                    .push((count, ElDeclKinds::NextSibling(at)));
            }
            Kind::Open(at, is_implicit_close) => {
                if is_implicit_close {
                    self.element_declarations
                        .push((count, ElDeclKinds::NextSibling(at)));
                } else {
                    self.element_declarations
                        .push((count, ElDeclKinds::FirstChild(at)));
                }
            }
            Kind::Placeholder(at, _, first_child) => {
                if first_child {
                    self.element_declarations
                        .push((count, ElDeclKinds::FirstChild(at)));
                } else {
                    self.element_declarations
                        .push((count, ElDeclKinds::NextSibling(at)));
                }
            }
            Kind::Text(at) => {
                self.element_declarations
                    .push((count, ElDeclKinds::NextSibling(at)));
            }
        }
    }

    pub fn add_decl_specific(&mut self, count: usize, data: ElDeclKinds) {
        self.element_declarations.push((count, data));
    }

    pub fn set_last_used_decl(&mut self, new_val: usize) {
        self.last_used_decl = Some(new_val);
    }

    pub fn get_final_stmts(&mut self) -> &mut Vec<Stmt> {
        &mut self.final_stmts
    }

    pub fn add_decls_to_final(&mut self, temp_id: usize) {
        let return_id = self.element_declarations.first().map(|f| f.0);
        let last = self.last_used_decl.unwrap_or(0_usize);
        if let Some(Stmt::Decl(Decl::Var(vardecl))) = self.final_stmts.get_mut(0) {
            // Add new var decls
            for (el_id, decl) in self
                .element_declarations
                .drain(..)
                .take_while(|(cnt, _)| *cnt <= last)
            {
                let expr = match decl {
                    ElDeclKinds::Tmpl => Box::new(Expr::Call(id_to_call_expr(Ident {
                        span: DUMMY_SP,
                        ctxt: SyntaxContext::empty(),
                        sym: generate_template_name(temp_id),
                        optional: false,
                    }))),
                    ElDeclKinds::FirstChild(el) => Box::new(Expr::Member(MemberExpr {
                        span: DUMMY_SP,
                        obj: name_as_expr(generate_el(el)),
                        prop: MemberProp::Ident(IdentName {
                            span: DUMMY_SP,
                            sym: "firstChild".into(),
                        }),
                    })),
                    ElDeclKinds::NextSibling(el) => Box::new(Expr::Member(MemberExpr {
                        span: DUMMY_SP,
                        obj: name_as_expr(generate_el(el)),
                        prop: MemberProp::Ident(IdentName {
                            span: DUMMY_SP,
                            sym: "nextSibling".into(),
                        }),
                    })),
                };

                let transformed_decl = VarDeclarator {
                    span: DUMMY_SP,
                    name: Pat::Ident(BindingIdent {
                        type_ann: None,
                        id: Ident {
                            span: DUMMY_SP,
                            ctxt: SyntaxContext::empty(),
                            sym: generate_el(el_id),
                            optional: false,
                        },
                    }),
                    init: Some(expr),
                    definite: false,
                };
                vardecl.decls.push(transformed_decl);
            }
        }
        // Add return stmt
        if let Some(return_id) = return_id {
            self.final_stmts.push(Stmt::Return(ReturnStmt {
                span: DUMMY_SP,
                arg: Some(Box::new(Expr::Ident(Ident {
                    span: DUMMY_SP,
                    ctxt: SyntaxContext::empty(),
                    sym: generate_el(return_id),
                    optional: false,
                }))),
            }))
        }
    }

    pub fn add_stmt(&mut self, stmt: Stmt) {
        self.final_stmts.push(stmt);
    }

    pub fn add_penultimate_stmt(&mut self, stmt: Stmt) {
        self.final_stmts.push(stmt);
        let len = self.final_stmts.len();
        if len > 1 {
            self.final_stmts.swap(len - 1, len - 2);
        }
    }

    pub fn take_final_stmts(&mut self) -> Vec<Stmt> {
        let mut blank: Vec<Stmt> = Vec::new();
        std::mem::swap(&mut blank, self.get_final_stmts());
        blank
    }

    pub fn set_kind(&mut self, k: Kind) {
        self.prev_kind = k;
    }
    pub fn new() -> Self {
        let el_dec_stmts = VarDecl {
            span: DUMMY_SP,
            ctxt: SyntaxContext::empty(),
            kind: VarDeclKind::Var,
            declare: false,
            decls: vec![],
        };
        Self {
            element_declarations: Vec::new(),
            prev_kind: Kind::None,
            final_stmts: vec![Stmt::Decl(Decl::Var(Box::new(el_dec_stmts)))],
            last_used_decl: None,
        }
    }
}

/*
let mut element_declarations: Vec<(usize, ElDeclKinds)> = Vec::new();
let mut prev_kind = Kind::None;


let el_dec_stmts = VarDecl {
    span: DUMMY_SP,
    ctxt: SyntaxContext::empty(),
    kind: VarDeclKind::Var,
    declare: false,
    decls: vec![],
};
let mut tmp_stmts: Vec<Stmt> = vec![Stmt::Decl(Decl::Var(Box::new(el_dec_stmts)))];
*/

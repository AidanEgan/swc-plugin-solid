use swc_core::{
    atoms::Atom,
    common::{SyntaxContext, DUMMY_SP},
    ecma::ast::{
        BindingIdent, BlockStmt, BlockStmtOrExpr, CallExpr, Decl, Expr, ExprStmt, Ident,
        KeyValueProp, MemberExpr, MemberProp, ObjectLit, Pat, Prop, PropName, PropOrSpread, Stmt,
        VarDecl, VarDeclKind, VarDeclarator,
    },
};

use crate::{
    builder::client::{
        builder_helpers::{wrap_in_arrow, wrap_in_empty_arrow},
        element_properties::ElementPropertiesBuilder,
    },
    helpers::{
        common_into_expressions::{ident_callee, ident_expr},
        generate_var_names::{generate_effect, generate_effect_arg},
    },
    transform::parent_visitor::ParentVisitor,
};

#[derive(Debug)]
pub enum EffectVariant {
    Class,
    Style,
    Std,
}

#[derive(Debug)]
pub struct EffectMetadata {
    pub expr: Box<Expr>,
    pub v_val: Ident,
    pub obj: MemberExpr,
    pub variant: EffectVariant,
    pub count: usize,
    pub name: Atom,
}

#[derive(Debug)]
pub struct EffectBuilder {
    arrow_expr_args: Vec<Atom>,
    vars: VarDecl,
    pub data: Vec<EffectMetadata>,
    obj_props: Vec<Atom>,
}

impl EffectBuilder {
    pub fn new() -> Self {
        Self {
            arrow_expr_args: vec![],
            vars: VarDecl {
                span: DUMMY_SP,
                ctxt: SyntaxContext::empty(),
                kind: VarDeclKind::Var,
                declare: false,
                decls: vec![],
            },
            data: vec![],
            obj_props: vec![],
        }
    }

    // should be unique i guess
    pub fn get_new_obj_prop(&mut self) -> Atom {
        let mut v = self.obj_props.len();
        let mut res: Vec<u8> = vec![];
        while v > 25 {
            // Cast is fine b/c of mod
            res.push((v % 26_usize) as u8 + 97_u8);
            v /= 26;
        }
        res.push(v as u8 + 97_u8);
        let as_atom: Atom = unsafe { &*str::from_utf8_unchecked_mut(res.as_mut_slice()) }.into();
        self.obj_props.push(as_atom.clone());
        as_atom
    }

    pub fn get_new_obj(&mut self) -> MemberExpr {
        let effect_arg = generate_effect_arg();
        if !self.arrow_expr_args.contains(&effect_arg) {
            self.arrow_expr_args.push(effect_arg.clone());
        }
        MemberExpr {
            span: DUMMY_SP,
            obj: ident_expr(effect_arg),
            prop: MemberProp::Ident(self.get_new_obj_prop().into()),
        }
    }

    pub fn build_effect(&mut self, mut stmts: Vec<Stmt>) -> Option<Stmt> {
        for datum in self.data.iter() {
            self.vars.decls.push(VarDeclarator {
                span: DUMMY_SP,
                name: Pat::Ident(BindingIdent {
                    // Don't HAVE to clone, but probs cheaper
                    id: datum.name.clone().into(),
                    type_ann: None,
                }),
                init: None,
                definite: false,
            });
        }
        if stmts.len() == 1 {
            let only_stmt = stmts.pop()?;
            if let Stmt::Expr(inner_expr) = only_stmt {
                let box_e: Box<Expr> =
                    wrap_in_empty_arrow(BlockStmtOrExpr::Expr(inner_expr.expr).into()).into();
                let e = CallExpr {
                    span: DUMMY_SP,
                    ctxt: SyntaxContext::empty(),
                    callee: ident_callee(generate_effect()),
                    args: vec![box_e.into()],
                    type_args: None,
                };
                Some(Stmt::Expr(ExprStmt {
                    span: DUMMY_SP,
                    expr: e.into(),
                }))
            } else {
                None
            }
        } else {
            let mut final_stmts =
                vec![Stmt::Decl(Decl::Var(std::mem::take(&mut self.vars).into()))];
            final_stmts.extend(stmts);
            let inner_arr: Box<Expr> = wrap_in_arrow(
                BlockStmtOrExpr::BlockStmt(BlockStmt {
                    span: DUMMY_SP,
                    ctxt: SyntaxContext::empty(),
                    stmts: final_stmts,
                })
                .into(),
                self.arrow_expr_args
                    .drain(..)
                    .map(|p| Pat::Ident(p.into()))
                    .collect(),
            )
            .into();
            let props: Box<Expr> = ObjectLit {
                span: DUMMY_SP,
                props: self
                    .obj_props
                    .drain(..)
                    .map(|p| {
                        PropOrSpread::Prop(
                            Prop::KeyValue(KeyValueProp {
                                key: PropName::Ident(p.into()),
                                value: ident_expr("undefined".into()),
                            })
                            .into(),
                        )
                    })
                    .collect(),
            }
            .into();
            let e = CallExpr {
                span: DUMMY_SP,
                ctxt: SyntaxContext::empty(),
                callee: ident_callee(generate_effect()),
                args: vec![inner_arr.into(), props.into()],
                type_args: None,
            };
            Some(Stmt::Expr(ExprStmt {
                span: DUMMY_SP,
                expr: e.into(),
            }))
        }
    }
}

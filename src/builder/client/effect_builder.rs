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
    builder::client::builder_helpers::{wrap_in_arrow, wrap_in_empty_arrow},
    helpers::{
        common_into_expressions::{ident_callee, ident_expr},
        generate_var_names::{generate_effect, generate_effect_arg, generate_v},
    },
};

#[derive(Debug)]
pub enum EffectVariant {
    Class,
    ClassList(bool), // Determines if _$className or el.classname.toggle is used
    Style(Option<PropName>), // Determines which style js fn to use
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
    uses_arg: bool,
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
            uses_arg: false,
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

    pub fn set_uses_arg(&mut self, uses: bool) {
        self.uses_arg = uses;
    }

    pub fn add_data(
        &mut self,
        expr: Box<Expr>,
        variant: EffectVariant,
        assigned_v: usize,
        count: usize,
        prop: &str,
    ) {
        let assigned_prop = self.get_new_obj();
        self.data.push(EffectMetadata {
            expr,
            v_val: generate_v(assigned_v).into(),
            obj: assigned_prop,
            variant,
            count,
            name: prop.into(),
        });
    }

    pub fn add_var_decl(&mut self, name: Atom, value: Option<Box<Expr>>) {
        self.vars.decls.push(VarDeclarator {
            span: DUMMY_SP,
            name: Pat::Ident(BindingIdent {
                // Don't HAVE to clone, but probs cheaper
                id: name.into(),
                type_ann: None,
            }),
            init: value,
            definite: false,
        });
    }

    // Pretty specific logic here really only meant to work with
    // element properties builder. Was hard to break this logic up :(
    pub fn build_effect(&mut self, mut stmts: Vec<Stmt>) -> Option<Stmt> {
        if stmts.len() == 1 {
            let only_stmt = stmts.pop()?;
            if let Stmt::Expr(inner_expr) = only_stmt {
                let box_e: Box<Expr> = if !self.uses_arg {
                    wrap_in_empty_arrow(BlockStmtOrExpr::Expr(inner_expr.expr).into()).into()
                } else {
                    wrap_in_arrow(
                        BlockStmtOrExpr::Expr(inner_expr.expr).into(),
                        vec![Pat::Ident(generate_effect_arg().into())],
                    )
                    .into()
                };

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

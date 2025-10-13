use swc_core::{
    atoms::Atom,
    common::{SyntaxContext, DUMMY_SP},
    ecma::ast::{
        BindingIdent, BlockStmt, BlockStmtOrExpr, CallExpr, Decl, Expr, ExprStmt, Ident,
        KeyValueProp, MemberExpr, MemberProp, ObjectLit, ParenExpr, Pat, Prop, PropName,
        PropOrSpread, ReturnStmt, Stmt, VarDecl, VarDeclKind, VarDeclarator,
    },
};

use crate::{
    builder::client::builder_helpers::{wrap_in_arrow, wrap_in_empty_arrow},
    helpers::{
        common_into_expressions::{ident_callee, ident_expr, ident_name},
        generate_var_names::{generate_effect, generate_effect_arg, generate_v},
    },
};

#[derive(Debug)]
pub enum EffectVariant {
    Prop,
    Class,
    ClassName,
    ClassList(bool), // Determines if _$className or el.classname.toggle is used
    Style(Option<PropName>), // Determines which style js fn to use
    Std(bool),       // true = setBooleanAttr, false = setAttr
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
    vars: VarDecl,
    pub data: Vec<EffectMetadata>,
    pub isolated_effects: Vec<EffectMetadata>,
    obj_props: Vec<Atom>,
    uses_effect_arg: bool,
}

impl EffectBuilder {
    pub fn new() -> Self {
        Self {
            vars: VarDecl {
                span: DUMMY_SP,
                ctxt: SyntaxContext::empty(),
                kind: VarDeclKind::Var,
                declare: false,
                decls: vec![],
            },
            data: vec![],
            obj_props: vec![],
            isolated_effects: vec![],
            uses_effect_arg: false,
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

    pub fn get_effect_arg(&mut self) -> Atom {
        let effect_arg = generate_effect_arg();
        self.uses_effect_arg = true;
        effect_arg
    }

    pub fn set_uses_effect(&mut self, new_val: bool) -> bool {
        let old = self.uses_effect_arg;
        self.uses_effect_arg = new_val;
        old
    }

    pub fn get_new_obj(&mut self) -> MemberExpr {
        // 'Uses effect arg will be implicit'
        let effect_arg = generate_effect_arg();
        MemberExpr {
            span: DUMMY_SP,
            obj: ident_expr(effect_arg),
            prop: MemberProp::Ident(self.get_new_obj_prop().into()),
        }
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

    pub fn add_var_decl(&mut self, id: Ident, value: Option<Box<Expr>>) {
        self.vars.decls.push(VarDeclarator {
            span: DUMMY_SP,
            name: Pat::Ident(BindingIdent {
                // Don't HAVE to clone, but probs cheaper
                id,
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
                /*
                 * Why did I do this? Am I missing some case?
                let expr = if inner_expr.expr.is_paren()
                    && inner_expr.expr.as_paren().unwrap().expr.is_assign()
                {
                    inner_expr.expr.expect_paren().expr
                } else {
                    inner_expr.expr
                };
                */
                // Swc may remove parens if not needed
                let expr = if inner_expr.expr.is_paren() {
                    inner_expr.expr
                } else {
                    Expr::Paren(ParenExpr {
                        span: DUMMY_SP,
                        expr: inner_expr.expr,
                    })
                    .into()
                };
                let box_e: Box<Expr> = if !self.uses_effect_arg {
                    wrap_in_empty_arrow(BlockStmtOrExpr::Expr(expr).into()).into()
                } else {
                    wrap_in_arrow(
                        BlockStmtOrExpr::Expr(expr).into(),
                        vec![Pat::Ident(ident_name(generate_effect_arg(), false).into())],
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
            //return _p$;
            final_stmts.push(Stmt::Return(ReturnStmt {
                span: DUMMY_SP,
                arg: Some(ident_expr(generate_effect_arg())),
            }));
            let inner_arr: Box<Expr> = wrap_in_arrow(
                BlockStmtOrExpr::BlockStmt(BlockStmt {
                    span: DUMMY_SP,
                    ctxt: SyntaxContext::empty(),
                    stmts: final_stmts,
                })
                .into(),
                vec![Pat::Ident(ident_name(generate_effect_arg(), false).into())],
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

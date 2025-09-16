/*
 * JSX Opening Element can have a bunch of properties that have custom logic for how to build them.
 * Notably, properties like class/className, ref, and style all have custom logic
 * Additionally there is cusom logic for expressions and spread exprs
 * TODO: SOME CUSTOM COMPONENT LOGIC LIVES IN STD BUILDER BUT SHOULD PROBABLY BE MOVED HERE
 * SINCE OTHER LOGIC WILL LIVE HERE ANYWAY
 */

use std::collections::{HashMap, HashSet};
use swc_core::{
    common::{SyntaxContext, DUMMY_SP},
    ecma::ast::{
        AssignExpr, AssignTarget, BinExpr, BindingIdent, CallExpr, CondExpr, Decl, Expr,
        ExprOrSpread, ExprStmt, Lit, Pat, SimpleAssignTarget, Stmt, UnaryExpr, UnaryOp, VarDecl,
        VarDeclKind, VarDeclarator,
    },
};

use crate::helpers::{
    common_into_expressions::{ident_callee, ident_expr, ident_name},
    generate_var_names::{generate_el, generate_ref, generate_use},
};

fn generate_use_expr(args: Vec<ExprOrSpread>) -> Box<Expr> {
    Box::new(Expr::Call(CallExpr {
        span: DUMMY_SP,
        ctxt: SyntaxContext::empty(),
        callee: ident_callee(generate_use()),
        args,
        type_args: None,
    }))
}

#[derive(Default, Debug, Clone)]
pub struct ElementPropertiesBuilder {
    local_ref_count: usize,
    pub statements: Vec<Stmt>,
    pub used_imports: HashSet<String>,
    pub used_events: HashSet<String>,
    pub direct_template_inserts: HashMap<String, String>,
}

impl ElementPropertiesBuilder {
    pub fn new(ref_count: usize) -> Self {
        Self {
            local_ref_count: ref_count,
            statements: vec![],
            used_imports: HashSet::new(),
            used_events: HashSet::new(),
            direct_template_inserts: HashMap::new(),
        }
    }

    // Fns for each type of opening element
    // Returns og expr if not transformed
    pub fn create_ref_statements(
        &mut self,
        element_count: Option<usize>,
        ref_expr: Box<Expr>,
    ) -> Option<Box<Expr>> {
        /*
         * Custom
         * ref(r$) {
            var _ref$4 = ref;
            typeof _ref$4 === "function" ? _ref$4(r$) : ref = r$;
           },
         * -- AS FN --
         * is just the raw fn expr -> handled elsewhere
         *
         * Standard
         * var _ref$2 = otherRef;
           typeof _ref$2 === "function" ? _$use(_ref$2, _el$3) : otherRef = _el$3;
         * -- AS FN --
         * _$use(x => setValue(x), _el$4);
        */

        // As fn
        if ref_expr.is_arrow() || ref_expr.is_fn_expr() {
            if element_count.is_none() {
                // Terminate early for custom component
                return Some(ref_expr);
            }
            self.statements.push(Stmt::Expr(ExprStmt {
                span: DUMMY_SP,
                expr: generate_use_expr(vec![ref_expr.into()]),
            }));
            return None;
        }
        // Will be declaring refs
        self.local_ref_count += 1;

        // Non fn inner part
        let ref_decl = VarDeclarator {
            span: DUMMY_SP,
            name: Pat::Ident(BindingIdent {
                id: ident_name(generate_ref(self.local_ref_count), false),
                type_ann: None,
            }),
            // Used twice so it has to be cloned. Should just be an ident anyway
            init: Some(ref_expr.clone()),
            definite: false,
        };
        self.statements.push(Stmt::Decl(Decl::Var(Box::new(VarDecl {
            span: DUMMY_SP,
            ctxt: SyntaxContext::empty(),
            kind: VarDeclKind::Var,
            declare: false,
            decls: vec![ref_decl],
        }))));

        let function_ternary_arm = if let Some(element_count) = element_count {
            generate_use_expr(vec![
                ident_expr(generate_ref(self.local_ref_count)).into(),
                ident_expr(generate_el(element_count)).into(),
            ])
        } else {
            Box::new(Expr::Call(CallExpr {
                span: DUMMY_SP,
                ctxt: SyntaxContext::empty(),
                callee: ident_callee(generate_ref(self.local_ref_count)),
                args: vec![ident_expr("r$".into()).into()],
                type_args: None,
            }))
        };

        let assignment_ternary_arm = ref_expr.ident().map(|ident| {
            Expr::Assign(AssignExpr {
                span: DUMMY_SP,
                op: swc_core::ecma::ast::AssignOp::Assign,
                left: AssignTarget::Simple(SimpleAssignTarget::Ident(BindingIdent {
                    id: ident,
                    type_ann: None,
                })),
                right: Box::new(ident_expr(if let Some(element_count) = element_count {
                    generate_el(element_count)
                } else {
                    "r$".into()
                })),
            })
        });

        let typeof_expr = Expr::Bin(BinExpr {
            span: DUMMY_SP,
            op: swc_core::ecma::ast::BinaryOp::EqEqEq,
            left: Box::new(Expr::Unary(UnaryExpr {
                span: DUMMY_SP,
                op: UnaryOp::TypeOf,
                arg: Box::new(ident_expr(generate_ref(self.local_ref_count))),
            })),
            right: Box::new(Expr::Lit(Lit::Str("function".into()))),
        });

        let finalized_expr = if let Some(assignment_ternary_arm) = assignment_ternary_arm {
            Expr::Cond(CondExpr {
                span: DUMMY_SP,
                test: Box::new(typeof_expr),
                cons: function_ternary_arm,
                alt: Box::new(assignment_ternary_arm),
            })
        } else {
            Expr::Bin(BinExpr {
                span: DUMMY_SP,
                op: swc_core::ecma::ast::BinaryOp::LogicalAnd,
                left: Box::new(typeof_expr),
                right: function_ternary_arm,
            })
        };

        self.statements.push(Stmt::Expr(ExprStmt {
            span: DUMMY_SP,
            expr: Box::new(finalized_expr),
        }));
        None
    }

    pub fn create_class_statements() {}
}

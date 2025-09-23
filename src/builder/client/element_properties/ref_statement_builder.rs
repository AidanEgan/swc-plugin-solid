use swc_core::{
    common::{SyntaxContext, DUMMY_SP},
    ecma::ast::{
        AssignExpr, AssignTarget, BinExpr, BindingIdent, CallExpr, CondExpr, Decl, Expr, ExprStmt,
        Lit, Pat, SimpleAssignTarget, Stmt, UnaryExpr, UnaryOp, VarDecl, VarDeclKind,
        VarDeclarator,
    },
};

use crate::{
    builder::client::element_properties::{generate_use_expr, ElementPropertiesBuilder},
    helpers::{
        common_into_expressions::{ident_callee, ident_expr, ident_name},
        generate_var_names::{generate_el, generate_ref, generate_use},
    },
    transform::parent_visitor::ParentVisitor,
};

// Fns for each type of opening element
// Returns og expr if not transformed
pub fn create_ref_statements(
    statements: &mut Vec<Stmt>,
    local_ref_count: &mut usize,
    element_count: Option<usize>,
    ref_expr: Box<Expr>,
) -> (Option<Box<Expr>>, bool) {
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
    let mut import_use = false;

    // As fn
    if ref_expr.is_arrow() || ref_expr.is_fn_expr() {
        if element_count.is_none() {
            // Terminate early for custom component
            return (Some(ref_expr), import_use);
        }
        import_use = true;
        statements.push(Stmt::Expr(ExprStmt {
            span: DUMMY_SP,
            expr: generate_use_expr(vec![ref_expr.into()]),
        }));
        return (None, import_use);
    }
    // Will be declaring refs
    *local_ref_count += 1;

    // Non fn inner part
    let ref_decl = VarDeclarator {
        span: DUMMY_SP,
        name: Pat::Ident(BindingIdent {
            id: ident_name(generate_ref(*local_ref_count), false),
            type_ann: None,
        }),
        // Used twice so it has to be cloned. Should just be an ident anyway
        init: Some(ref_expr.clone()),
        definite: false,
    };
    statements.push(Stmt::Decl(Decl::Var(Box::new(VarDecl {
        span: DUMMY_SP,
        ctxt: SyntaxContext::empty(),
        kind: VarDeclKind::Var,
        declare: false,
        decls: vec![ref_decl],
    }))));

    let function_ternary_arm = if let Some(element_count) = element_count {
        import_use = true;
        generate_use_expr(vec![
            ident_expr(generate_ref(*local_ref_count)).into(),
            ident_expr(generate_el(element_count)).into(),
        ])
    } else {
        Box::new(Expr::Call(CallExpr {
            span: DUMMY_SP,
            ctxt: SyntaxContext::empty(),
            callee: ident_callee(generate_ref(*local_ref_count)),
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
            right: Box::new({
                let name = if let Some(element_count) = element_count {
                    generate_el(element_count)
                } else {
                    "r$".into()
                };
                Expr::Ident(ident_name(name, false))
            }),
        })
    });

    let typeof_expr = Expr::Bin(BinExpr {
        span: DUMMY_SP,
        op: swc_core::ecma::ast::BinaryOp::EqEqEq,
        left: Box::new(Expr::Unary(UnaryExpr {
            span: DUMMY_SP,
            op: UnaryOp::TypeOf,
            arg: ident_expr(generate_ref(*local_ref_count)),
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

    statements.push(Stmt::Expr(ExprStmt {
        span: DUMMY_SP,
        expr: Box::new(finalized_expr),
    }));
    (None, import_use)
}

impl<'a, T: ParentVisitor> ElementPropertiesBuilder<'a, T> {
    pub fn ref_builder(&mut self, element_count: Option<usize>, ref_expr: Box<Expr>) {
        let (_, used_use) = create_ref_statements(
            &mut self.statements,
            self.parent_visitor.ref_count(),
            element_count,
            ref_expr,
        );
        if used_use {
            self.parent_visitor
                .add_import(generate_use().as_str().into());
        };
    }
}

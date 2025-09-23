use swc_core::{
    atoms::Atom,
    common::{SyntaxContext, DUMMY_SP},
    ecma::{
        ast::{
            AssignExpr, AssignTarget, CallExpr, Callee, Expr, ExprOrSpread, ExprStmt, IdentName,
            Lit, MemberExpr, MemberProp, SimpleAssignTarget, Stmt,
        },
        utils::ExprExt,
    },
};

use crate::{
    builder::client::element_properties::ElementPropertiesBuilder,
    constants::events::is_delegated_event,
    helpers::{
        common_into_expressions::{create_lit_str_expr, ident_callee, ident_expr, ident_name},
        generate_var_names::{generate_add_event_listener, generate_el, DELEGATE_EVENTS},
    },
    transform::parent_visitor::{self, ParentVisitor},
};

const ADD_EVENT_LISTENER: &str = "addEventListener";

fn use_double_dolar_syntax<T: ParentVisitor>(parent_visitor: &mut T, expr: &Box<Expr>) -> bool {
    expr.is_array_lit()
        || expr.is_arrow()
        || expr.is_fn_expr()
        || if let Some(ident) = expr.as_ident() {
            parent_visitor.get_var_if_in_scope(&ident.sym).is_some()
        } else {
            false
        }
}

fn simple_member_expression(obj_name: Atom, prop_name: Atom) -> MemberExpr {
    MemberExpr {
        span: DUMMY_SP,
        obj: ident_expr(obj_name),
        prop: MemberProp::Ident(IdentName {
            span: DUMMY_SP,
            sym: prop_name,
        }),
    }
}

fn add_event_listener_capture(element_count: usize, args: Vec<ExprOrSpread>) -> Box<Expr> {
    let call_expr = CallExpr {
        span: DUMMY_SP,
        ctxt: SyntaxContext::empty(),
        callee: Callee::Expr(
            simple_member_expression(generate_el(element_count), ADD_EVENT_LISTENER.into()).into(),
        ),
        args,
        type_args: None,
    };
    Box::new(call_expr.into())
}

fn add_event_listener(args: Vec<ExprOrSpread>) -> Box<Expr> {
    let call_expr = CallExpr {
        span: DUMMY_SP,
        ctxt: SyntaxContext::empty(),
        callee: ident_callee(generate_add_event_listener()),
        args,
        type_args: None,
    };
    Box::new(call_expr.into())
}

impl<'a, T: ParentVisitor> ElementPropertiesBuilder<'a, T> {
    pub fn event_handler_builder(
        &mut self,
        event_name: &str,
        element_count: usize,
        event_expr: Box<Expr>,
    ) {
        // Can be delegated, onNondelegated, on:event, oncapture:event
        if event_name.starts_with("on:") {
            self.parent_visitor
                .add_import(generate_add_event_listener().as_str().into());
            let actual_name = event_name[3..].to_lowercase();
            let res = add_event_listener(vec![
                Expr::Ident(ident_name(generate_el(element_count), false)).into(),
                create_lit_str_expr(actual_name.as_str()).into(),
                event_expr.into(),
            ]);
            self.statements.push(Stmt::Expr(ExprStmt {
                span: DUMMY_SP,
                expr: res.into(),
            }));
        } else if event_name.starts_with("oncapture:") {
            let actual_name = event_name[10..].to_lowercase();
            self.statements.push(Stmt::Expr(ExprStmt {
                span: DUMMY_SP,
                expr: add_event_listener_capture(
                    element_count,
                    vec![
                        create_lit_str_expr(actual_name.as_str()).into(),
                        event_expr.into(),
                        Expr::Lit(Lit::Bool(true.into())).into(),
                    ],
                ),
            }));
        } else {
            let actual_name = event_name[2..].to_lowercase();
            if is_delegated_event(&actual_name) {
                self.parent_visitor.add_import(DELEGATE_EVENTS.into());
                self.parent_visitor.add_event(actual_name.as_str().into());
                if use_double_dolar_syntax(self.parent_visitor, &event_expr) {
                    let mut event_expr = event_expr;
                    let mut extra_data: Option<Box<Expr>> = None;
                    // Need to pull data out of array expressions
                    if let Some(arr_expr) = event_expr.as_mut_array() {
                        if let Some(Some(e)) = arr_expr.elems.get_mut(1) {
                            extra_data = Some(std::mem::take(&mut e.expr));
                        };
                        // Assignment to event_expr must be done last
                        if let Some(Some(e)) = arr_expr.elems.get_mut(0) {
                            event_expr = std::mem::take(&mut e.expr);
                        };
                    };

                    let assignment = AssignExpr {
                        span: DUMMY_SP,
                        op: swc_core::ecma::ast::AssignOp::Assign,
                        left: AssignTarget::Simple(SimpleAssignTarget::Member(
                            simple_member_expression(
                                generate_el(element_count),
                                format!("{0}{1}", "$$", actual_name).into(),
                            ),
                        )),
                        right: event_expr,
                    };
                    self.statements.push(Stmt::Expr(ExprStmt {
                        span: DUMMY_SP,
                        expr: Box::new(assignment.into()),
                    }));
                    if let Some(extra_data) = extra_data {
                        let assignment = AssignExpr {
                            span: DUMMY_SP,
                            op: swc_core::ecma::ast::AssignOp::Assign,
                            left: AssignTarget::Simple(SimpleAssignTarget::Member(
                                simple_member_expression(
                                    generate_el(element_count),
                                    format!("{0}{1}{2}", "$$", actual_name, "Data").into(),
                                ),
                            )),
                            right: extra_data,
                        };
                        self.statements.push(Stmt::Expr(ExprStmt {
                            span: DUMMY_SP,
                            expr: Box::new(assignment.into()),
                        }));
                    }
                } else {
                    self.parent_visitor
                        .add_import(generate_add_event_listener().as_str().into());
                    self.statements.push(Stmt::Expr(ExprStmt {
                        span: DUMMY_SP,
                        expr: add_event_listener(vec![
                            ident_expr(generate_el(element_count)).into(),
                            create_lit_str_expr(actual_name.as_str()).into(),
                            event_expr.into(),
                            Expr::Lit(Lit::Bool(true.into())).into(),
                        ]),
                    }));
                }
            } else {
                self.parent_visitor
                    .add_import(generate_add_event_listener().as_str().into());
                self.statements.push(Stmt::Expr(ExprStmt {
                    span: DUMMY_SP,
                    expr: add_event_listener(vec![
                        ident_expr(generate_el(element_count)).into(),
                        create_lit_str_expr(actual_name.as_str()).into(),
                        event_expr.into(),
                    ]),
                }));
            }
        }
    }
}

use swc_core::{
    common::{SyntaxContext, DUMMY_SP},
    ecma::ast::{CallExpr, Callee, Expr, ExprStmt, Lit, Null, Stmt},
};

use crate::{
    builder::client::builder_helpers::{expr_or_spread, name_as_expr},
    helpers::generate_var_names::{generate_el, generate_insert},
};

pub struct InsertQueue {
    pub queue: Vec<InsertBuilder>,
    pub was_first_el: bool,
}

impl InsertQueue {
    pub fn new() -> Self {
        Self {
            queue: vec![],
            was_first_el: false,
        }
    }
    pub fn drain_insert_queue(&mut self, before: PossibleInsert, tmp_stmts: &mut Vec<Stmt>) {
        let before = match before {
            PossibleInsert::Null => {
                if self.was_first_el && self.queue.len() == 1 {
                    PossibleInsert::Undefined
                } else {
                    PossibleInsert::Null
                }
            }
            x => x,
        };
        for builder in self.queue.drain(..) {
            tmp_stmts.push(Stmt::Expr(ExprStmt {
                span: DUMMY_SP,
                expr: create_insert_expr(builder, before),
            }));
        }
        self.was_first_el = false;
    }
    pub fn add(&mut self, new_val: InsertBuilder, was_first: bool) {
        self.queue.push(new_val);
        self.was_first_el = self.was_first_el || was_first;
    }
}

pub struct InsertBuilder {
    pub parent_el: usize,
    pub expr: Box<Expr>,
}

#[derive(Copy, Clone)]
pub enum PossibleInsert {
    At(usize),
    Null,
    Undefined,
}
fn create_insert_expr(insert: InsertBuilder, before: PossibleInsert) -> Box<Expr> {
    let before = match before {
        PossibleInsert::At(dom_el_id) => expr_or_spread(name_as_expr(generate_el(dom_el_id))), // DOM Element
        PossibleInsert::Null => {
            expr_or_spread(Box::new(Expr::Lit(Lit::Null(Null { span: DUMMY_SP }))))
        }
        PossibleInsert::Undefined => expr_or_spread(name_as_expr("undefined".into())),
    };
    Box::new(Expr::Call(CallExpr {
        span: DUMMY_SP,
        ctxt: SyntaxContext::empty(),
        callee: Callee::Expr(name_as_expr(generate_insert())),
        args: vec![
            expr_or_spread(name_as_expr(generate_el(insert.parent_el))), // Parent
            expr_or_spread(insert.expr),                                 // Expression
            before,
            // There is also an 'initial' arg but i dont think client rendering uses it
        ],
        type_args: None,
    }))
}

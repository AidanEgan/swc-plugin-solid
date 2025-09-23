use swc_core::{
    common::{SyntaxContext, DUMMY_SP},
    ecma::ast::{CallExpr, Callee, Expr, ExprOrSpread, ExprStmt, Lit, Null, Stmt},
};

use crate::{
    builder::client::builder_helpers::name_as_expr,
    helpers::generate_var_names::{generate_el, generate_insert},
};

pub struct InsertQueue {
    pub queue: Vec<InsertBuilder>,
    pub was_first_el: bool,
    pub used_insert: bool,
}

impl InsertQueue {
    pub fn new() -> Self {
        Self {
            queue: vec![],
            was_first_el: false,
            used_insert: false,
        }
    }
    pub fn drain_insert_queue(&mut self, before: PossibleInsert, tmp_stmts: &mut Vec<Stmt>) {
        // Tells if needs insert import
        self.used_insert = self.used_insert || !self.queue.is_empty();
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
    let before: Option<ExprOrSpread> = match before {
        PossibleInsert::At(dom_el_id) => Some(name_as_expr(generate_el(dom_el_id)).into()), // DOM Element
        PossibleInsert::Null => {
            Some(Box::new(Expr::Lit(Lit::Null(Null { span: DUMMY_SP }))).into())
        }
        PossibleInsert::Undefined => None, //name_as_expr("undefined".into()).into(),
    };
    let mut args: Vec<ExprOrSpread> = vec![
        name_as_expr(generate_el(insert.parent_el)).into(), // Parent
        insert.expr.into(),                                 // Expression
                                                            // Add before if applicable
                                                            // There is also an 'initial' arg but i dont think client rendering uses it
    ];
    if let Some(before) = before {
        args.push(before);
    };
    Box::new(Expr::Call(CallExpr {
        span: DUMMY_SP,
        ctxt: SyntaxContext::empty(),
        callee: Callee::Expr(name_as_expr(generate_insert())),
        args,
        type_args: None,
    }))
}

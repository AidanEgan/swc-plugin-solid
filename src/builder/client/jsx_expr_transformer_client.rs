// Kind of need to implement a scope manager here to catch case of vars declared within a JSX expression block
// It's probably a fairly niche use case so I'm not doing it now.
use swc_core::{
    common::util::take::Take,
    ecma::{
        ast::{BinaryOp, BlockStmtOrExpr, Expr},
        visit::{VisitMut, VisitMutWith},
    },
};

use crate::{
    builder::client::{
        builder_helpers::{memoize_bin_cond_expr, wrap_in_empty_arrow},
        jsx_expr_builder_client::{build_js_from_client_jsx, standard_build_res_wrappings},
        jsx_parser_client::ClientJsxElementVisitor,
    },
    transform::parent_visitor::ParentVisitor,
};

#[derive(Debug)]
pub struct ClientJsxExprTransformer<'a, T: ParentVisitor> {
    parent_visitor: &'a mut T,
    transform_call_exprs: bool,
    memo_bin_and_cond: bool,
    is_custom: bool,
    pub should_wrap_in_effect: bool,
    pub should_getter: bool,
}

impl<'a, T: ParentVisitor> ClientJsxExprTransformer<'a, T> {
    pub fn new(
        parent_visitor: &'a mut T,
        transform_call_exprs: bool,
        memo_bin_and_cond: bool,
        is_custom: bool,
    ) -> Self {
        Self {
            parent_visitor,
            transform_call_exprs,
            memo_bin_and_cond,
            is_custom,
            should_wrap_in_effect: false,
            should_getter: false,
        }
    }

    pub fn visit_and_wrap_outer_expr(&mut self, node: &mut Box<Expr>, wrap_member_exprs: bool) {
        node.visit_mut_with(self);
        if let Some(call_expr) = node.as_mut_call() {
            // Optimize by just calling with callee
            if call_expr.args.is_empty() && self.transform_call_exprs {
                if let Some(expr) = call_expr.callee.as_mut_expr() {
                    *node = remove_expr_from_tree(expr);
                }
            } else {
                let tmp = std::mem::take(node);
                *node = Box::new(Expr::Arrow(wrap_in_empty_arrow(
                    BlockStmtOrExpr::Expr(tmp).into(),
                )));
            }
        } else if wrap_member_exprs && node.is_member() {
            let tmp = std::mem::take(node);
            *node = Box::new(Expr::Arrow(wrap_in_empty_arrow(
                BlockStmtOrExpr::Expr(tmp).into(),
            )));
        }
    }

    pub fn visit_custom_component(&mut self, node: &mut Box<Expr>) {
        node.visit_mut_with(self);
        if node.is_call() || node.is_fn_expr() || node.is_member() {
            self.should_getter = true;
        }
    }
}

fn remove_expr_from_tree(e: &mut Box<Expr>) -> Box<Expr> {
    std::mem::take(e)
}

impl<'a, T: ParentVisitor> VisitMut for ClientJsxExprTransformer<'a, T> {
    // Intentionally do nothing
    fn visit_mut_expr(&mut self, node: &mut swc_core::ecma::ast::Expr) {
        node.visit_mut_children_with(self);
        if !self.should_wrap_in_effect && node.is_call()
            || node.is_arrow()
            || node.is_fn_expr()
            || node.is_member()
        // Checked in visitor => || (node.is_bin() && node.as_bin().unwrap().op == BinaryOp::In)
        {
            self.should_wrap_in_effect = true;
        }
        if node.is_jsx_element() || node.is_jsx_fragment() {
            let mut visitor = ClientJsxElementVisitor::new();
            node.visit_mut_with(&mut visitor);
            *node = *standard_build_res_wrappings(build_js_from_client_jsx(
                visitor,
                self.parent_visitor,
            ));
        }
    }

    // "&&" expressions within JSX need to be memo-ized
    fn visit_mut_bin_expr(&mut self, node: &mut swc_core::ecma::ast::BinExpr) {
        if node.op == BinaryOp::In {
            self.should_getter = true;
            self.should_wrap_in_effect = true;
        }
        //TODO - needs to check if need to _$memo it -> for el's with no parent
        if self.memo_bin_and_cond
            && (node.left.is_call() || node.left.is_member())
            && (node.right.is_jsx_element() || node.right.is_jsx_fragment())
        {
            self.should_wrap_in_effect = true;
            let mut dummy = Box::new(Expr::dummy());
            std::mem::swap(&mut node.left, &mut dummy);
            *node.left = *memoize_bin_cond_expr(dummy);
        }
        node.visit_mut_children_with(self);
    }

    // Ternary exprssions within jsx need to be memo-ized
    fn visit_mut_cond_expr(&mut self, node: &mut swc_core::ecma::ast::CondExpr) {
        //TODO - needs to check if need to _$memo it -> for el's with no parent
        if self.memo_bin_and_cond
            && (node.test.is_call() || node.test.is_member())
            && (node.alt.is_jsx_element()
                || node.alt.is_jsx_fragment()
                || node.cons.is_jsx_element()
                || node.cons.is_jsx_fragment())
        {
            self.should_wrap_in_effect = true;
            let mut dummy = Box::new(Expr::dummy());
            std::mem::swap(&mut node.test, &mut dummy);
            *node.test = *memoize_bin_cond_expr(dummy);
        }
        node.visit_mut_children_with(self);
    }
}

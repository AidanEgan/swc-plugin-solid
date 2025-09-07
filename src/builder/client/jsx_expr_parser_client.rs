use swc_core::{
    common::{SyntaxContext, DUMMY_SP},
    ecma::{
        ast::{ArrowExpr, BlockStmtOrExpr, Callee, Expr, Lit},
        visit::{Visit, VisitWith},
    },
};

use crate::builder::parser_types::JsxTemplateKind;

#[derive(Debug, Clone)]
pub enum TransformedExprRes {
    NewExpr(Expr),
    NewTmpl(JsxTemplateKind),
}

#[derive(Debug, Clone, Default)]
pub struct ClientJsxExprParser {
    pub result: Option<TransformedExprRes>,
    pub needs_revisit: bool,
}

impl ClientJsxExprParser {
    pub fn new() -> Self {
        Self {
            result: None,
            needs_revisit: false,
        }
    }
}

fn wrap_expr_with_arrow(old_expr: Box<Expr>) -> Expr {
    let new_expr = ArrowExpr {
        // New nodes got no span
        span: DUMMY_SP,
        // The syntax context. For new nodes, this should be empty.
        ctxt: SyntaxContext::empty(),
        params: Vec::with_capacity(0),
        body: Box::new(BlockStmtOrExpr::Expr(old_expr)),
        is_async: false,
        is_generator: false,
        type_params: None,
        return_type: None,
    };
    Expr::Arrow(new_expr)
}

impl Visit for ClientJsxExprParser {
    // Intentionally do nothing
    fn visit_jsx_empty_expr(&mut self, _node: &swc_core::ecma::ast::JSXEmptyExpr) {
        /* Intentionally do nothing here */
    }

    // General visitor for all expressions
    fn visit_expr(&mut self, node: &swc_core::ecma::ast::Expr) {
        match node {
            Expr::Lit(lit_exp) => {
                lit_exp.visit_with(self);
                if self.result.is_none() {
                    self.result = Some(TransformedExprRes::NewExpr(node.clone()));
                }
            }
            Expr::Call(call_expr) => {
                call_expr.visit_with(self);
            }
            Expr::Bin(bin_expr) => {
                bin_expr.visit_with(self);
            }
            Expr::Cond(cond_expr) => {
                cond_expr.visit_with(self);
            }
            _ => { /* Will just do the generic expr transform */ }
        };
        if self.result.is_none() {
            self.result = Some(TransformedExprRes::NewExpr(wrap_expr_with_arrow(Box::new(
                node.clone(),
            ))))
        }
    }

    // Specific implementations
    fn visit_lit(&mut self, node: &swc_core::ecma::ast::Lit) {
        match node {
            Lit::Str(string_lit) => {
                self.result = Some(TransformedExprRes::NewTmpl(JsxTemplateKind::Text(
                    string_lit.value.as_str().into(),
                )));
            }
            Lit::Num(num_lit) => {
                self.result = Some(TransformedExprRes::NewTmpl(JsxTemplateKind::Text(
                    num_lit.value.to_string(),
                )));
            }
            _ => { /* Handled above */ }
        }
    }

    fn visit_call_expr(&mut self, node: &swc_core::ecma::ast::CallExpr) {
        // Optimize by just calling with callee
        if node.args.is_empty() {
            match &node.callee {
                Callee::Expr(expr) => {
                    // Revist all expressions that aren't just a simple ident expr
                    self.needs_revisit = self.needs_revisit || !expr.is_ident();
                    self.result = Some(TransformedExprRes::NewExpr(*expr.clone()));
                }
                _ => { /* Will just do the generic expr transform */ }
            }
        }
    }

    // "&&" expressions within JSX need to be memo-ized
    fn visit_bin_expr(&mut self, node: &swc_core::ecma::ast::BinExpr) {
        //TODO - needs to check if need to _$memo it
    }

    // Ternary exprssions within jsx need to be memo-ized
    fn visit_cond_expr(&mut self, node: &swc_core::ecma::ast::CondExpr) {
        //TODO - needs to check if need to _$memo it
    }
}

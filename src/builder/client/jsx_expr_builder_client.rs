use super::jsx_parser_client::ClientJsxElementVisitor;
use std::collections::HashMap;

use swc_core::ecma::ast::Expr;

pub fn build_js_from_client_jsx(parsed_data: &ClientJsxElementVisitor) -> Expr /* Arrow expression  */
{
    todo!("Build an arrow fn expr from jsx data. NB: This expr will need to be wrapped in a paren \
           expr. and then invoke as an iife through a call expr, but that will be done by the caller");
}

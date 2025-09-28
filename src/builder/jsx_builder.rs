use swc_core::ecma::{
    ast::{Expr, JSXElement, JSXFragment},
    visit::VisitMutWith,
};

use crate::builder::client::{
    jsx_expr_builder_client::{build_js_from_client_jsx, standard_build_res_wrappings},
    jsx_parser_client::ClientJsxElementVisitor,
};

use crate::transform::parent_visitor::ParentVisitor;

// Might delete later, could be useful for wrapping data
pub enum ParsedJsxData {
    Client(ClientJsxElementVisitor),
    Server,
    Universal,
    Hydration,
}

pub trait JsxBuilder {
    fn visit_and_build_from_jsx<T: ParentVisitor>(&mut self, parent_visitor: &mut T) -> Box<Expr>;
}

impl JsxBuilder for JSXElement {
    fn visit_and_build_from_jsx<T: ParentVisitor>(&mut self, parent_visitor: &mut T) -> Box<Expr> {
        let parsed = parse_jsx(parent_visitor, self);
        build_from_jsx(parsed, parent_visitor)
    }
}

impl JsxBuilder for JSXFragment {
    fn visit_and_build_from_jsx<T: ParentVisitor>(&mut self, parent_visitor: &mut T) -> Box<Expr> {
        let parsed = parse_jsx(parent_visitor, self);
        build_from_jsx(parsed, parent_visitor)
    }
}

fn parse_jsx<T: ParentVisitor, K: VisitMutWith<ClientJsxElementVisitor>>(
    parent_visitor: &T,
    el: &mut K,
) -> ParsedJsxData {
    match parent_visitor.get_generate() {
        "ssr" => {
            todo!("Do SSR (later :))");
        }
        "universal" => {
            todo!("Do universal (later :))");
        }
        // Client
        _ => {
            let mut visitor = ClientJsxElementVisitor::new();
            el.visit_mut_with(&mut visitor);
            ParsedJsxData::Client(visitor)
        }
    }
}

// Can be call expr or array expr (possibly more?)
fn build_from_jsx<T>(data: ParsedJsxData, parent_visitor: &mut T) -> Box<Expr>
where
    T: ParentVisitor,
{
    let res = match data {
        ParsedJsxData::Server => {
            todo!("Do SSR (later :))");
        }
        ParsedJsxData::Universal => {
            todo!("Do universal (later :))");
        }
        ParsedJsxData::Hydration => {
            todo!("Do hydration (later :))");
        }
        ParsedJsxData::Client(parsed_data) => {
            // Attach parent stuff to parent visitor
            build_js_from_client_jsx(parsed_data, parent_visitor)
        }
    };
    standard_build_res_wrappings(res)
}

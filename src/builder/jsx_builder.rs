use swc_core::ecma::{
    ast::{Expr, JSXElement, JSXFragment},
    visit::{Visit, VisitWith},
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
    fn visit_and_build_from_jsx<T: ParentVisitor>(
        &self,
        parent_visitor: &mut T,
    ) -> (Box<Expr>, bool);
}

impl JsxBuilder for JSXElement {
    fn visit_and_build_from_jsx<T: ParentVisitor>(
        &self,
        parent_visitor: &mut T,
    ) -> (Box<Expr>, bool) {
        let parsed = parse_jsx(parent_visitor, |v| {
            self.visit_with(v);
        });
        build_from_jsx(parsed, parent_visitor)
    }
}

impl JsxBuilder for JSXFragment {
    fn visit_and_build_from_jsx<T: ParentVisitor>(
        &self,
        parent_visitor: &mut T,
    ) -> (Box<Expr>, bool) {
        let parsed = parse_jsx(parent_visitor, |v| {
            self.visit_with(v);
        });
        build_from_jsx(parsed, parent_visitor)
    }
}

fn parse_jsx<T: ParentVisitor, F: Fn(&mut dyn Visit)>(
    parent_visitor: &T,
    on_visit: F,
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
            on_visit(&mut visitor);
            ParsedJsxData::Client(visitor)
        }
    }
}

// Can be call expr or array expr (possibly more?)
fn build_from_jsx<T>(data: ParsedJsxData, parent_visitor: &mut T) -> (Box<Expr>, bool)
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
            let needs_revisit = parsed_data.needs_revisit;
            let built = build_js_from_client_jsx(parsed_data, parent_visitor);
            (built, needs_revisit)
        }
    };
    (standard_build_res_wrappings(res.0), res.1)
}

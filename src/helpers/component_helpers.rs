use swc_core::ecma::{
    ast::{Expr, JSXElementName},
    transforms::base::ext::ExprRefExt,
};

use crate::transform::parent_visitor::ParentVisitor;

// TODO: This creates a lot of redundant strings
// could be optimized to not recrate all those strings?
// JSWord???

pub fn get_component_name(name: &JSXElementName) -> String {
    match name {
        JSXElementName::Ident(n) => n.sym.as_str().into(),
        JSXElementName::JSXMemberExpr(n) => n.prop.sym.as_str().into(),
        JSXElementName::JSXNamespacedName(n) => {
            format!("{0}:{1}", n.ns.sym.as_str(), n.name.sym.as_str())
        }
    }
}

// Check if this is a dom component or JSX component
// Uses the rule of thumb that JSX components start
// with a capital letter
pub fn is_solid_component(nm: &str) -> bool {
    if !nm.is_empty() {
        nm.as_bytes()[0].is_ascii_uppercase()
    } else {
        false
    }
}

// User can provide a set of built-in components that are treated as
// reserved names by the library. Generaly these components will need
// to have a corresponding import statement.
pub fn is_built_in<T: ParentVisitor>(name: &str, parent_vis: &T) -> bool {
    parent_vis.get_built_ins().contains(name)
}

// Check if this is a custom element
// 'has is' refers to an 'is' attribute.
// It's more optimal to check for that when parsing
// the elements attrs instead of doing that again here
pub fn is_ce(name: &str, has_is: bool) -> bool {
    has_is || name.contains("-")
}

// Checks if an element in undefined
pub fn is_undefined(expr: &Box<Expr>) -> bool {
    if let Some(ident) = expr.as_ident() {
        ident.sym.as_str() == "undefined"
    } else {
        false
    }
}

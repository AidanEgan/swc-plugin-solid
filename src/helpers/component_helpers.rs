use swc_core::ecma::ast::JSXElementName;

use crate::transform::parent_visitor::ParentVisitor;

// TODO: This creates a lot of redundant strings
// could be optimized to not recrate all those strings?
// JSWord???

// Check if this is a dom component or JSX component
// Uses the rule of thumb that JSX components start
// with a capital letter
pub fn is_solid_component(name: &JSXElementName) -> (bool, String) {
    match name {
        JSXElementName::Ident(n) => {
            let nm: String = n.sym.as_str().into();
            if !nm.is_empty() {
                (n.sym.as_bytes()[0].is_ascii_uppercase(), nm)
            } else {
                (false, nm)
            }
        }
        JSXElementName::JSXMemberExpr(n) => (true, n.prop.sym.as_str().into()),
        JSXElementName::JSXNamespacedName(n) => (
            false,
            format!("{0}:{1}", n.ns.sym.as_str(), n.name.sym.as_str()),
        ),
    }
}

// User can provide a set of built-in components that are treated as
// reserved names by the library. Generaly these components will need
// to have a corresponding import statement.
pub fn is_built_in<T: ParentVisitor>(name: &str, parent_vis: &T) -> bool {
    parent_vis.get_built_ins().contains(name)
}

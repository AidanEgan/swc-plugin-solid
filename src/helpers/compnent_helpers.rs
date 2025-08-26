use crate::transform::parent_visitor::ParentVisitor;

// Check if this is a dom component or JSX component
// Uses the rule of thumb that JSX components start
// with a capital letter
pub fn is_solid_component(name: &str) -> bool {
    if name.len() > 0 {
        name.as_bytes()[0].is_ascii_uppercase()
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

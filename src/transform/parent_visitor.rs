use std::borrow::Cow;
use std::collections::HashSet;

use swc_core::atoms::Atom;

use crate::builder::jsx_builder::ParsedJsxData;
use crate::transform::scope_manager::TrackedVariable;

// Holds result from builder
pub trait ParentVisitor {
    fn element_count(&mut self) -> &mut usize;
    fn ref_count(&mut self) -> &mut usize;
    fn v_count(&mut self) -> &mut usize;
    fn attach_data(&mut self, data: ParsedJsxData);
    fn get_built_ins(&self) -> &HashSet<String>;
    fn get_generate(&self) -> &str;
    fn get_is_hydratable(&self) -> bool;
    fn get_template_id(&mut self, template: &str) -> usize;
    fn register_event(&mut self, event: Cow<str>);
    fn add_import(&mut self, import_name: Cow<str>);
    fn get_var_if_in_scope(&self, var: &Atom) -> Option<&TrackedVariable>;
    fn add_event(&mut self, event_name: Cow<str>);
}

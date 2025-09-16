use std::borrow::Cow;
use std::collections::HashSet;

use crate::builder::jsx_builder::ParsedJsxData;

// Holds result from builder
pub trait ParentVisitor {
    fn element_count(&mut self) -> &mut usize;
    fn attach_data(&mut self, data: ParsedJsxData);
    fn get_built_ins(&self) -> &HashSet<String>;
    fn get_generate(&self) -> &str;
    fn get_is_hydratable(&self) -> bool;
    fn get_template_id(&mut self, template: &str) -> usize;
    fn register_event(&mut self, event: Cow<str>);
    fn add_import(&mut self, import_name: Cow<str>);
}

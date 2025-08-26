use std::collections::HashSet;

use crate::builder::jsx_builder::ParsedJsxData;

// Holds result from builder
pub trait ParentVisitor {
    fn attach_data(&mut self, data: ParsedJsxData);
    fn get_built_ins(&self) -> &HashSet<String>;
    fn get_generate(&self) -> &str;
    fn get_is_hydratable(&self) -> bool;
}

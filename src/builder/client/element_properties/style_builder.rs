use crate::{
    builder::client::element_properties::{ElementPropertiesBuilder, PossibleEffectStatement},
    transform::parent_visitor::ParentVisitor,
};

impl<'a, T: ParentVisitor> ElementPropertiesBuilder<'a, T> {
    pub fn individual_style_builder(
        &mut self,
        element_count: usize,
        data: PossibleEffectStatement,
    ) {
        todo!()
    }
}

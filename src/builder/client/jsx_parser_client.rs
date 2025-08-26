use std::collections::HashMap;
use swc_core::ecma::ast::{JSXElement, JSXFragment};
use swc_core::ecma::visit::Visit;

// Logic for elements and fragments will be similar - use this for both

pub struct ClientJsxElementVisitor {
    pub templates: HashMap<String, Vec<String>>,
    pub needs_revisit: bool,
}

impl Visit for ClientJsxElementVisitor {
    fn visit_jsx_element(&mut self, node: &JSXElement) {
        todo!("The easy part :)");
    }
    fn visit_jsx_fragment(&mut self, node: &JSXFragment) {
        todo!("Basically the same");
    }
}

impl ClientJsxElementVisitor {
    pub fn new() -> ClientJsxElementVisitor {
        ClientJsxElementVisitor {
            templates: HashMap::new(),
            needs_revisit: false,
        }
    }
}

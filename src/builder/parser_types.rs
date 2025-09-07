use std::collections::HashMap;

use swc_core::ecma::ast::Expr;

/*
pub struct Attrs {
    name: &str,
}
*/
#[derive(Debug, Clone, Default)]
pub struct JsxOpeningMetadata {
    pub attrs: HashMap<String, String>,
    pub styles: HashMap<String, String>,
    pub events: HashMap<String, Expr>, // Might just have to clone expr :(
    pub value: String,
}

impl JsxOpeningMetadata {
    pub fn new(value: String) -> Self {
        Self {
            attrs: HashMap::new(),
            styles: HashMap::new(),
            events: HashMap::new(),
            value,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct JsxCustomComponentMetadata<T: Clone> {
    pub value: String,
    pub props: HashMap<String, Expr>, // Might just have to clone expr :(
    pub children: Vec<T>,
    pub needs_revisit: bool, // Might needs swc to re-evaluate expressions in props
    pub is_builtin: bool,    // Users can provide list of builtin components which we need to import
}

// Subset of custom component metadata
#[derive(Debug, Clone, Default)]
pub struct JsxFragmentMetadata<T: Clone> {
    pub children: Vec<T>,
    pub needs_revisit: bool, // Might needs swc to re-evaluate expressions in props
}

#[derive(Debug, Clone)]
pub enum JsxTemplateKind {
    Opening(JsxOpeningMetadata), // Opening element data
    Closing(String),             // Closing element type
    Text(String),                // Holds text value
    Placeholder(usize),          // Holds id for placeholder (expr or custom component)
}

#[derive(Debug, Clone)]
pub enum PossiblePlaceholders<T: Clone> {
    Component(JsxCustomComponentMetadata<T>),
    Expression(Expr),
    Fragment(JsxFragmentMetadata<T>), // Each child will be visited separately
}

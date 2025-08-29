use std::collections::HashMap;

use swc_core::ecma::ast::Expr;

#[derive(Debug, Clone, Default)]
pub struct JsxOpeningMetadata {
    attrs: HashMap<String, String>,
    styles: HashMap<String, String>,
    events: HashMap<String, Expr>, // Might just have to clone expr :(
    value: String,
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
    value: String,
    props: HashMap<String, Expr>, // Might just have to clone expr :(
    children: Vec<T>,
    needs_revisit: bool, // Might needs swc to re-evaluate expressions in props
}

// Subset of custom component metadata
#[derive(Debug, Clone, Default)]
pub struct JsxFragmentMetadata<T: Clone> {
    children: Vec<T>,
    needs_revisit: bool, // Might needs swc to re-evaluate expressions in props
}

#[derive(Debug, Clone)]
pub enum JsxTemplateKind<T: Clone> {
    Opening(JsxOpeningMetadata),      // Opening element data
    Closing(String),                  // Closing element type
    Text(String),                     // Holds text value
    Placeholder(usize),               // Holds id for placeholder (expr or custom component)
    Fragment(JsxFragmentMetadata<T>), // Each child will be visited separately
}

#[derive(Debug, Clone)]
pub enum PossiblePlaceholders<T: Clone> {
    Component(JsxCustomComponentMetadata<T>),
    Expression(Expr),
}

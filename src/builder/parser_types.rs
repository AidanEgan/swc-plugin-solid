use swc_core::{
    atoms::Atom,
    ecma::ast::{Expr, JSXAttr, JSXAttrOrSpread, JSXAttrValue},
};

/*
pub struct Attrs {
    name: &str,
}
*/
#[derive(Debug, Clone, Default)]
pub struct JsxOpeningMetadata {
    // Spread elements have no 'key'
    pub attrs: Vec<(Option<Atom>, JSXAttrOrSpread)>,
    pub value: String,
    pub has_spread: bool,
}

impl JsxOpeningMetadata {
    pub fn new(value: String) -> Self {
        Self {
            attrs: Vec::new(),
            value,
            has_spread: false,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct JsxCustomComponentMetadata<T: Clone> {
    pub value: String,
    pub props: Vec<(Option<Atom>, JSXAttrOrSpread)>, // Might just have to clone expr :(
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
    Expression(Box<Expr>),
    Fragment(JsxFragmentMetadata<T>), // Each child will be visited separately
}

use super::super::parser_types::{JsxOpeningMetadata, JsxTemplateKind, PossiblePlaceholders};
use crate::builder::client::builder_helpers::own_box_expr;
use crate::builder::parser_types::{JsxCustomComponentMetadata, JsxFragmentMetadata};
use crate::helpers::component_helpers::{get_component_name, is_solid_component};
use crate::helpers::opening_element_helpers::parse_attrs;
use swc_core::ecma::ast::{Expr, JSXElement, JSXExpr, JSXFragment, Lit};
use swc_core::ecma::visit::{VisitMut, VisitMutWith};

// Logic for elements and fragments will be similar - use this for both

#[derive(Debug, Clone, Default)]
pub struct ClientJsxElementVisitor {
    placeholder_count: usize,
    pub template: Vec<JsxTemplateKind>,
    pub needs_revisit: bool,
    // Option used so placeholders can be moved out of vec more easily
    pub placeholders: Vec<Option<PossiblePlaceholders<ClientJsxElementVisitor>>>,
}

impl ClientJsxElementVisitor {
    fn add_expr_placeholder(&mut self, data: Box<Expr>) {
        self.template
            .push(JsxTemplateKind::Placeholder(self.placeholder_count));
        self.placeholders
            .push(Some(PossiblePlaceholders::Expression(data)));
        self.placeholder_count += 1;
    }
}

impl VisitMut for ClientJsxElementVisitor {
    fn visit_mut_jsx_closing_element(&mut self, node: &mut swc_core::ecma::ast::JSXClosingElement) {
        // Does it matter here??? I
        let name = get_component_name(&node.name);
        self.template.push(JsxTemplateKind::Closing(name));
    }
    fn visit_mut_jsx_fragment(&mut self, node: &mut JSXFragment) {
        let mut needs_revisit = false;
        /*
         * Each child is visited separately w/ its own visitor
         */
        let children = node
            .children
            .drain(..)
            .map(|mut child| {
                let mut new_visitor = ClientJsxElementVisitor::new();
                child.visit_mut_with(&mut new_visitor);
                needs_revisit = needs_revisit || new_visitor.needs_revisit;
                new_visitor
            })
            .collect();
        self.template
            .push(JsxTemplateKind::Placeholder(self.placeholder_count));
        self.placeholders
            .push(Some(PossiblePlaceholders::Fragment(JsxFragmentMetadata {
                children,
                needs_revisit,
            })));
        self.placeholder_count += 1;
    }

    fn visit_mut_jsx_element(&mut self, node: &mut JSXElement) {
        let (is_custom_component, name) = is_solid_component(&node.opening.name);
        if is_custom_component {
            let custom_component = JsxCustomComponentMetadata::<ClientJsxElementVisitor> {
                value: name,
                props: parse_attrs(&mut node.opening).0,
                children: node
                    .children
                    .drain(..)
                    .map(|mut child| {
                        let mut new_visitor = ClientJsxElementVisitor::new();
                        child.visit_mut_with(&mut new_visitor);
                        self.needs_revisit = self.needs_revisit || new_visitor.needs_revisit;
                        new_visitor
                    })
                    .collect(),
                needs_revisit: true,
                is_builtin: false,
            };
            // Index is off by one
            self.template
                .push(JsxTemplateKind::Placeholder(self.placeholder_count));
            self.placeholders
                .push(Some(PossiblePlaceholders::Component(custom_component)));
            self.placeholder_count += 1;
        } else {
            node.opening.visit_mut_with(self);
            node.children.visit_mut_with(self);
        }
    }

    fn visit_mut_jsx_opening_element(&mut self, node: &mut swc_core::ecma::ast::JSXOpeningElement) {
        //Custom components do not visit here!!!!
        // Done here to avoid extra clone if not needed
        let name = get_component_name(&node.name);
        let close = if node.self_closing {
            Some(JsxTemplateKind::Closing(name.clone()))
        } else {
            None
        };
        let (attrs, has_spread) = parse_attrs(node);
        let opening = JsxOpeningMetadata {
            value: name,
            attrs,
            has_spread,
        };
        self.template.push(JsxTemplateKind::Opening(opening));
        if let Some(close) = close {
            self.template.push(close);
        }
    }

    /* Possible JSX Element childs (along with JSX Element/JSX Fragment) */

    fn visit_mut_jsx_expr_container(&mut self, node: &mut swc_core::ecma::ast::JSXExprContainer) {
        // These are basically just jsx text, we can treat them as such
        match &mut node.expr {
            JSXExpr::Expr(expr) => {
                if let Some(lit) = expr.as_mut_lit() {
                    match lit {
                        Lit::Str(string_lit) => {
                            self.template
                                .push(JsxTemplateKind::Text(string_lit.value.to_string()));
                            return;
                        }
                        Lit::Num(num_lit) => {
                            self.template
                                .push(JsxTemplateKind::Text(num_lit.value.to_string()));
                            return;
                        }
                        _ => { /* Pass */ }
                    }
                }

                self.add_expr_placeholder(own_box_expr(expr));
            }
            JSXExpr::JSXEmptyExpr(_) => { /* Explicitly do nothing */ }
        }
    }

    fn visit_mut_jsx_text(&mut self, node: &mut swc_core::ecma::ast::JSXText) {
        // Just get rid of newlines because they will be a problem for the template
        // POSSIBLE OPTIMIZATION: Could just 'trim' the string and omit empty JSX Text values?
        let transformed = node.value.replace(['\n', '\r'], "");
        self.template.push(JsxTemplateKind::Text(transformed));
    }

    /* Nothing to do?
    fn visit_jsx_element_child(&mut self, node: &swc_core::ecma::ast::JSXElementChild) {
        todo!("PLEASE DO")
    }
    */

    // Need to "unspread" it
    fn visit_mut_jsx_spread_child(&mut self, node: &mut swc_core::ecma::ast::JSXSpreadChild) {
        self.add_expr_placeholder(own_box_expr(&mut node.expr));
    }
    /* Not sure if there is anything special to do here
    fn visit_jsx_member_expr(&mut self, node: &swc_core::ecma::ast::JSXMemberExpr) {

    }
    */
    fn visit_mut_jsx_attr(&mut self, node: &mut swc_core::ecma::ast::JSXAttr) {}
}

impl ClientJsxElementVisitor {
    pub fn new() -> ClientJsxElementVisitor {
        ClientJsxElementVisitor {
            placeholder_count: 0,
            template: Vec::new(),
            needs_revisit: false,
            placeholders: Vec::new(),
        }
    }
}

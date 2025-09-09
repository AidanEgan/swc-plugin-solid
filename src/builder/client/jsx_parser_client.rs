use super::super::parser_types::{JsxOpeningMetadata, JsxTemplateKind, PossiblePlaceholders};
use super::jsx_expr_parser_client::ClientJsxExprParser;
use super::jsx_expr_parser_client::TransformedExprRes;
use crate::builder::parser_types::JsxFragmentMetadata;
use crate::helpers::component_helpers::is_solid_component;
use swc_core::ecma::ast::{JSXElement, JSXFragment};
use swc_core::ecma::visit::{Visit, VisitWith};

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
    fn add_data_from_expr(&mut self, data: TransformedExprRes) {
        match data {
            TransformedExprRes::NewExpr(transformed_expr) => {
                self.template
                    .push(JsxTemplateKind::Placeholder(self.placeholder_count));
                self.placeholders
                    .push(Some(PossiblePlaceholders::Expression(transformed_expr)));
                self.placeholder_count += 1;
            }
            TransformedExprRes::NewTmpl(transformed_tmpl) => {
                self.template.push(transformed_tmpl);
            }
        }
    }
}

impl Visit for ClientJsxElementVisitor {
    fn visit_jsx_closing_element(&mut self, node: &swc_core::ecma::ast::JSXClosingElement) {
        // Does it matter here??? I
        let (_is_custom_component, name) = is_solid_component(&node.name);
        self.template.push(JsxTemplateKind::Closing(name));
    }
    fn visit_jsx_fragment(&mut self, node: &JSXFragment) {
        let mut needs_revisit = false;
        /*
         * Each child is visited separately w/ its own visitor
         */
        let children = node
            .children
            .iter()
            .map(|child| {
                let mut new_visitor = ClientJsxElementVisitor::new();
                child.visit_with(&mut new_visitor);
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
    fn visit_jsx_opening_element(&mut self, node: &swc_core::ecma::ast::JSXOpeningElement) {
        let (is_custom_component, name) = is_solid_component(&node.name);
        if is_custom_component {
            todo!("Create component placeholder -- check for builtins")
        } else {
            // Done here to avoid extra clone if not needed
            let close = if node.self_closing {
                Some(JsxTemplateKind::Closing(name.clone()))
            } else {
                None
            };
            let mut opening = JsxOpeningMetadata::new(name);
            /*
            todo!("add events from tag");
            todo!("add styles from tag");
            todo!("add attributes from tag");
            */
            self.template.push(JsxTemplateKind::Opening(opening));
            if let Some(close) = close {
                self.template.push(close);
            }
        }
    }

    /* Possible JSX Element childs (along with JSX Element/JSX Fragment) */

    fn visit_jsx_expr_container(&mut self, node: &swc_core::ecma::ast::JSXExprContainer) {
        let mut expr_visitor: ClientJsxExprParser = ClientJsxExprParser::new();
        node.expr.visit_with(&mut expr_visitor);
        if let Some(res) = expr_visitor.result {
            self.add_data_from_expr(res);
        }
    }

    fn visit_jsx_text(&mut self, node: &swc_core::ecma::ast::JSXText) {
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
    fn visit_jsx_spread_child(&mut self, node: &swc_core::ecma::ast::JSXSpreadChild) {
        let mut expr_visitor: ClientJsxExprParser = ClientJsxExprParser::new();
        node.expr.visit_with(&mut expr_visitor);
        if let Some(res) = expr_visitor.result {
            self.add_data_from_expr(res);
        }
    }
    /* Not sure if there is anything special to do here
    fn visit_jsx_member_expr(&mut self, node: &swc_core::ecma::ast::JSXMemberExpr) {

    }
    */
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

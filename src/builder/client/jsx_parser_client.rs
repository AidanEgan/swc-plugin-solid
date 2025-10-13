use super::super::parser_types::{JsxOpeningMetadata, JsxTemplateKind, PossiblePlaceholders};
use crate::builder::client::builder_helpers::own_box_expr;
use crate::builder::parser_types::{JsxCustomComponentMetadata, JsxFragmentMetadata};
use crate::constants::properties::is_self_closing;
use crate::constants::svg::is_svg_element_name;
use crate::helpers::component_helpers::{get_component_name, is_ce, is_solid_component};
use crate::helpers::opening_element_helpers::parse_attrs;
use swc_core::ecma::ast::{Expr, JSXElement, JSXExpr, JSXFragment, Lit};
use swc_core::ecma::visit::{VisitMut, VisitMutWith};

// Logic for elements and fragments will be similar - use this for both

#[derive(Debug, Clone, Default)]
pub struct ClientJsxElementVisitor {
    placeholder_count: usize,
    pub template: Vec<JsxTemplateKind>,
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
        /*
         * Each child is visited separately w/ its own visitor
         */
        let children = node
            .children
            .drain(..)
            .filter_map(|mut child| {
                let mut new_visitor = ClientJsxElementVisitor::new();
                child.visit_mut_with(&mut new_visitor);
                if new_visitor.template.is_empty() && new_visitor.placeholders.is_empty() {
                    None
                } else {
                    Some(new_visitor)
                }
            })
            .collect();
        self.template
            .push(JsxTemplateKind::Placeholder(self.placeholder_count));
        self.placeholders
            .push(Some(PossiblePlaceholders::Fragment(JsxFragmentMetadata {
                children,
            })));
        self.placeholder_count += 1;
    }

    fn visit_mut_jsx_element(&mut self, node: &mut JSXElement) {
        let name = get_component_name(&node.opening.name);
        let is_custom_component = is_solid_component(&name);
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
                        new_visitor
                    })
                    .collect(),
                is_builtin: false,
            };
            // Index is off by one
            self.template
                .push(JsxTemplateKind::Placeholder(self.placeholder_count));
            self.placeholders
                .push(Some(PossiblePlaceholders::Component(custom_component)));
            self.placeholder_count += 1;
        } else {
            let implicit_self_close = is_self_closing(name.as_str());
            // HTML Tags that are always self closing will implicitly be closed
            // by the framework. We don't need to add an explicit closing tag.
            let close = if node.opening.self_closing && !implicit_self_close {
                Some(JsxTemplateKind::Closing(name.clone()))
            } else {
                None
            };
            let (attrs, has_spread, has_is) = parse_attrs(&mut node.opening);
            let is_svg = is_svg_element_name(name.as_str());
            let is_ce = is_ce(name.as_str(), has_is);
            let opening = JsxOpeningMetadata {
                has_children: !node.children.is_empty(),
                value: name,
                attrs,
                has_spread,
                is_ce,
                is_svg,
                implicit_self_close,
            };
            self.template.push(JsxTemplateKind::Opening(opening));
            if let Some(close) = close {
                self.template.push(close);
            }
            node.children.visit_mut_with(self);
            node.closing.visit_mut_with(self);
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
        /*
         * This is a bit of an open question for me. In theory, there should be no
         * rendering difference between `<div> Hello World </div>` and `<div>Hello World</div>`
         * So I could just optimize out the whitespace.
         * This differs from the existing implementation however, so there could be subtle differences.
         * In the case a user NEEDS whitespace they could do `{" "}` to guarantee it. That would
         * render a placeholder with a space as it's value. Which would add the space to the template
         */
        let trimmed = node.value.trim();
        if !trimmed.is_empty() {
            // Get rid of newlines because they will be a problem for the template
            //let transformed = node.value.replace(['\n', '\r'], "");
            self.template
                .push(JsxTemplateKind::Text(trimmed.to_string()));
        }
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

    fn visit_mut_jsx_attr(&mut self, node: &mut swc_core::ecma::ast::JSXAttr) {}
    */
}

impl ClientJsxElementVisitor {
    pub fn new() -> ClientJsxElementVisitor {
        ClientJsxElementVisitor {
            placeholder_count: 0,
            template: Vec::new(),
            placeholders: Vec::new(),
        }
    }
}

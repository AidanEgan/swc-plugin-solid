use super::super::parser_types::{JsxOpeningMetadata, JsxTemplateKind, PossiblePlaceholders};
use crate::helpers::compnent_helpers::is_solid_component;
use swc_core::common::{SyntaxContext, DUMMY_SP};
use swc_core::ecma::ast::{
    ArrowExpr, BlockStmtOrExpr, Callee, Expr, JSXElement, JSXExpr, JSXFragment, Lit,
};
use swc_core::ecma::visit::{Visit, VisitWith};

// Logic for elements and fragments will be similar - use this for both

fn wrap_expr_with_arrow(old_expr: Box<Expr>) -> Expr {
    let new_expr = ArrowExpr {
        // New nodes got no span
        span: DUMMY_SP,
        // The syntax context. For new nodes, this should be empty.
        ctxt: SyntaxContext::empty(),
        params: Vec::with_capacity(0),
        body: Box::new(BlockStmtOrExpr::Expr(old_expr)),
        is_async: false,
        is_generator: false,
        type_params: None,
        return_type: None,
    };
    Expr::Arrow(new_expr)
}

#[derive(Debug, Clone, Default)]
pub struct ClientJsxElementVisitor {
    placeholder_count: usize,
    pub template: Vec<JsxTemplateKind<ClientJsxElementVisitor>>,
    pub needs_revisit: bool,
    pub placeholders: Vec<PossiblePlaceholders<ClientJsxElementVisitor>>,
}

impl Visit for ClientJsxElementVisitor {
    fn visit_jsx_element(&mut self, node: &JSXElement) {
        node.opening.visit_with(self);
        node.visit_children_with(self);
        if node.closing.is_some() {
            // Unwrap is safe here!
            node.closing.as_ref().unwrap().visit_with(self);
        }
    }
    fn visit_jsx_fragment(&mut self, node: &JSXFragment) {
        todo!("Basically the same");
    }
    fn visit_jsx_opening_element(&mut self, node: &swc_core::ecma::ast::JSXOpeningElement) {
        let (is_custom_component, name) = is_solid_component(&node.name);
        if is_custom_component {
            todo!("Create component placeholder")
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
        let mut transformed_expr: Option<Expr> = None;
        match &node.expr {
            JSXExpr::JSXEmptyExpr(_) => {
                // We can just ignore it
                return;
            }
            JSXExpr::Expr(e) => {
                match &**e {
                    Expr::Lit(lit_exp) => match lit_exp {
                        Lit::Str(string_lit) => {
                            self.template
                                .push(JsxTemplateKind::Text(string_lit.value.as_str().into()));
                        }
                        Lit::Num(num_lit) => {
                            self.template
                                .push(JsxTemplateKind::Text(num_lit.value.to_string()));
                        }
                        _ => {
                            transformed_expr = Some(*e.clone());
                        }
                    },
                    Expr::Call(call_expr) => {
                        // Optimize by just calling wiht callee
                        if call_expr.args.len() == 0 {
                            match &call_expr.callee {
                                Callee::Expr(expr) => {
                                    // Revist all expressions that aren't just a simple ident expr
                                    self.needs_revisit = self.needs_revisit || !expr.is_ident();
                                    transformed_expr = Some(*expr.clone());
                                }
                                _ => { /* Will just do the generic expr transform */ }
                            }
                        }
                    }
                    _ => { /* Will just do the generic expr transform */ }
                };
                if transformed_expr.is_none() {
                    transformed_expr = Some(wrap_expr_with_arrow(e.clone()))
                }
            }
        }
        if let Some(transformed_expr) = transformed_expr {
            self.template
                .push(JsxTemplateKind::Placeholder(self.placeholder_count));
            self.placeholders
                .push(PossiblePlaceholders::Expression(transformed_expr));
            self.placeholder_count += 1;
        }
    }

    fn visit_jsx_element_child(&mut self, node: &swc_core::ecma::ast::JSXElementChild) {
        todo!("PLEASE DO")
    }

    fn visit_jsx_spread_child(&mut self, node: &swc_core::ecma::ast::JSXSpreadChild) {
        todo!("DO")
    }
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

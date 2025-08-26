use super::{create_new_expr, create_new_expr_option, CreateNewExprError};
use crate::builder::jsx_builder::ParsedJsxData;
use crate::transform::parent_visitor::ParentVisitor;
use crate::{config::PluginArgs, helpers::should_skip::should_skip};
use std::collections::{HashMap, HashSet};
use swc_core::common::Spanned;
use swc_core::ecma::ast::{BlockStmtOrExpr, ExprOrSpread};
use swc_core::ecma::visit::VisitMutWith;
use swc_core::{
    common::{comments::Comments, SourceMapper},
    ecma::{
        ast::{Expr, Id},
        visit::VisitMut,
    },
};

pub struct SolidJsVisitor<C: Clone + Comments, S: SourceMapper> {
    // We may not need Arc in the plugin context - this is only to preserve isomorphic interface
    // between plugin & custom transform pass.
    source_map: std::sync::Arc<S>,
    comments: C,
    options: PluginArgs,
    filename: String,
    meta: HashMap<String, String>,
    component_names: HashSet<String>,
    function_names: HashSet<String>,
    // Variable tracking for React Compiler optimizations
    variable_bindings: HashMap<Id, Expr>,
}

impl<C: Clone + Comments, S: SourceMapper> ParentVisitor for SolidJsVisitor<C, S> {
    fn attach_data(&mut self, data: ParsedJsxData) {
        match data {
            ParsedJsxData::Client(_) => {
                todo!("Attach relvant data to Solid Visitor");
            }
            _ => {
                todo!("Support use cases other than client!");
            }
        }
    }
    fn get_built_ins(&self) -> &std::collections::HashSet<String> {
        &self.options.built_ins
    }
    fn get_generate(&self) -> &str {
        &self.options.generate
    }
    fn get_is_hydratable(&self) -> bool {
        self.options.hydratable
    }
}

impl<C: Clone + Comments, S: SourceMapper> SolidJsVisitor<C, S> {
    pub fn new(
        source_map: std::sync::Arc<S>,
        comments: C,
        plugin_options: PluginArgs,
        filename: &str,
    ) -> Self {
        let function_names: HashSet<String> = Default::default();
        let component_names: HashSet<String> = Default::default();

        SolidJsVisitor {
            source_map,
            comments,
            options: plugin_options,
            filename: filename.to_string(),
            meta: Default::default(),
            component_names,
            function_names,
            variable_bindings: Default::default(),
        }
    }
}

impl<C: Clone + Comments, S: SourceMapper> VisitMut for SolidJsVisitor<C, S> {
    // Serves as main entry + exit point. This is where pre-processing will happen
    // as well as where post-processing will happen
    fn visit_mut_program(&mut self, node: &mut swc_core::ecma::ast::Program) {
        // Pre-process
        if let Some(module) = node.as_module() {
            if should_skip(
                &self.options.require_import_source,
                &self.comments,
                module.span_lo(),
            ) {
                return;
            }
        }

        node.visit_mut_children_with(self);

        // Post-process
        // See: https://github.com/ryansolid/dom-expressions/blob/main/packages/babel-plugin-jsx-dom-expressions/src/shared/postprocess.js
    }

    fn visit_mut_arrow_expr(&mut self, node: &mut swc_core::ecma::ast::ArrowExpr) {
        match create_new_expr_option(node.body.as_expr(), self) {
            Ok((new_expr, needs_more_traverse)) => {
                node.body = Box::new(BlockStmtOrExpr::Expr(new_expr));
                if needs_more_traverse {
                    node.visit_mut_children_with(self);
                }
            }
            Err(CreateNewExprError::NoChangeNeeded) => {
                // Any potential JSX in a block statement will be encapsulated
                // by some other statement, likely a return stmt. No need to handle here.
                node.visit_mut_children_with(self);
            }
            Err(CreateNewExprError::ExprNotFound) => { /* Nothing to do here! */ }
        }
    }

    // Ternary expr.
    fn visit_mut_cond_expr(&mut self, node: &mut swc_core::ecma::ast::CondExpr) {
        let mut visit_children = false;
        match create_new_expr(&node.cons, self) {
            Ok((new_expr, needs_more_traverse)) => {
                node.cons = new_expr;
                if needs_more_traverse {
                    node.visit_mut_children_with(self);
                }
            }
            Err(CreateNewExprError::NoChangeNeeded) => {
                // Any potential JSX in a block statement will be encapsulated
                // by some other statement, likely a return stmt. No need to handle here.
                visit_children = true;
            }
            Err(CreateNewExprError::ExprNotFound) => { /* Nothing to do here! */ }
        }
        match create_new_expr(&node.alt, self) {
            Ok((new_expr, needs_more_traverse)) => {
                node.alt = new_expr;
                if needs_more_traverse {
                    node.visit_mut_children_with(self);
                }
            }
            Err(CreateNewExprError::NoChangeNeeded) => {
                // Any potential JSX in a block statement will be encapsulated
                // by some other statement, likely a return stmt. No need to handle here.
                visit_children = true;
            }
            Err(CreateNewExprError::ExprNotFound) => { /* Nothing to do here! */ }
        }
        if visit_children {
            node.visit_mut_children_with(self);
        }
    }

    fn visit_mut_return_stmt(&mut self, node: &mut swc_core::ecma::ast::ReturnStmt) {
        // Check is 'JSX' return statement -> exit if not
        match create_new_expr_option(node.arg.as_ref(), self) {
            Ok((new_expr, needs_more_traverse)) => {
                node.arg = Some(new_expr);
                if needs_more_traverse {
                    node.visit_mut_children_with(self);
                }
            }
            Err(CreateNewExprError::NoChangeNeeded) => {
                node.visit_mut_children_with(self);
            }
            Err(CreateNewExprError::ExprNotFound) => { /* Nothing to do here! */ }
        }
    }

    fn visit_mut_var_declarator(&mut self, node: &mut swc_core::ecma::ast::VarDeclarator) {
        // Check + transform potential JSX
        match create_new_expr_option(node.init.as_ref(), self) {
            Ok((new_expr, needs_more_traverse)) => {
                node.init = Some(new_expr);
                if needs_more_traverse {
                    node.visit_mut_children_with(self);
                }
            }
            Err(CreateNewExprError::NoChangeNeeded) => {
                node.visit_mut_children_with(self);
            }
            Err(CreateNewExprError::ExprNotFound) => { /* Nothing to do here! */ }
        }
    }

    fn visit_mut_call_expr(&mut self, node: &mut swc_core::ecma::ast::CallExpr) {
        // Replace any JSX in function args
        // This is particularly important as this parser recursively transforms jsx
        //
        let mut transformed_vec: Vec<(usize, ExprOrSpread)> = Vec::new();
        let mut any_recurse = false;
        // Collect all transformed expressions
        for (i, v) in node.args.iter().enumerate() {
            let new_expr_res = create_new_expr(&v.expr, self);
            if let Ok((transformed, needs_recurse)) = new_expr_res {
                any_recurse = any_recurse || needs_recurse;
                transformed_vec.push((
                    i,
                    ExprOrSpread {
                        spread: None,
                        expr: transformed,
                    },
                ));
            } else if let Err(CreateNewExprError::NoChangeNeeded) = new_expr_res {
                any_recurse = true;
            }
        }
        // Swap out old expressions for replaced ones
        for (replace_index, replace_expr) in transformed_vec {
            // Should always works
            if let Some(expr) = node.args.get_mut(replace_index) {
                *expr = replace_expr;
            }
        }
        if any_recurse {
            node.visit_mut_children_with(self);
        }
    }
}

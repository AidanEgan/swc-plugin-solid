use super::CreateNewExprError;
use crate::builder::jsx_builder::ParsedJsxData;
use crate::transform::create_new_expr_mut;
use crate::transform::parent_visitor::ParentVisitor;
use crate::transform::postprocess::{add_events, add_imports, create_template_declarations};
use crate::transform::scope_manager::{ScopeManager, TrackedVariable};
use crate::{config::PluginArgs, helpers::should_skip::should_skip};
use std::borrow::Cow;
use std::collections::{BTreeSet, HashMap};
use swc_core::common::comments::Comment;
use swc_core::common::source_map::SmallPos;
use swc_core::common::{BytePos, Spanned};
use swc_core::ecma::ast::{
    BlockStmt, Decl, FnDecl, Function, ModuleItem, Program, Stmt, VarDecl, VarDeclKind,
};
use swc_core::ecma::visit::VisitMutWith;
use swc_core::{
    common::{comments::Comments, SourceMapper},
    ecma::visit::VisitMut,
};

#[derive(Debug, Clone, Copy)]
pub struct TemplateMetaData {
    pub is_ce: bool,
    pub is_svg: bool,
    pub is_import_node: bool,
}

pub struct SolidJsVisitor<C: Clone + Comments, S: SourceMapper> {
    // We may not need Arc in the plugin context - this is only to preserve isomorphic interface
    // between plugin & custom transform pass.
    source_map: std::sync::Arc<S>,
    comments: C,
    options: PluginArgs,
    scope_manager: ScopeManager,
    templates: HashMap<String, usize>,
    template_data: HashMap<usize, TemplateMetaData>,
    events: BTreeSet<String>,
    // Want to guarantee order (mostly for tests)
    imports: BTreeSet<String>,
    element_count: usize,
    ref_count: usize,
    v_count: usize,
    template_count: usize,
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
    fn register_template(
        &mut self,
        template: &str,
        is_ce: bool,
        is_svg: bool,
        is_import_node: bool,
    ) -> usize {
        if let Some(id) = self.templates.get(template) {
            // Template already exists
            *id
        } else {
            // Attaching new template
            self.template_count += 1;
            self.templates
                .insert(template.to_string(), self.template_count);
            self.template_data.insert(
                self.template_count,
                TemplateMetaData {
                    is_ce,
                    is_svg,
                    is_import_node,
                },
            );
            self.template_count
        }
    }
    fn register_event(&mut self, event: Cow<str>) {
        if !self.events.contains(event.as_ref()) {
            self.events.insert(event.into_owned());
        }
    }
    fn element_count(&mut self) -> &mut usize {
        &mut self.element_count
    }
    fn ref_count(&mut self) -> &mut usize {
        &mut self.ref_count
    }
    fn v_count(&mut self) -> &mut usize {
        &mut self.v_count
    }
    fn add_import(&mut self, import_name: Cow<str>) {
        if !self.imports.contains(import_name.as_ref()) {
            self.imports.insert(import_name.into_owned());
        }
    }
    fn add_event(&mut self, event_name: Cow<str>) {
        if !self.events.contains(event_name.as_ref()) {
            self.events.insert(event_name.into_owned());
        }
    }
    fn get_var_if_in_scope(&self, var: &swc_core::atoms::Atom) -> Option<&TrackedVariable> {
        self.scope_manager.try_get_var(var)
    }
    fn has_static_marker(&self, span_lo: swc_core::common::BytePos) -> bool {
        // Genuinely at a loss for words. No idea why this has to be done
        let span_lo = BytePos(span_lo.0 + 1);

        if let Some(comments) = self.comments.get_trailing(span_lo) {
            if let Some(c) = comments.last() {
                // Do not effect if @once preceeds expr
                return c.text.as_str() == self.options.static_marker.as_str();
            }
        }
        false
    }
}

impl<C: Clone + Comments, S: SourceMapper> SolidJsVisitor<C, S> {
    pub fn new(source_map: std::sync::Arc<S>, comments: C, plugin_options: PluginArgs) -> Self {
        SolidJsVisitor {
            source_map,
            comments,
            options: plugin_options,
            template_count: 0,
            element_count: 0,
            ref_count: 0,
            v_count: 0,
            scope_manager: ScopeManager::new(),
            templates: HashMap::new(),
            template_data: HashMap::new(),
            events: BTreeSet::new(),
            imports: BTreeSet::new(),
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
        let has_templates = !self.templates.is_empty();
        let mut imports_vec = add_imports(
            &mut self.imports,
            has_templates,
            self.options.module_name.clone(),
        );
        if has_templates {
            println!(
                "\nTemplate debugging: {0:?}\n\n{1:?}\n",
                self.templates, self.template_data
            );
            let decl = create_template_declarations(
                &mut self.templates,
                &mut self.template_data,
                self.template_count,
            );
            imports_vec.push(ModuleItem::Stmt(Stmt::Decl(Decl::Var(Box::new(decl)))));
        }

        // Have to insert these statements at the start - O(n) operation ugh
        match node {
            Program::Module(module) => {
                imports_vec.extend(module.body.drain(..).chain(add_events(&mut self.events)));
                module.body = imports_vec;
            }
            Program::Script(script) => {
                script.body = imports_vec
                    .into_iter()
                    .chain(add_events(&mut self.events))
                    // Lazy, not great way to just pull out templates
                    .filter_map(|x| {
                        if let ModuleItem::Stmt(s) = x {
                            Some(s)
                        } else {
                            None
                        }
                    })
                    .chain(script.body.drain(..))
                    .collect();
            }
        }
    }

    // Scope manager - a few places need to have context about vars

    fn visit_mut_var_decl(&mut self, n: &mut VarDecl) {
        let is_const = n.kind == VarDeclKind::Const;
        for declarator in &n.decls {
            self.scope_manager.add_var(declarator, is_const);
        }
        n.visit_mut_children_with(self);
    }

    // Function Declarations
    fn visit_mut_fn_decl(&mut self, n: &mut FnDecl) {
        self.scope_manager
            .declare_variable(n.ident.sym.clone(), TrackedVariable::FunctionIdent(false));
        n.visit_mut_children_with(self);
    }

    // Function and block scoping
    fn visit_mut_function(&mut self, n: &mut Function) {
        self.scope_manager.enter_scope();
        // Here you would also add parameters to the scope
        n.visit_mut_children_with(self);
        self.scope_manager.exit_scope();
    }

    fn visit_mut_block_stmt(&mut self, n: &mut BlockStmt) {
        self.scope_manager.enter_scope();
        n.visit_mut_children_with(self);
        self.scope_manager.exit_scope();
    }

    // Core visitor to find + transform JSX Expressions
    fn visit_mut_expr(&mut self, node: &mut swc_core::ecma::ast::Expr) {
        match create_new_expr_mut(node, self) {
            Ok(new_expr) => {
                *node = *new_expr;
            }
            Err(CreateNewExprError::NoChangeNeeded) => {
                // Any potential JSX in a block statement will be encapsulated
                // by some other statement, likely a return stmt. No need to handle here.
                node.visit_mut_children_with(self);
            }
            Err(CreateNewExprError::ExprNotFound) => {
                /* TODO: REMOVE - This shouldn't be possible!!! */
            }
        }
    }
}

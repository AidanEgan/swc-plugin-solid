use std::collections::HashMap;

use swc_core::{
    atoms::Atom,
    ecma::ast::{Lit, Pat, VarDeclarator},
};

#[derive(Debug, Clone)]
pub enum TrackedVariable {
    Literal(String),     // Literal string/num value. Used for classname optimization
    FunctionIdent(bool), // Variable used to initialize a fn. Used for delegated events
    Referred(Atom),      // Re-assigns an already tracked var
    StoredConstant,      // Refs track 'const' vars
    Imported,
}

#[derive(Debug)]
pub struct ScopeManager {
    scopes: Vec<HashMap<Atom, TrackedVariable>>,
}

impl ScopeManager {
    pub fn new() -> Self {
        ScopeManager {
            scopes: vec![HashMap::new()], // Start with a global scope
        }
    }

    pub fn enter_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn exit_scope(&mut self) {
        self.scopes.pop();
    }

    pub fn declare_variable(&mut self, name: Atom, data: TrackedVariable) {
        if let Some(current_scope) = self.scopes.last_mut() {
            current_scope.insert(name, data);
        }
    }

    fn is_in_scope(&self, name: &Atom) -> bool {
        self.scopes
            .iter()
            .rev()
            .any(|scope| scope.contains_key(name))
    }

    pub fn try_get_var(&self, name: &Atom) -> Option<&TrackedVariable> {
        for scope in self.scopes.iter().rev() {
            let v = scope.get(name);
            if v.is_some() {
                return v;
            }
        }
        None
    }

    pub fn track_import(&mut self, import_name: Atom) {
        self.declare_variable(import_name, TrackedVariable::Imported);
    }

    pub fn add_var(&mut self, declarator: &VarDeclarator, is_const: bool) {
        if let Some(some_expr) = &declarator.init {
            match &declarator.name {
                Pat::Ident(ident) => {
                    if some_expr.is_arrow() || some_expr.is_fn_expr() {
                        self.declare_variable(
                            ident.sym.clone(),
                            TrackedVariable::FunctionIdent(is_const),
                        );
                        return;
                    }
                    if let Some(other_ident) = some_expr.as_ident() {
                        if self.is_in_scope(&other_ident.sym) {
                            self.declare_variable(
                                ident.sym.clone(),
                                TrackedVariable::Referred(other_ident.sym.clone()),
                            );
                            return;
                        }
                    }
                    if let Some(lit) = some_expr.as_lit() {
                        match lit {
                            Lit::Str(lit_str) => {
                                self.declare_variable(
                                    ident.sym.clone(),
                                    TrackedVariable::Literal(lit_str.value.to_string()),
                                );
                                return;
                            }
                            Lit::Num(lit_num) => {
                                self.declare_variable(
                                    ident.sym.clone(),
                                    TrackedVariable::Literal(lit_num.value.to_string()),
                                );
                                return;
                            }
                            _ => { /* Skip */ }
                        }
                    }
                    // Not really intereseted in some of these
                    if is_const
                        && !(some_expr.is_arrow()
                            || some_expr.is_call()
                            || some_expr.is_jsx_element()
                            || some_expr.is_jsx_fragment())
                    {
                        self.declare_variable(ident.sym.clone(), TrackedVariable::StoredConstant);
                    }
                }
                _ => { /* Handle other patterns */ }
            }
        }
    }
}

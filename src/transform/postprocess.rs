use std::collections::{BTreeMap, HashSet};

use swc_core::{
    atoms::Atom,
    common::{source_map::PURE_SP, SyntaxContext, DUMMY_SP},
    ecma::ast::{
        BindingIdent, CallExpr, Callee, Expr, ExprOrSpread, Ident, ImportDecl,
        ImportNamedSpecifier, ImportPhase, ImportSpecifier, Lit, ModuleDecl, ModuleExportName,
        ModuleItem, Pat, VarDecl, VarDeclKind, VarDeclarator,
    },
};

use crate::helpers::generate_var_names::{
    generate_import_name, generate_template_expr_name, generate_template_name,
};

fn ident_name(name: Atom, is_pure: bool) -> Ident {
    Ident {
        span: if is_pure { PURE_SP } else { DUMMY_SP },
        ctxt: SyntaxContext::empty(),
        sym: name,
        optional: false,
    }
}

pub fn create_template_declarations(templates: &mut BTreeMap<String, usize>) -> VarDecl {
    let mut swapped: BTreeMap<String, usize> = BTreeMap::new();
    std::mem::swap(templates, &mut swapped);
    let decls = swapped
        .into_iter()
        .map(|(k, v)| {
            let tempvar = VarDeclarator {
                span: DUMMY_SP,
                name: Pat::Ident(BindingIdent {
                    id: ident_name(generate_template_name(v), false),
                    type_ann: None,
                }),
                init: Some(Box::new(Expr::Call(CallExpr {
                    span: DUMMY_SP,
                    ctxt: SyntaxContext::empty(),
                    callee: Callee::Expr(Box::new(Expr::Ident(ident_name(
                        generate_template_expr_name(),
                        true,
                    )))),
                    args: vec![ExprOrSpread {
                        spread: None,
                        expr: Box::new(Expr::Lit(Lit::Str(k.into()))),
                    }],
                    type_args: None,
                }))),
                definite: false,
            };
            tempvar
        })
        .collect();

    VarDecl {
        span: DUMMY_SP,
        ctxt: SyntaxContext::empty(),
        kind: VarDeclKind::Var,
        declare: false,
        decls,
    }
}

fn generic_import(name: Atom, module_name: &str) -> ModuleItem {
    ModuleItem::ModuleDecl(ModuleDecl::Import(ImportDecl {
        span: DUMMY_SP,
        specifiers: vec![ImportSpecifier::Named(ImportNamedSpecifier {
            span: DUMMY_SP,
            local: ident_name(name.clone(), false),
            imported: Some(ModuleExportName::Ident(ident_name(
                generate_import_name(name),
                false,
            ))),
            is_type_only: false,
        })],
        src: Box::new(module_name.into()),
        type_only: false,
        with: None,
        phase: ImportPhase::Evaluation,
    }))
}

pub fn add_imports(
    imports: &mut HashSet<String>,
    has_templates: bool,
    module_name: String,
) -> Vec<ModuleItem> {
    let mut stmts: Vec<ModuleItem> = Vec::new();
    if has_templates {
        let import_stmt = generic_import(generate_template_expr_name(), &module_name);
        stmts.push(import_stmt);
    }
    stmts.extend(
        imports
            .drain()
            .map(|imp| generic_import(imp.into(), &module_name)),
    );
    stmts
}

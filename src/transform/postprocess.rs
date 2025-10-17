use std::collections::{BTreeSet, HashMap};

use swc_core::{
    atoms::Atom,
    common::{SyntaxContext, DUMMY_SP},
    ecma::ast::{
        ArrayLit, BindingIdent, CallExpr, Callee, Expr, ExprOrSpread, ExprStmt, Ident, ImportDecl,
        ImportNamedSpecifier, ImportPhase, ImportSpecifier, Lit, ModuleDecl, ModuleExportName,
        ModuleItem, Pat, Stmt, VarDecl, VarDeclKind, VarDeclarator,
    },
};

use crate::{
    helpers::{
        common_into_expressions::{ident_callee, ident_name},
        generate_var_names::{
            generate_import_name, generate_template_expr_name, generate_template_name,
            DELEGATE_EVENTS,
        },
    },
    transform::TemplateMetaData,
};

fn empty_var_decl() -> VarDeclarator {
    VarDeclarator {
        span: DUMMY_SP,
        name: Pat::Ident(BindingIdent {
            id: Ident {
                span: DUMMY_SP,
                ctxt: SyntaxContext::empty(),
                sym: Atom::default(),
                optional: false,
            },
            type_ann: None,
        }),
        init: None,
        definite: false,
    }
}

pub fn create_template_declarations(
    templates: &mut HashMap<String, usize>,
    template_data: &mut HashMap<usize, TemplateMetaData>,
    num_templates: usize,
) -> VarDecl {
    let mut decls: Vec<VarDeclarator> = vec![empty_var_decl(); num_templates];
    // 2 assumptions that should always be true:
    // 1 <= i <= num_templates && i is unique
    for (template, i) in templates.drain() {
        let (is_ce, is_svg, is_import_node, is_mathml) = template_data
            .get(&i)
            .map(|d| (d.is_ce, d.is_svg, d.is_import_node, d.is_mathml))
            .unwrap_or((false, false, false, false));
        let mut args = vec![ExprOrSpread {
            spread: None,
            expr: Box::new(Expr::Lit(Lit::Str(template.into()))),
        }];
        if is_ce || is_svg || is_import_node || is_mathml {
            args.push(Expr::Lit(Lit::Bool((is_ce || is_import_node).into())).into());
            args.push(Expr::Lit(Lit::Bool(is_svg.into())).into());
            args.push(Expr::Lit(Lit::Bool(is_mathml.into())).into());
        }

        decls[i - 1].name.as_mut_ident().unwrap().id.sym = generate_template_name(i);
        decls[i - 1].init = Some(Box::new(Expr::Call(CallExpr {
            span: DUMMY_SP,
            ctxt: SyntaxContext::empty(),
            callee: Callee::Expr(Box::new(Expr::Ident(ident_name(
                generate_template_expr_name(),
                true,
            )))),
            args,
            type_args: None,
        })));
    }
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
    imports: &mut BTreeSet<String>,
    has_templates: bool,
    module_name: String,
) -> Vec<ModuleItem> {
    let mut stmts: Vec<ModuleItem> = Vec::new();
    if has_templates {
        let import_stmt = generic_import(generate_template_expr_name(), &module_name);
        stmts.push(import_stmt);
    }

    stmts.extend(
        std::mem::take(imports)
            .into_iter()
            .map(|imp| generic_import(imp.into(), &module_name)),
    );
    stmts
}

pub fn add_events(events: &mut BTreeSet<String>) -> Option<ModuleItem> {
    if events.is_empty() {
        None
    } else {
        let events_arr: Vec<Option<ExprOrSpread>> = std::mem::take(events)
            .into_iter()
            .map(|e| {
                Some(ExprOrSpread {
                    spread: None,
                    expr: Expr::Lit(Lit::Str(e.into())).into(),
                })
            })
            .collect();
        let exp = CallExpr {
            span: DUMMY_SP,
            ctxt: SyntaxContext::empty(),
            callee: ident_callee(DELEGATE_EVENTS.into()),
            args: vec![Expr::Array(ArrayLit {
                span: DUMMY_SP,
                elems: events_arr,
            })
            .into()],
            type_args: None,
        };
        Some(ModuleItem::Stmt(Stmt::Expr(ExprStmt {
            span: DUMMY_SP,
            expr: exp.into(),
        })))
    }
}

use super::super::parser_types::JsxTemplateKind;
use super::jsx_parser_client::ClientJsxElementVisitor;
use crate::{
    builder::{
        builder_types::Kind,
        client::{
            block_expr_builder::BlockExprBuilder,
            builder_helpers::{
                block_to_call_expr, id_to_call_expr, wrap_in_empty_arrow, wrap_with_memo,
            },
            custom_client_component_builder::ClientCustomComponentBuilder,
            effect_builder::EffectBuilder,
            element_properties::{ElementPropertiesBuilder, StatementInserts},
            insert_queue::{InsertBuilder, InsertQueue, PossibleInsert},
            jsx_expr_transformer_client::ClientJsxExprTransformer,
        },
        parser_types::{JsxFragmentMetadata, PossiblePlaceholders},
    },
    helpers::generate_var_names::{generate_effect, generate_insert, generate_template_name},
    transform::parent_visitor::ParentVisitor,
};

use swc_core::{
    common::{util::take::Take, SyntaxContext, DUMMY_SP},
    ecma::ast::{
        ArrayLit, BlockStmt, BlockStmtOrExpr, CallExpr, Callee, Expr, ExprOrSpread, Ident,
    },
};

pub enum BuildResults {
    Block(BlockStmt),
    Arr(ArrayLit),
    Id(Ident),
    Completed(Box<Expr>),
}

pub fn standard_build_res_wrappings(br: BuildResults) -> Box<Expr> {
    match br {
        BuildResults::Block(block_stmt) => Box::new(Expr::Call(block_to_call_expr(block_stmt))),
        BuildResults::Arr(array_lit) => Box::new(Expr::Array(array_lit)),
        BuildResults::Id(ident) => Box::new(Expr::Call(id_to_call_expr(ident))),
        BuildResults::Completed(comp) => comp,
    }
}

fn add_closes(templ_string: &mut String, closing_el_builder: &mut String) {
    if !closing_el_builder.is_empty() {
        // Adds closing elements to template and resets the
        // builder string
        *templ_string += closing_el_builder.drain(..).as_str();
    }
}

fn add_opening(
    templ_string: &mut String,
    parent_element_stack: Option<&mut Vec<usize>>,
    opening_el: &str,
    props: Option<Vec<(String, String)>>,
    count: usize,
) {
    if let Some(parent_element_stack) = parent_element_stack {
        parent_element_stack.push(count);
    }
    *templ_string += "<";
    *templ_string += opening_el;
    if let Some(props) = props {
        props.into_iter().for_each(|(k, v)| {
            if v.is_empty() {
                *templ_string += " ";
                *templ_string += k.as_str();
            } else if v.contains(" ") {
                // Keep and escape
                *templ_string += format!(r#" {0}="{1}""#, k, v).as_str();
            } else {
                *templ_string += format!(" {0}={1}", k, v).as_str();
            }
        });
    }
    *templ_string += ">";
}

fn memo_call_expr_mut(res: &mut BuildResults) {
    if let BuildResults::Completed(expr) = res {
        if let Expr::Call(call_expr) = &mut **expr {
            if let Callee::Expr(inner_expr) = &mut call_expr.callee {
                if call_expr.args.is_empty() {
                    let mut dummy = Box::new(Expr::dummy());
                    std::mem::swap(inner_expr, &mut dummy);
                    *expr = Box::new(wrap_with_memo(dummy).into());
                }
            } else {
                let mut dummy = CallExpr::dummy();
                std::mem::swap(call_expr, &mut dummy);
                let new_expr = Box::new(dummy.into());
                *expr = Box::new(
                    wrap_with_memo(Box::new(
                        wrap_in_empty_arrow(BlockStmtOrExpr::Expr(new_expr).into()).into(),
                    ))
                    .into(),
                );
            }
        }
    }
}

fn handle_fragment_chunks<T: ParentVisitor>(
    mut frag: JsxFragmentMetadata<ClientJsxElementVisitor>,
    parent_visitor: &mut T,
) -> BuildResults {
    if frag.children.len() == 1 {
        let mut build_res = build_js_from_client_jsx(frag.children.remove(0), parent_visitor);
        memo_call_expr_mut(&mut build_res);
        return build_res;
    }
    let arr = ArrayLit {
        span: DUMMY_SP,
        elems: frag
            .children
            .into_iter()
            .map(|child| {
                let mut build_res = build_js_from_client_jsx(child, parent_visitor);
                memo_call_expr_mut(&mut build_res);
                Some(ExprOrSpread {
                    spread: None,
                    expr: standard_build_res_wrappings(build_res),
                })
            })
            .collect(),
    };
    BuildResults::Arr(arr)
}

pub fn build_js_from_client_jsx<T: ParentVisitor>(
    mut parsed_data: ClientJsxElementVisitor,
    parent_visitor: &mut T,
) -> BuildResults {
    // Special cases
    if let Some(JsxTemplateKind::Placeholder(p)) = parsed_data.template.first() {
        // Remove is fine becuse the length of the array is always just 1.
        match parsed_data.placeholders.remove(*p) {
            Some(PossiblePlaceholders::Fragment(f)) => {
                return handle_fragment_chunks(f, parent_visitor);
            }
            Some(PossiblePlaceholders::Component(c)) => {
                let mut client_component_builder =
                    ClientCustomComponentBuilder::new(parent_visitor, c);
                return BuildResults::Completed(
                    client_component_builder.build_and_wrap_custom_component(),
                );
            }
            Some(PossiblePlaceholders::Expression(e)) => {
                return BuildResults::Completed(e);
            }
            None => { /* Do Nothing here */ }
        }
    }

    let mut templ_ce = false;
    let mut templ_svg = false;
    let mut templ_import_node = false;
    let mut templ_mathml = false;

    let mut templ_string = String::new();
    let mut closing_el_builder = String::new();

    let mut parent_element_stack: Vec<usize> = Vec::new();

    /* Helper utils so we don't have to re-traverse the vec */
    let mut insert_queue = InsertQueue::new();
    let mut block_builder = BlockExprBuilder::new();
    let mut effect_builder = EffectBuilder::new();

    // Build template string -> attach to parent visitor
    for part in parsed_data.template {
        let count = *(parent_visitor.element_count());
        match part {
            JsxTemplateKind::Opening(open) => {
                let implicit_self_close = open.implicit_self_close;
                templ_ce = templ_ce || open.is_ce;
                templ_svg = templ_svg || open.is_svg;
                templ_import_node = templ_import_node || open.is_import_node;
                templ_mathml = templ_mathml || open.is_mathml;
                add_closes(&mut templ_string, &mut closing_el_builder);

                let num_els = open.attrs.len();
                let (el_name, tmpl_inserts) = if num_els > 0 {
                    let mut property_builder =
                        ElementPropertiesBuilder::new(parent_visitor, &mut effect_builder);
                    let (element_name, all_data_inlined) =
                        property_builder.build_el_property_statements(open, count);

                    // Check if all attrs were not inlined
                    if !all_data_inlined {
                        block_builder.set_last_used_decl(count);
                    }
                    for to_insert in property_builder.statements.drain(..) {
                        block_builder.add_stmt(to_insert);
                    }
                    // Insert this BEFORE special inserts
                    block_builder.add_decl(count);
                    for statement_insert in property_builder.statement_inserts {
                        match statement_insert {
                            StatementInserts::TextContentData => {
                                // Add space and statement
                                templ_string += " ";
                                block_builder.add_decl_specific(
                                    count + 1,
                                    crate::builder::builder_types::ElDeclKinds::FirstChild(count),
                                );
                            }
                        }
                    }
                    (
                        element_name,
                        Some(std::mem::take(
                            &mut property_builder.direct_template_inserts,
                        )),
                    )
                } else {
                    // Regular insert
                    block_builder.add_decl(count);
                    (open.value, None)
                };
                add_opening(
                    &mut templ_string,
                    if !implicit_self_close {
                        Some(&mut parent_element_stack)
                    } else {
                        None
                    },
                    &el_name,
                    tmpl_inserts,
                    count,
                );

                insert_queue
                    .drain_insert_queue(PossibleInsert::At(count), block_builder.get_final_stmts());
                block_builder.set_kind(Kind::Open(count, implicit_self_close));
                *(parent_visitor.element_count()) += 1;
            }
            JsxTemplateKind::Closing(close) => {
                // The 'next sibling' is in reference to the opening element's id
                let open_id = parent_element_stack.pop();
                closing_el_builder += "</";
                closing_el_builder += &(close + ">");
                insert_queue
                    .drain_insert_queue(PossibleInsert::Null, block_builder.get_final_stmts());
                // open_id SHOULD always be defined
                block_builder.set_kind(Kind::Close(
                    open_id.expect("Closing tag should have corresponding open tag"),
                ));
                // Do not incremement element_count or add_decl. Not referenced in template
            }
            JsxTemplateKind::Text(txt) => {
                if let Kind::Placeholder(_, false, _) = block_builder.prev_kind {
                    templ_string += "<!>"; //Placeholder in templ syntax
                    block_builder.add_decl(count);
                    *(parent_visitor.element_count()) += 1;
                }
                templ_string += txt.as_str();
                block_builder.add_decl(count);
                insert_queue
                    .drain_insert_queue(PossibleInsert::At(count), block_builder.get_final_stmts());
                block_builder.set_kind(Kind::Text(count));
                *(parent_visitor.element_count()) += 1;
            }
            // Needs long form true
            JsxTemplateKind::Placeholder(p) => {
                block_builder.set_last_used_decl(count);
                // Only gets added to template in case it is sandwhiched between two
                // jsxtext elements. So it's template addition is handled above instead
                // of here.
                if let Some(placeholder_data) = parsed_data.placeholders.get_mut(p) {
                    let placeholder_expr = match placeholder_data.take() {
                        Some(PossiblePlaceholders::Component(c)) => {
                            let mut client_builder =
                                ClientCustomComponentBuilder::new(parent_visitor, c);
                            // TODO YOU NEED TO BUILD THE INSERT CALLEXPR AND WRAP THIS IN IT!
                            Some(client_builder.build_and_wrap_custom_component())
                        }
                        // TODO TRANSFORM EXPR
                        Some(PossiblePlaceholders::Expression(mut e)) => {
                            let mut t = ClientJsxExprTransformer::new(parent_visitor, true, false);
                            t.visit_and_wrap_outer_expr(&mut e);
                            Some(e)
                        }
                        Some(PossiblePlaceholders::Fragment(f)) => Some(
                            standard_build_res_wrappings(handle_fragment_chunks(f, parent_visitor)),
                        ),
                        _ => None, // Something went wrong :( (add logging?)
                    };
                    if let Some(placeholder_expr) = placeholder_expr {
                        insert_queue.add(
                            InsertBuilder {
                                parent_el: *parent_element_stack.last().unwrap_or(&0_usize), // There should always be an element
                                expr: placeholder_expr,
                            },
                            matches!(block_builder.prev_kind, Kind::Open(_, _)),
                        );
                    }
                }
                block_builder.set_kind(Kind::Placeholder(
                    match block_builder.prev_kind {
                        Kind::Open(at, _) => at,
                        Kind::Placeholder(at, _, _) => at,
                        Kind::Text(at) => at,
                        Kind::Close(at) => at,
                        Kind::None => count,
                    },
                    match block_builder.prev_kind {
                        Kind::Text(_) => false,
                        Kind::Placeholder(_, bool, _) => bool,
                        _ => true,
                    },
                    match block_builder.prev_kind {
                        Kind::Placeholder(_, _, bool) => bool,
                        Kind::Open(_, is_self_closing) => !is_self_closing,
                        Kind::None => true,
                        _ => false,
                    },
                ));
            }
        }
    }
    let temp_id = parent_visitor.register_template(
        templ_string.as_str(),
        templ_ce,
        templ_svg,
        templ_import_node,
        templ_mathml,
    );

    if block_builder.last_used_decl.is_some() {
        insert_queue.drain_insert_queue(PossibleInsert::Undefined, block_builder.get_final_stmts());
        if insert_queue.used_insert {
            parent_visitor.add_import(generate_insert().as_str().into());
        }
        // Does need long form
        // This should always be true - Rust just likes to be very sure :)
        // Save for return stmt
        block_builder.add_decls_to_final(temp_id);
        let mut add_import = false;
        // Check for effects
        let mut property_builder =
            ElementPropertiesBuilder::new(parent_visitor, &mut effect_builder);
        if let Some(stmt) = property_builder.build_effect_statement(false) {
            add_import = true;
            block_builder.add_penultimate_stmt(stmt);
        }
        while let Some(isolated_effect_stmt) = property_builder.build_effect_statement(true) {
            add_import = true;
            block_builder.add_penultimate_stmt(isolated_effect_stmt);
        }

        if add_import {
            parent_visitor.add_import(generate_effect().as_str().into());
        }

        BuildResults::Block(BlockStmt {
            span: DUMMY_SP,
            ctxt: SyntaxContext::empty(),
            stmts: block_builder.take_final_stmts(),
        })
    } else {
        // Most basic case, just returns the tmplate string

        let name = Ident::new(
            generate_template_name(temp_id),
            DUMMY_SP,
            SyntaxContext::empty(),
        );
        BuildResults::Id(name)
    }
}

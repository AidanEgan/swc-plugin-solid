use super::super::parser_types::JsxTemplateKind;
use super::jsx_parser_client::ClientJsxElementVisitor;
use crate::{
    builder::{
        builder_types::Kind,
        client::{
            block_expr_builder::BlockExprBuilder,
            builder_helpers::{block_to_call_expr, id_to_call_expr, name_as_expr},
            insert_queue::{InsertBuilder, InsertQueue, PossibleInsert},
        },
        parser_types::{JsxCustomComponentMetadata, JsxFragmentMetadata, PossiblePlaceholders},
    },
    helpers::generate_var_names::{generate_create_component_name, generate_template_name},
    transform::parent_visitor::ParentVisitor,
};

use swc_core::{
    common::{SyntaxContext, DUMMY_SP},
    ecma::ast::{
        ArrayLit, BlockStmt, CallExpr, Callee, Expr, ExprOrSpread, GetterProp, Ident, IdentName,
        KeyValueProp, ObjectLit, Prop, PropName, PropOrSpread, ReturnStmt, Stmt,
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
    parent_element_stack: &mut Vec<usize>,
    opening_el: &str,
    count: usize,
) {
    parent_element_stack.push(count);
    *templ_string += "<";
    *templ_string += opening_el;
    *templ_string += ">";
}

fn create_component_expr<T: ParentVisitor>(
    mut comp: JsxCustomComponentMetadata<ClientJsxElementVisitor>,
    parent_visitor: &mut T,
) -> CallExpr {
    let props = comp
        .props
        .into_iter()
        .map(|(key, val)| {
            let key = PropName::Ident(IdentName {
                span: DUMMY_SP,
                sym: key.into(),
            });
            // Might be other edge cases but callexpr is
            // the only one I'm seeing
            let prop = if val.is_call() {
                Prop::Getter(GetterProp {
                    span: DUMMY_SP,
                    key,
                    type_ann: None,
                    body: Some(BlockStmt {
                        span: DUMMY_SP,
                        ctxt: SyntaxContext::empty(),
                        stmts: vec![Stmt::Return(ReturnStmt {
                            span: DUMMY_SP,
                            arg: Some(Box::new(val)),
                        })],
                    }),
                })
            } else {
                Prop::KeyValue(KeyValueProp {
                    key,
                    value: Box::new(val),
                })
            };
            PropOrSpread::Prop(Box::new(prop))
        })
        .collect();
    let mut obj = ObjectLit {
        span: DUMMY_SP,
        props,
    };
    // Add children
    if !comp.children.is_empty() {
        let blockstmt = if comp.children.len() == 1 {
            match build_js_from_client_jsx(comp.children.remove(0), parent_visitor) {
                BuildResults::Block(bs) => Some(bs),
                BuildResults::Id(id) => Some(BlockStmt {
                    span: DUMMY_SP,
                    ctxt: SyntaxContext::empty(),
                    stmts: vec![Stmt::Return(ReturnStmt {
                        span: DUMMY_SP,
                        arg: Some(Box::new(Expr::Call(id_to_call_expr(id)))),
                    })],
                }),
                BuildResults::Arr(arr) => Some(BlockStmt {
                    span: DUMMY_SP,
                    ctxt: SyntaxContext::empty(),
                    stmts: vec![Stmt::Return(ReturnStmt {
                        span: DUMMY_SP,
                        arg: Some(Box::new(Expr::Array(arr))),
                    })],
                }),
                BuildResults::Completed(comp) => {
                    if !comp.is_call() {
                        let childrenval = Prop::KeyValue(KeyValueProp {
                            key: PropName::Ident("children".into()),
                            value: comp,
                        });
                        obj.props.push(PropOrSpread::Prop(Box::new(childrenval)));
                        // EVALUTAION STOPS HERE IN THIS CASE
                        None
                    } else {
                        // Only call expressions treated like this
                        Some(BlockStmt {
                            span: DUMMY_SP,
                            ctxt: SyntaxContext::empty(),
                            stmts: vec![Stmt::Return(ReturnStmt {
                                span: DUMMY_SP,
                                arg: Some(comp),
                            })],
                        })
                    }
                }
            }
        } else {
            let arr = ArrayLit {
                span: DUMMY_SP,
                elems: comp
                    .children
                    .into_iter()
                    .map(|c| {
                        Some(ExprOrSpread {
                            spread: None,
                            expr: standard_build_res_wrappings(build_js_from_client_jsx(
                                c,
                                parent_visitor,
                            )),
                        })
                    })
                    .collect(),
            };
            Some(BlockStmt {
                span: DUMMY_SP,
                ctxt: SyntaxContext::empty(),
                stmts: vec![Stmt::Return(ReturnStmt {
                    span: DUMMY_SP,
                    arg: Some(Box::new(Expr::Array(arr))),
                })],
            })
        };
        if blockstmt.is_some() {
            let childrengetter = Prop::Getter(GetterProp {
                span: DUMMY_SP,
                key: PropName::Ident("children".into()),
                type_ann: None,
                body: blockstmt,
            });
            obj.props.push(PropOrSpread::Prop(Box::new(childrengetter)));
        } // Other case handled already within match exp
    };
    CallExpr {
        span: DUMMY_SP,
        ctxt: SyntaxContext::empty(),
        callee: Callee::Expr(name_as_expr(generate_create_component_name())),
        args: vec![ExprOrSpread {
            spread: None,
            expr: name_as_expr(comp.value.into()),
        }],
        type_args: None,
    }
}

fn handle_fragment_chunks<T: ParentVisitor>(
    mut frag: JsxFragmentMetadata<ClientJsxElementVisitor>,
    parent_visitor: &mut T,
) -> BuildResults {
    if frag.children.len() == 1 {
        return build_js_from_client_jsx(frag.children.remove(0), parent_visitor);
    }
    let arr = ArrayLit {
        span: DUMMY_SP,
        elems: frag
            .children
            .into_iter()
            .map(|child| {
                Some(ExprOrSpread {
                    spread: None,
                    expr: standard_build_res_wrappings(build_js_from_client_jsx(
                        child,
                        parent_visitor,
                    )),
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
                return BuildResults::Completed(Box::new(Expr::Call(create_component_expr(
                    c,
                    parent_visitor,
                ))));
            }
            Some(PossiblePlaceholders::Expression(e)) => {
                return BuildResults::Completed(Box::new(e));
            }
            None => { /* Do Nothing here */ }
        }
    }

    let mut templ_string = String::new();
    let mut closing_el_builder = String::new();

    let mut parent_element_stack: Vec<usize> = Vec::new();

    /* Helper utils so we don't have to re-traverse the vec */
    let mut needs_long_form = false;
    let mut insert_queue = InsertQueue::new();
    let mut block_builder = BlockExprBuilder::new();

    // Build template string -> attach to parent visitor
    for part in parsed_data.template {
        let count = *(parent_visitor.element_count());
        match part {
            JsxTemplateKind::Opening(open) => {
                needs_long_form = needs_long_form
                    || !open.attrs.is_empty()
                    || !open.events.is_empty()
                    || !open.styles.is_empty();
                add_closes(&mut templ_string, &mut closing_el_builder);
                add_opening(
                    &mut templ_string,
                    &mut parent_element_stack,
                    &open.value,
                    count,
                );
                block_builder.add_decl(count);
                insert_queue
                    .drain_insert_queue(PossibleInsert::At(count), block_builder.get_final_stmts());
                block_builder.set_kind(Kind::Open(count));
            }
            JsxTemplateKind::Closing(close) => {
                parent_element_stack.pop();
                closing_el_builder += "</";
                closing_el_builder += &(close + ">");
                block_builder.add_decl(count);
                insert_queue
                    .drain_insert_queue(PossibleInsert::Null, block_builder.get_final_stmts());
                block_builder.set_kind(Kind::Close(count));
            }
            JsxTemplateKind::Text(txt) => {
                if let Kind::Placeholder(_, false) = block_builder.prev_kind {
                    templ_string += "<!>"; //Placeholder in templ syntax
                    block_builder.add_decl(count);
                    *(parent_visitor.element_count()) += 1;
                }
                templ_string += txt.as_str();
                block_builder.add_decl(count);
                insert_queue
                    .drain_insert_queue(PossibleInsert::At(count), block_builder.get_final_stmts());
                block_builder.set_kind(Kind::Text(count));
            }
            // Needs long form true
            JsxTemplateKind::Placeholder(p) => {
                needs_long_form = true;
                // Only gets added to template in case it is sandwhiched between two
                // jsxtext elements. So it's template addition is handled above instead
                // of here.
                if let Some(placeholder_data) = parsed_data.placeholders.get_mut(p) {
                    let placeholder_expr = match placeholder_data.take() {
                        Some(PossiblePlaceholders::Component(c)) => {
                            let transformed = create_component_expr(c, parent_visitor);
                            // TODO YOU NEED TO BUILD THE INSERT CALLEXPR AND WRAP THIS IN IT!
                            Some(Box::new(Expr::Call(transformed)))
                        }
                        Some(PossiblePlaceholders::Expression(e)) => Some(Box::new(e)),
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
                            matches!(block_builder.prev_kind, Kind::Open(_)),
                        );
                    }
                }
                block_builder.set_kind(Kind::Placeholder(
                    count,
                    match block_builder.prev_kind {
                        Kind::Text(_) => false,
                        Kind::Placeholder(_, bool) => bool,
                        _ => true,
                    },
                ));
            }
        }
        // Either frament or custom component. Both placeholder types
        *(parent_visitor.element_count()) += 1;
    }
    let temp_id = parent_visitor.get_template_id(templ_string.as_str());
    // Most basic case, just returns the tmplate string
    if !needs_long_form {
        let name = Ident::new(
            generate_template_name(temp_id),
            DUMMY_SP,
            SyntaxContext::empty(),
        );
        return BuildResults::Id(name);
    }
    // Does need long form
    // This should always be true - Rust just likes to be very sure :)
    // Save for return stmt
    block_builder.add_decls_to_final(temp_id);

    BuildResults::Block(BlockStmt {
        span: DUMMY_SP,
        ctxt: SyntaxContext::empty(),
        stmts: block_builder.take_final_stmts(),
    })
    // Still need to add all different handlers. Custom logic for styles and classnames
}

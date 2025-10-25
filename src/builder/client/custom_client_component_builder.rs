use swc_core::{
    atoms::Atom,
    common::{util::take::Take, BytePos, Spanned, SyntaxContext, DUMMY_SP},
    ecma::{
        ast::{
            ArrayLit, BinaryOp, BlockStmt, CallExpr, ComputedPropName, Expr, ExprOrSpread,
            Function, GetterProp, Ident, IdentName, JSXAttrOrSpread, JSXAttrValue, JSXExpr,
            KeyValueProp, Lit, MethodProp, ObjectLit, Param, Pat, Prop, PropName, PropOrSpread,
            ReturnStmt, Stmt,
        },
        visit::VisitMutWith,
    },
};

use crate::{
    builder::{
        client::{
            element_properties::ref_statement_builder::create_ref_statements,
            jsx_expr_builder_client::{
                build_js_from_client_jsx, memo_expr_mut, standard_build_res_wrappings, BuildResults,
            },
            jsx_expr_transformer_client::ClientJsxExprTransformer,
            jsx_parser_client::ClientJsxElementVisitor,
        },
        parser_types::JsxCustomComponentMetadata,
    },
    helpers::{
        common_into_expressions::{cannot_convert_to_ident, ident_callee, ident_expr, ident_name},
        generate_var_names::{
            generate_create_component_name, generate_merge_props, generate_ref_raw, MEMO, REF_RAW,
        },
        parent_visitor_helpers::check_var_name_in_scope,
    },
    transform::{parent_visitor::ParentVisitor, scope_manager::TrackedVariable},
};

enum InnerIifeRes {
    NoChange(Box<Expr>),
    InnerExpr(Box<Expr>),
    InnerStmts(Vec<Stmt>),
}

fn grab_iife_mut(mut expr: Box<Expr>) -> InnerIifeRes {
    if let Some(call_expr) = expr.as_mut_call() {
        if call_expr.args.is_empty() {
            if let Some(inner_expr) = call_expr.callee.as_mut_expr() {
                if let Some(inner_expr) = inner_expr.as_mut_paren().map(|p| &mut p.expr) {
                    if let Some(inner_arrow) = inner_expr.as_mut_arrow() {
                        if inner_arrow.params.is_empty() {
                            return match &mut *inner_arrow.body {
                                swc_core::ecma::ast::BlockStmtOrExpr::BlockStmt(block_stmt) => {
                                    InnerIifeRes::InnerStmts(std::mem::take(&mut block_stmt.stmts))
                                }
                                swc_core::ecma::ast::BlockStmtOrExpr::Expr(expr) => {
                                    InnerIifeRes::InnerExpr(std::mem::take(expr))
                                }
                            };
                        }
                    } else if let Some(inner_fn) = inner_expr.as_mut_fn_expr() {
                        if inner_fn.function.params.is_empty() && inner_fn.function.body.is_some() {
                            return InnerIifeRes::InnerStmts(std::mem::take(
                                &mut inner_fn.function.body.as_mut().unwrap().stmts,
                            ));
                        }
                    }
                }
            }
        }
    }
    InnerIifeRes::NoChange(expr)
}

fn grab_inner_expr(raw: JSXAttrOrSpread) -> (Box<Expr>, Option<BytePos>) {
    match raw {
        JSXAttrOrSpread::JSXAttr(jsxattr) => match jsxattr.value {
            Some(JSXAttrValue::Lit(l)) => (Box::new(l.into()), None),
            Some(JSXAttrValue::JSXElement(e)) => (Box::new(e.into()), None),
            Some(JSXAttrValue::JSXFragment(f)) => (Box::new(f.into()), None),
            Some(JSXAttrValue::JSXExprContainer(c)) => {
                let span_lo = c.span_lo();
                match c.expr {
                    JSXExpr::JSXEmptyExpr(_) => (Box::new(Expr::default()), None),
                    JSXExpr::Expr(expr) => (expr, Some(span_lo)),
                }
            }
            None => (Box::new(Expr::Lit(Lit::Bool(true.into()))), None),
        },
        JSXAttrOrSpread::SpreadElement(spread_element) => (spread_element.expr, None),
    }
}

fn transform_inner_expression_mut<T: ParentVisitor>(
    parent_visitor: &mut T,
    to_visit: &mut Box<Expr>,
    wrap_member_exprs: bool,
) {
    let mut attribute_visitor = ClientJsxExprTransformer::new(parent_visitor, true, true, true);
    attribute_visitor.visit_and_wrap_outer_expr(to_visit, wrap_member_exprs);
}

fn build_ref_expr(stmts: Vec<Stmt>) -> Prop {
    let new_val = MethodProp {
        key: PropName::Ident(ident_name(generate_ref_raw(), false).into()),
        function: Box::new(Function {
            params: vec![Param {
                span: DUMMY_SP,
                decorators: vec![],
                pat: Pat::Ident(ident_name("r$".into(), false).into()),
            }],
            decorators: vec![],
            span: DUMMY_SP,
            ctxt: SyntaxContext::empty(),
            is_generator: false,
            is_async: false,
            type_params: None,
            return_type: None,
            body: Some(BlockStmt {
                span: DUMMY_SP,
                ctxt: SyntaxContext::empty(),
                stmts,
            }),
        }),
    };
    Prop::Method(new_val)
}

fn build_props_oject_expr<T: ParentVisitor>(
    parent_visitor: &mut T,
    needs_children_getter: bool,
    props: Vec<ParsedProp>,
) -> Box<Expr> {
    let mut statements: Vec<Stmt> = vec![];
    let mut static_ref = false;
    let props = props
        .into_iter()
        .map(|parsed| {
            let mut needs_getter_prop = false;
            let (key, mut val, is_static) = (parsed.name, parsed.expr, parsed.is_static);
            // Apply correct transformation
            match key.as_str() {
                REF_RAW => {
                    // Ref statements change a bit differently
                    if let Some(ident) = val.as_ident() {
                        match check_var_name_in_scope(parent_visitor, &ident.sym) {
                            Ok(_) => {
                                static_ref = true;
                            }
                            Err(reason) => {
                                match reason {
                                    Some(TrackedVariable::StoredConstant) => {
                                        static_ref = true;
                                    }
                                    Some(TrackedVariable::Imported) => {
                                        static_ref = true;
                                    }
                                    _ => { /* Skip */ }
                                }
                            }
                        }
                    } else {
                        let mut attribute_visitor =
                            ClientJsxExprTransformer::new(parent_visitor, false, false, true);
                        val.visit_mut_with(&mut attribute_visitor);
                    }
                    //
                }
                "children" => { /* Intentionally do nothing */ }
                // Standard expr transformation
                _ => {
                    let mut attribute_visitor =
                        ClientJsxExprTransformer::new(parent_visitor, false, true, true);
                    attribute_visitor.visit_custom_component(&mut val);
                    needs_getter_prop =
                        attribute_visitor.should_getter || val.is_call() || val.is_array();
                }
            }
            // Certain exprs need to be transformed
            if !static_ref && key.as_str() == REF_RAW {
                // No transformation done, ownership of expr moves back here
                if let Some(returned_val) =
                    // "Other tuple val returns '_use' usage not needed here"
                    create_ref_statements(&mut statements, &mut 0, None, val, false).0
                {
                    val = returned_val;
                } else {
                    return build_ref_expr(std::mem::take(&mut statements)).into();
                }
            }

            let mut must_use_getter = false;

            // IIFEs are optimized I guess
            let val = if !is_static {
                match grab_iife_mut(val) {
                    InnerIifeRes::InnerStmts(stmts) => {
                        let key = if cannot_convert_to_ident(&key) {
                            PropName::Computed(ComputedPropName {
                                span: DUMMY_SP,
                                expr: key.into(),
                            })
                        } else {
                            PropName::Ident(IdentName {
                                span: DUMMY_SP,
                                sym: key.into(),
                            })
                        };
                        let prop = Prop::Getter(GetterProp {
                            span: DUMMY_SP,
                            key,
                            type_ann: None,
                            body: Some(BlockStmt {
                                span: DUMMY_SP,
                                ctxt: SyntaxContext::empty(),
                                stmts: stmts,
                            }),
                        });
                        return PropOrSpread::Prop(Box::new(prop));
                    }
                    InnerIifeRes::InnerExpr(expr) => {
                        must_use_getter = true;
                        expr
                    }
                    InnerIifeRes::NoChange(expr) => expr,
                }
            } else {
                val
            };
            // Might be other edge cases than callexpr
            must_use_getter = must_use_getter
                || (key.as_str() == "children"
                    && (needs_children_getter || val.is_call() || val.is_array()))
                || (!is_static && needs_getter_prop);
            let key = if cannot_convert_to_ident(&key) {
                PropName::Computed(ComputedPropName {
                    span: DUMMY_SP,
                    expr: key.into(),
                })
            } else {
                PropName::Ident(IdentName {
                    span: DUMMY_SP,
                    sym: key.into(),
                })
            };

            let prop = if must_use_getter {
                Prop::Getter(GetterProp {
                    span: DUMMY_SP,
                    key,
                    type_ann: None,
                    body: Some(BlockStmt {
                        span: DUMMY_SP,
                        ctxt: SyntaxContext::empty(),
                        stmts: vec![Stmt::Return(ReturnStmt {
                            span: DUMMY_SP,
                            arg: Some(val),
                        })],
                    }),
                })
            } else {
                Prop::KeyValue(KeyValueProp { key, value: val })
            };
            PropOrSpread::Prop(Box::new(prop))
        })
        .collect();
    Box::new(Expr::Object(ObjectLit {
        span: DUMMY_SP,
        props,
    }))
}

struct ParsedProp {
    name: Atom,
    expr: Box<Expr>,
    is_static: bool,
}

enum ParsedPropsOrSpread {
    Props(Vec<ParsedProp>),
    Spread(Box<Expr>),
}

pub struct ClientCustomComponentBuilder<'a, T: ParentVisitor> {
    parent_visitor: &'a mut T,
    metadata: JsxCustomComponentMetadata<ClientJsxElementVisitor>,
    parsed_props: Vec<ParsedPropsOrSpread>,
    needs_merge_props: bool,
    needs_children_getter: bool,
}

impl<'a, T: ParentVisitor> ClientCustomComponentBuilder<'a, T> {
    pub fn new(
        parent_visitor: &'a mut T,
        metadata: JsxCustomComponentMetadata<ClientJsxElementVisitor>,
    ) -> Self {
        Self {
            parent_visitor,
            metadata,
            parsed_props: Vec::new(),
            needs_merge_props: false,
            needs_children_getter: false,
        }
    }

    fn parse_all_props(&mut self) {
        let mut parsed_prop_slice: Vec<ParsedProp> = Vec::new();
        for (maybe_key, wrapped_expression) in self.metadata.props.drain(..) {
            self.needs_merge_props = self.needs_merge_props || maybe_key.is_none();
            let (raw_expression, container_span) = grab_inner_expr(wrapped_expression);
            if let Some(key) = maybe_key {
                let is_static = if let Some(span_lo) = container_span {
                    self.parent_visitor.has_static_marker(span_lo)
                } else {
                    false
                };
                parsed_prop_slice.push(ParsedProp {
                    name: key,
                    expr: raw_expression,
                    is_static,
                });
            } else {
                if !parsed_prop_slice.is_empty() {
                    self.parsed_props
                        .push(ParsedPropsOrSpread::Props(parsed_prop_slice))
                }
                self.parsed_props
                    .push(ParsedPropsOrSpread::Spread(raw_expression));
                parsed_prop_slice = vec![];
            }
        }
        if !parsed_prop_slice.is_empty() {
            self.parsed_props
                .push(ParsedPropsOrSpread::Props(parsed_prop_slice))
        }

        if self.metadata.children.is_empty() {
            return;
        }

        let mut children: Vec<BuildResults> = self
            .metadata
            .children
            .drain(..)
            .map(|child| {
                let mut build = build_js_from_client_jsx(child, self.parent_visitor);
                if let BuildResults::Completed(c) = &mut build {
                    let mut child_visitor =
                        ClientJsxExprTransformer::new(self.parent_visitor, false, true, true);
                    child_visitor.visit_custom_component(c);
                    self.needs_children_getter =
                        self.needs_children_getter || child_visitor.should_getter;
                    // TODO -> Repeat on need revisit? May not be poss
                }
                build
            })
            .collect();

        let wrapped = if children.len() == 1 {
            standard_build_res_wrappings(children.pop().unwrap())
        } else {
            let arr = ArrayLit {
                span: DUMMY_SP,
                elems: children
                    .into_iter()
                    .map(|mut x| {
                        if memo_expr_mut(&mut x) {
                            self.parent_visitor.add_import(MEMO.into());
                        }
                        Some(standard_build_res_wrappings(x).into())
                    })
                    .collect(),
            };
            standard_build_res_wrappings(BuildResults::Arr(arr))
        };
        let new_entry = ParsedProp {
            name: "children".into(),
            expr: wrapped,
            is_static: false,
        };
        match self.parsed_props.last_mut() {
            Some(ParsedPropsOrSpread::Props(p)) => {
                p.push(new_entry);
            }
            // Either last el is spread or is empty
            // in either case we want a new el in arr
            _ => {
                self.parsed_props
                    .push(ParsedPropsOrSpread::Props(vec![new_entry]));
            }
        }
    }

    fn create_props_obj(&mut self) -> Box<Expr> {
        // Use mergeprops if necessary
        // build props object
        // make transformations -> specifically for ref
        let res = if self.needs_merge_props {
            let merge_props = generate_merge_props();
            self.parent_visitor.add_import(merge_props.as_str().into());
            let merge_props_expr = Expr::Call(CallExpr {
                span: DUMMY_SP,
                ctxt: SyntaxContext::empty(),
                callee: ident_callee(merge_props),
                type_args: None,
                args: self
                    .parsed_props
                    .drain(..)
                    .map(|parse| match parse {
                        ParsedPropsOrSpread::Props(items) => build_props_oject_expr(
                            self.parent_visitor,
                            self.needs_children_getter,
                            items,
                        )
                        .into(),
                        ParsedPropsOrSpread::Spread(mut expr) => {
                            transform_inner_expression_mut(self.parent_visitor, &mut expr, true);
                            expr.into()
                        }
                    })
                    .collect(),
            });
            Box::new(merge_props_expr)
        } else {
            // Should have length of 1
            match self.parsed_props.pop() {
                Some(ParsedPropsOrSpread::Props(items)) => {
                    build_props_oject_expr(self.parent_visitor, self.needs_children_getter, items)
                        .into()
                }
                Some(ParsedPropsOrSpread::Spread(mut expr)) => {
                    transform_inner_expression_mut(self.parent_visitor, &mut expr, true);
                    expr.into()
                }
                None => Box::new(Expr::Object(ObjectLit::default())),
            }
        };
        // TODO: ADD PROP BUILDER STUFF TO PARENT@
        res
    }

    pub fn build_and_wrap_custom_component(&mut self) -> Box<Expr> {
        self.parse_all_props();
        let args: Vec<ExprOrSpread> = vec![
            ident_expr(self.metadata.value.as_str().into()).into(),
            self.create_props_obj().into(),
        ];
        let create_component = generate_create_component_name();
        self.parent_visitor
            .add_import(create_component.as_str().into());
        self.needs_children_getter = false;
        Box::new(Expr::Call(CallExpr {
            span: DUMMY_SP,
            ctxt: SyntaxContext::empty(),
            callee: ident_callee(create_component),
            args,
            type_args: None,
        }))
    }
}

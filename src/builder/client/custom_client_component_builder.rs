use swc_core::{
    atoms::Atom,
    common::{util::take::Take, SyntaxContext, DUMMY_SP},
    ecma::{
        ast::{
            ArrayLit, BlockStmt, CallExpr, Expr, ExprOrSpread, Function, GetterProp, IdentName,
            JSXAttrOrSpread, JSXAttrValue, JSXExpr, KeyValueProp, Lit, MethodProp, ObjectLit,
            Param, Pat, Prop, PropName, PropOrSpread, ReturnStmt, Stmt,
        },
        visit::VisitMutWith,
    },
};

use crate::{
    builder::{
        client::{
            element_properties::ref_statement_builder::create_ref_statements,
            jsx_expr_builder_client::{
                build_js_from_client_jsx, standard_build_res_wrappings, BuildResults,
            },
            jsx_expr_transformer_client::ClientJsxExprTransformer,
            jsx_parser_client::ClientJsxElementVisitor,
        },
        parser_types::JsxCustomComponentMetadata,
    },
    helpers::{
        common_into_expressions::{ident_callee, ident_expr, ident_name},
        generate_var_names::{
            generate_create_component_name, generate_merge_props, generate_ref_raw, REF_RAW,
        },
    },
    transform::parent_visitor::ParentVisitor,
};

fn grab_inner_expr(raw: JSXAttrOrSpread) -> Box<Expr> {
    match raw {
        JSXAttrOrSpread::JSXAttr(jsxattr) => match jsxattr.value {
            Some(JSXAttrValue::Lit(l)) => Box::new(l.into()),
            Some(JSXAttrValue::JSXElement(e)) => Box::new(e.into()),
            Some(JSXAttrValue::JSXFragment(f)) => Box::new(f.into()),
            Some(JSXAttrValue::JSXExprContainer(c)) => match c.expr {
                JSXExpr::JSXEmptyExpr(_) => Box::new(Expr::dummy()),
                JSXExpr::Expr(expr) => expr,
            },
            None => Box::new(Expr::Lit(Lit::Bool(true.into()))),
        },
        JSXAttrOrSpread::SpreadElement(spread_element) => spread_element.expr,
    }
}

fn transform_inner_expression_mut<T: ParentVisitor>(
    parent_visitor: &mut T,
    to_visit: &mut Box<Expr>,
) -> bool {
    let mut attribute_visitor = ClientJsxExprTransformer::new(parent_visitor, true, true);
    attribute_visitor.visit_and_wrap_outer_expr(to_visit);
    attribute_visitor.needs_revisit
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

fn build_props_oject_expr(props: Vec<(Atom, Box<Expr>)>) -> Box<Expr> {
    let mut statements: Vec<Stmt> = vec![];
    let props = props
        .into_iter()
        .map(|(key, mut val)| {
            // Certain exprs need to be transformed
            if key.as_str() == REF_RAW {
                // No transformation done, ownership of expr moves back here
                if let Some(returned_val) =
                    // "Other tuple val returns '_use' usage not needed here"
                    create_ref_statements(&mut statements, &mut 0, None, val).0
                {
                    val = returned_val;
                } else {
                    return build_ref_expr(std::mem::take(&mut statements)).into();
                }
            }
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

enum ParsedPropsOrSpread {
    Props(Vec<(Atom, Box<Expr>)>),
    Spread(Box<Expr>),
}

pub struct ClientCustomComponentBuilder<'a, T: ParentVisitor> {
    parent_visitor: &'a mut T,
    metadata: JsxCustomComponentMetadata<ClientJsxElementVisitor>,
    parsed_props: Vec<ParsedPropsOrSpread>,
    needs_merge_props: bool,
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
        }
    }

    fn parse_all_props(&mut self) {
        let mut parsed_prop_slice: Vec<(Atom, Box<Expr>)> = Vec::new();
        for (maybe_key, wrapped_expression) in self.metadata.props.drain(..) {
            self.needs_merge_props = self.needs_merge_props || maybe_key.is_none();
            let mut raw_expression = grab_inner_expr(wrapped_expression);
            let mut needs_revisit =
                transform_inner_expression_mut(self.parent_visitor, &mut raw_expression);
            while needs_revisit {
                needs_revisit =
                    transform_inner_expression_mut(self.parent_visitor, &mut raw_expression);
            }
            if let Some(key) = maybe_key {
                parsed_prop_slice.push((key, raw_expression));
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

        let mut children: Vec<Box<Expr>> = self
            .metadata
            .children
            .drain(..)
            .map(|child| {
                let mut build = build_js_from_client_jsx(child, self.parent_visitor);
                if let BuildResults::Completed(c) = &mut build {
                    let mut child_visitor =
                        ClientJsxExprTransformer::new(self.parent_visitor, false, true);
                    c.visit_mut_with(&mut child_visitor);
                    // TODO -> Repeat on need revisit? May not be poss
                }
                standard_build_res_wrappings(build)
            })
            .collect();

        let wrapped = if children.len() == 1 {
            children.pop().unwrap()
        } else {
            let arr = ArrayLit {
                span: DUMMY_SP,
                elems: children.into_iter().map(|x| Some(x.into())).collect(),
            };
            standard_build_res_wrappings(BuildResults::Arr(arr))
        };
        let new_entry = ("children".into(), wrapped);
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
                        ParsedPropsOrSpread::Props(items) => build_props_oject_expr(items).into(),
                        ParsedPropsOrSpread::Spread(expr) => expr.into(),
                    })
                    .collect(),
            });
            Box::new(merge_props_expr)
        } else {
            // Should have length of 1
            match self.parsed_props.pop() {
                Some(ParsedPropsOrSpread::Props(items)) => build_props_oject_expr(items).into(),
                Some(ParsedPropsOrSpread::Spread(expr)) => expr.into(),
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
        Box::new(Expr::Call(CallExpr {
            span: DUMMY_SP,
            ctxt: SyntaxContext::empty(),
            callee: ident_callee(create_component),
            args,
            type_args: None,
        }))
    }
}

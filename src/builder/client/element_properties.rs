/*
 * JSX Opening Element can have a bunch of properties that have custom logic for how to build them.
 * Notably, properties like class/className, ref, and style all have custom logic
 * Additionally there is cusom logic for expressions and spread exprs
 * TODO: SOME CUSTOM COMPONENT LOGIC LIVES IN STD BUILDER BUT SHOULD PROBABLY BE MOVED HERE
 * SINCE OTHER LOGIC WILL LIVE HERE ANYWAY
 */

pub mod class_builder;
pub mod event_handler_builder;
pub mod ref_statement_builder;
pub mod standard_prop_builder;
pub mod style_builder;

use swc_core::{
    atoms::Atom,
    common::{SyntaxContext, DUMMY_SP},
    ecma::{
        ast::{
            AssignExpr, AssignTarget, BinExpr, CallExpr, Expr, ExprOrSpread, ExprStmt, Ident,
            JSXAttrOrSpread, JSXAttrValue, Lit, MemberExpr, SimpleAssignTarget, Stmt,
        },
        visit::VisitMutWith,
    },
};

use crate::{
    builder::{
        client::{
            effect_builder::{EffectBuilder, EffectMetadata},
            jsx_expr_builder_client::{build_js_from_client_jsx, standard_build_res_wrappings},
            jsx_expr_transformer_client::ClientJsxExprTransformer,
            jsx_parser_client::ClientJsxElementVisitor,
        },
        parser_types::JsxOpeningMetadata,
    },
    helpers::{
        common_into_expressions::{ident_callee, ident_expr},
        generate_var_names::{generate_use, generate_v},
    },
    transform::{
        parent_visitor::{self, ParentVisitor},
        scope_manager::TrackedVariable,
    },
};

type Effect = (Ident, MemberExpr);
pub enum PossibleEffectStatement {
    Std(Box<Expr>),
    Effect(Effect),
}

enum EffectOrInlineOrExpression {
    EffectRes((Box<Expr>, Effect)),
    InlineRes(String),
    ExpressionRes(Box<Expr>),
}

fn generate_effect_assignment(data: &Effect) -> Box<Expr> {
    let assign = AssignExpr {
        span: DUMMY_SP,
        op: swc_core::ecma::ast::AssignOp::Assign,
        left: AssignTarget::Simple(SimpleAssignTarget::Ident(data.0.clone().into())),
        right: data.1.clone().into(),
    };
    Box::new(assign.into())
}

fn generate_effect_statement(data: Effect, expr: Box<Expr>) -> Stmt {
    let expr = BinExpr {
        span: DUMMY_SP,
        op: swc_core::ecma::ast::BinaryOp::LogicalAnd,
        left: BinExpr {
            span: DUMMY_SP,
            op: swc_core::ecma::ast::BinaryOp::NotEqEq,
            left: data.0.into(),
            right: data.1.into(),
        }
        .into(),
        right: expr,
    };
    Stmt::Expr(ExprStmt {
        span: DUMMY_SP,
        expr: expr.into(),
    })
}

fn generate_use_expr(args: Vec<ExprOrSpread>) -> Box<Expr> {
    Box::new(Expr::Call(CallExpr {
        span: DUMMY_SP,
        ctxt: SyntaxContext::empty(),
        callee: ident_callee(generate_use()),
        args,
        type_args: None,
    }))
}

/*
 *
 * Used for opening elements on non-Custom components
 * ref implementation is partially used for custom compoennt
 * due to similar impelementation logic.
 *
 * Take a series of kv pairs with raw values,
 * transforms raw values into final exprs
 * generates a series of stmt values from those exprs
 * stmts are inserted as needed
 */

#[derive(Debug)]
pub struct ElementPropertiesBuilder<'a, T: ParentVisitor> {
    pub statements: Vec<Stmt>,
    pub direct_template_inserts: Vec<(String, String)>,
    pub parent_visitor: &'a mut T,
    effect_builder: &'a mut EffectBuilder,
    tmp_wrap_effect: bool,
}

// Implements many fns for different types of properties
// See 'element properties' folder
impl<'a, T: ParentVisitor> ElementPropertiesBuilder<'a, T> {
    fn effect_or_inline_or_expr(
        &mut self,
        data: PossibleEffectStatement,
    ) -> EffectOrInlineOrExpression {
        match data {
            PossibleEffectStatement::Std(expr) => match *expr {
                Expr::Lit(Lit::Str(s)) => {
                    EffectOrInlineOrExpression::InlineRes(s.value.as_str().into())
                }
                Expr::Lit(Lit::Num(n)) => {
                    EffectOrInlineOrExpression::InlineRes(n.value.to_string())
                }
                e => {
                    let inline = if let Some(ident) = e.as_ident() {
                        let mut val = &ident.sym;
                        loop {
                            match self.parent_visitor.get_var_if_in_scope(val) {
                                Some(TrackedVariable::FunctionIdent) => break None,
                                Some(TrackedVariable::Literal(l)) => break Some(l.clone()),
                                Some(TrackedVariable::Referred(r)) => {
                                    val = r;
                                }
                                None => break None,
                            }
                        }
                    } else {
                        None
                    };
                    if let Some(inline) = inline {
                        EffectOrInlineOrExpression::InlineRes(inline)
                    } else {
                        EffectOrInlineOrExpression::ExpressionRes(e.into())
                    }
                }
            },
            PossibleEffectStatement::Effect(e) => {
                EffectOrInlineOrExpression::EffectRes((generate_effect_assignment(&e), e))
            }
        }
    }

    pub fn new(parent_visitor: &'a mut T, effect_builder: &'a mut EffectBuilder) -> Self {
        Self {
            statements: vec![],
            direct_template_inserts: Vec::new(),
            parent_visitor,
            effect_builder,
            tmp_wrap_effect: false,
        }
    }

    pub fn std_transform(
        &mut self,
        attr: JSXAttrOrSpread,
        transform_call_exprs: bool,
    ) -> Box<Expr> {
        match attr {
            JSXAttrOrSpread::JSXAttr(jsxattr) => match jsxattr.value {
                Some(JSXAttrValue::JSXElement(mut jsxe)) => {
                    let mut sub_visitor = ClientJsxElementVisitor::new();
                    jsxe.visit_mut_with(&mut sub_visitor);
                    standard_build_res_wrappings(build_js_from_client_jsx(
                        sub_visitor,
                        self.parent_visitor,
                    ))
                }
                Some(JSXAttrValue::JSXFragment(mut jsxf)) => {
                    let mut sub_visitor = ClientJsxElementVisitor::new();
                    jsxf.visit_mut_with(&mut sub_visitor);
                    standard_build_res_wrappings(build_js_from_client_jsx(
                        sub_visitor,
                        self.parent_visitor,
                    ))
                }
                Some(JSXAttrValue::JSXExprContainer(exp)) => {
                    match exp.expr {
                        // Undefined behavior seemingly
                        swc_core::ecma::ast::JSXExpr::JSXEmptyExpr(_) => Box::new(Expr::default()),
                        swc_core::ecma::ast::JSXExpr::Expr(mut expr) => {
                            let mut sub_visitor = ClientJsxExprTransformer::new(
                                self.parent_visitor,
                                transform_call_exprs,
                                true,
                            );
                            if transform_call_exprs {
                                sub_visitor.visit_and_wrap_outer_expr(&mut expr);
                            } else {
                                expr.visit_mut_with(&mut sub_visitor);
                            }
                            self.tmp_wrap_effect = sub_visitor.should_wrap_in_effect;
                            expr
                        }
                    }
                }
                Some(JSXAttrValue::Lit(lit)) => Box::new(lit.into()),
                None => Box::new(Expr::Lit(Lit::Str("".into()))),
            },
            JSXAttrOrSpread::SpreadElement(spread_element) => spread_element.expr,
        }
    }

    pub fn handle_prop(name: Atom, data: JSXAttrOrSpread) {
        todo!();
    }

    pub fn build_effect_statement(&mut self) -> Option<Stmt> {
        let mut data = std::mem::take(&mut self.effect_builder.data);
        if data.is_empty() {
            return None;
        } else if data.len() == 1 {
            let raw = data.pop().unwrap();
            match raw.variant {
                super::effect_builder::EffectVariant::Class => {
                    self.class_builder(raw.count, PossibleEffectStatement::Std(raw.expr));
                }
                super::effect_builder::EffectVariant::Style => todo!(),
                super::effect_builder::EffectVariant::Std => {
                    self.standard_prop_builder(
                        raw.name,
                        raw.count,
                        PossibleEffectStatement::Std(raw.expr),
                    );
                }
            }
        } else {
            for datum in data {
                match datum.variant {
                    super::effect_builder::EffectVariant::Class => {
                        self.class_builder(
                            datum.count,
                            PossibleEffectStatement::Effect((datum.v_val, datum.obj)),
                        );
                    }
                    super::effect_builder::EffectVariant::Style => todo!(),
                    super::effect_builder::EffectVariant::Std => {
                        self.standard_prop_builder(
                            datum.name,
                            datum.count,
                            PossibleEffectStatement::Effect((datum.v_val, datum.obj)),
                        );
                    }
                }
            }
        }
        self.effect_builder
            .build_effect(std::mem::take(&mut self.statements))
    }

    // Needs to take opening metadata and parse it all out
    // Things to keep in mind
    // 1. Refs always apply
    // 2. spreads are weird. Causes a mergeprops but only affects props that come after
    // 3. some props get inlined into template
    pub fn build_el_property_statements(
        &mut self,
        data: JsxOpeningMetadata,
        element_count: usize,
    ) -> String {
        // More complex!
        if data.has_spread {
            let mut seen_first_spread = false;
            let mut merge_props_args: Vec<Box<Expr>> = vec![];
            /*
            let mut merge_props_args: Option<Vec<Box<Expr>>> = None;
            for (name, attr) in data.attrs {
                if let Some(name) = name {
                } else {
                    if let Some(merge_props_args) = &mut merge_props_args {
                        merge_props_args.push(self.std_transform(attr));
                    } else {
                        merge_props_args = Some(vec![self.std_transform(attr)])
                    }
                }
            }
            */
            todo!("")
        } else {
            for (name, attr) in data.attrs {
                match name
                    .expect("All non-spread properties should have a name")
                    .as_str()
                {
                    "class" | "className" => {
                        let expr = self.std_transform(attr, false);
                        if self.tmp_wrap_effect {
                            let assigned_v = self.parent_visitor.v_count();
                            let assigned_prop = self.effect_builder.get_new_obj();
                            self.effect_builder.data.push(EffectMetadata {
                                expr,
                                v_val: generate_v(*assigned_v).into(),
                                obj: assigned_prop,
                                variant: super::effect_builder::EffectVariant::Class,
                                count: element_count,
                                name: "class".into(),
                            });

                            *assigned_v += 1;
                            continue;
                        };
                        self.class_builder(element_count, PossibleEffectStatement::Std(expr));
                    }
                    "classList" => {
                        todo!("Implement class list object")
                    }
                    "style" => {
                        todo!("parse out individual styles")
                    }
                    "ref" => {
                        let expr = self.std_transform(attr, true);
                        self.ref_builder(Some(element_count), expr);
                    }
                    prop => {
                        let expr = self.std_transform(attr, true);
                        // Event handling
                        if prop.starts_with("on") {
                            self.event_handler_builder(prop, element_count, expr);
                        } else {
                            if self.tmp_wrap_effect {
                                let assigned_v = self.parent_visitor.v_count();
                                let assigned_prop = self.effect_builder.get_new_obj();
                                self.effect_builder.data.push(EffectMetadata {
                                    expr,
                                    v_val: generate_v(*assigned_v).into(),
                                    obj: assigned_prop,
                                    variant: super::effect_builder::EffectVariant::Std,
                                    count: element_count,
                                    name: prop.into(),
                                });

                                *assigned_v += 1;
                                continue;
                            };
                            self.standard_prop_builder(
                                prop.into(),
                                element_count,
                                PossibleEffectStatement::Std(expr),
                            );
                        }
                    }
                }
                self.tmp_wrap_effect = false;
            }
        }
        data.value
    }
}

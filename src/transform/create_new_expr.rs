// POTENTIAL TODO:
// THERE IS A LOT OF CLONING HERE THAT'S NOT NECESSARY
// COULD REFACTOR THIS TO WORK WITH MUT REFERENCES
// THAT WAY WE CAN RE-USE UNCAHNGED EXPRESSIONS INSTEAD OF CLONING THEM
use super::parent_visitor::ParentVisitor;
use crate::builder::jsx_builder::JsxBuilder;
use swc_core::{
    common::DUMMY_SP,
    ecma::ast::{ArrayLit, Expr, ExprOrSpread, JSXElementChild, JSXExpr, Lit, Str},
};

fn box_expr_to_expr_or_spread(expr: Box<Expr>) -> Option<ExprOrSpread> {
    Some(ExprOrSpread { spread: None, expr })
}
fn expr_to_expr_or_spread(expr: Expr) -> Option<ExprOrSpread> {
    box_expr_to_expr_or_spread(Box::new(expr))
}

// Could be more in the future
pub enum CreateNewExprError {
    NoChangeNeeded,
    ExprNotFound,
}
enum PossibleOption<T> {
    Raw(T),
    Wrapped(Option<T>),
}
// This result contins the new expr + a bool marking if the new expr needs to be traversed
// It will need to do that if there is some nested JSX that this fn will not parse out
type Res = Result<(Box<Expr>, bool), CreateNewExprError>;
fn create_new_expr_possible<T: ParentVisitor>(
    old_val: PossibleOption<&Box<Expr>>,
    attacher: &mut T,
) -> Res {
    let unwrapped = match old_val {
        PossibleOption::Raw(x) => x,
        PossibleOption::Wrapped(x) => x.ok_or(CreateNewExprError::ExprNotFound)?,
    };
    // Bizarre syntax and feels wrong, but does work :(
    match &**unwrapped {
        Expr::JSXElement(e) => Ok(e.visit_and_build_from_jsx(attacher)),
        Expr::JSXFragment(e) => {
            if e.children.len() == 1 {
                Ok(e.visit_and_build_from_jsx(attacher))
            } else {
                // Each element will be evaluated recursively
                let transformed_elems = e.children.iter().map(|el| {
                    match el {
                        JSXElementChild::JSXElement(jsx) => {
                            expr_to_expr_or_spread(Expr::JSXElement(jsx.clone()))
                        }
                        JSXElementChild::JSXFragment(frag) => {
                            expr_to_expr_or_spread(Expr::JSXFragment(frag.clone()))
                        }
                        JSXElementChild::JSXSpreadChild(spr) => {
                            // Add spread name
                            box_expr_to_expr_or_spread(spr.expr.clone())
                        }
                        JSXElementChild::JSXExprContainer(cont) => match cont.expr.clone() {
                            JSXExpr::Expr(e) => box_expr_to_expr_or_spread(e),
                            JSXExpr::JSXEmptyExpr(_) => None,
                        },
                        JSXElementChild::JSXText(txt) => {
                            let val = Str {
                                span: DUMMY_SP,
                                value: txt.value.as_str().trim().into(),
                                raw: None,
                            };
                            expr_to_expr_or_spread(Expr::Lit(Lit::Str(val)))
                        }
                    }
                });
                let arr = ArrayLit {
                    span: DUMMY_SP,
                    elems: transformed_elems.collect(),
                };
                Ok((Box::new(Expr::Array(arr)), true))
            }
        }
        Expr::Paren(e) => {
            // Returns an expr so ideall we would recurse into it to see if
            // we can find a JSX expression.
            // TODO - Usually the resulting expr would be wrapped in Paren
            // in this case we will want to piggy-back off of 'e'
            create_new_expr(&e.expr, attacher)
        }
        Expr::Array(arr_exp) => {
            let mut any_recurse = false;
            let transformed_arr = arr_exp.elems.clone().into_iter().map(|exp| {
                if let Some(expr_or_spread) = exp {
                    match create_new_expr(&expr_or_spread.expr, attacher) {
                        Ok((new_expr, needs_recurse)) => {
                            any_recurse = any_recurse || needs_recurse;
                            box_expr_to_expr_or_spread(new_expr)
                        }
                        Err(CreateNewExprError::NoChangeNeeded) => {
                            any_recurse = true;
                            Some(expr_or_spread)
                        }
                        Err(CreateNewExprError::ExprNotFound) => Some(expr_or_spread),
                    }
                } else {
                    None
                }
            });
            let arr = ArrayLit {
                span: DUMMY_SP,
                elems: transformed_arr.collect(),
            };
            let new_expr = Expr::Array(arr);
            Ok((Box::new(new_expr), any_recurse))
        }
        _ => {
            // The new expr is the old expr. Just return None;
            Err(CreateNewExprError::NoChangeNeeded)
        }
    }
}
pub fn create_new_expr_option<T: ParentVisitor>(
    old_val: Option<&Box<Expr>>,
    attacher: &mut T,
) -> Res {
    create_new_expr_possible(PossibleOption::Wrapped(old_val), attacher)
}
pub fn create_new_expr<T: ParentVisitor>(old_val: &Box<Expr>, attacher: &mut T) -> Res {
    create_new_expr_possible(PossibleOption::Raw(old_val), attacher)
}

use swc_core::atoms::Atom;

use crate::transform::{parent_visitor::ParentVisitor, scope_manager::TrackedVariable};

pub fn check_var_name_in_scope<T: ParentVisitor>(
    parent_visitor: &mut T,
    initial_val: &Atom,
) -> Result<String, Option<TrackedVariable>> {
    let mut val = initial_val;
    loop {
        match parent_visitor.get_var_if_in_scope(val) {
            Some(TrackedVariable::FunctionIdent(is_const)) => {
                break Err(Some(TrackedVariable::FunctionIdent(*is_const)))
            }
            Some(TrackedVariable::Literal(l)) => break Ok(l.clone()),
            Some(TrackedVariable::Referred(r)) => {
                val = r;
            }
            Some(TrackedVariable::StoredConstant) => {
                break Err(Some(TrackedVariable::StoredConstant))
            }
            Some(TrackedVariable::Imported) => break Err(Some(TrackedVariable::Imported)),
            None => break Err(None),
        }
    }
}

use swc_core::atoms::Atom;

const PATTERN: &str = "_$";
const TMPL: &str = "_tmpl$";
const TEMPLATE: &str = "_$template";
const EL: &str = "_el$";
const CREATE_COMPONENT: &str = "_$createComponent";
const INSERT: &str = "_$insert";

pub fn generate_template_name(id: usize) -> Atom {
    format!("{0}{1}", TMPL, id).into()
}
pub fn generate_create_component_name() -> Atom {
    CREATE_COMPONENT.into()
}
pub fn generate_insert() -> Atom {
    INSERT.into()
}
pub fn generate_el(el: usize) -> Atom {
    format!("{0}{1}", EL, el).into()
}
pub fn generate_template_expr_name() -> Atom {
    TEMPLATE.into()
}
pub fn generate_import_name(name: Atom) -> Atom {
    name.split(PATTERN).nth(1).unwrap_or("").into()
}

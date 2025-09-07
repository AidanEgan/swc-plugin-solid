use swc_core::atoms::Atom;

const TMPL: &str = "_tmpl$";
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

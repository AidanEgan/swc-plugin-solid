use swc_core::atoms::Atom;

// Variable values
const PATTERN: &str = "_$";
const TMPL: &str = "_tmpl$";
const EL: &str = "_el$";
const REF: &str = "_ref$";
const V: &str = "_v$";
const EFFECT_ARG: &str = "_p$";

// Imported values
const TEMPLATE: &str = "_$template";
const USE: &str = "_$use";
const MEMO: &str = "_&memo";
const CREATE_COMPONENT: &str = "_$createComponent";
const MERGE_PROPS: &str = "_$mergeProps";
const ADD_EVENT_LISTENER: &str = "_$addEventListener";
const INSERT: &str = "_$insert";
const SET_ATTRIBUTE: &str = "_$setAttribute";
const EFFECT: &str = "_$effect";
const CLASS_NAME: &str = "_$className";

// Raw values
pub const REF_RAW: &str = "ref";
pub const DELEGATE_EVENTS: &str = "_$delegateEvents";
pub const STYLE: &str = "_$style";
pub const SET_STYLE_PROPERTY: &str = "_$setStyleProperty";
pub const SPREAD: &str = "_$spread";
pub const CLASS_LIST: &str = "_$classList";

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
pub fn generate_v(v: usize) -> Atom {
    format!("{0}{1}", V, v).into()
}
pub fn generate_template_expr_name() -> Atom {
    TEMPLATE.into()
}
pub fn generate_import_name(name: Atom) -> Atom {
    name.split(PATTERN).nth(1).unwrap_or("").into()
}
pub fn generate_use() -> Atom {
    USE.into()
}
pub fn generate_ref(cnt: usize) -> Atom {
    format!("{0}{1}", REF, cnt).into()
}
pub fn generate_memo() -> Atom {
    MEMO.into()
}

pub fn generate_merge_props() -> Atom {
    MERGE_PROPS.into()
}

pub fn generate_ref_raw() -> Atom {
    REF_RAW.into()
}

pub fn generate_add_event_listener() -> Atom {
    ADD_EVENT_LISTENER.into()
}

pub fn generate_set_attribute() -> Atom {
    SET_ATTRIBUTE.into()
}

pub fn generate_effect() -> Atom {
    EFFECT.into()
}

pub fn generate_class_name() -> Atom {
    CLASS_NAME.into()
}

pub fn generate_effect_arg() -> Atom {
    EFFECT_ARG.into()
}

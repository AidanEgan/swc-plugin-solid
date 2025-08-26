use std::collections::HashSet;

// See: https://github.com/ryansolid/dom-expressions/blob/main/packages/babel-plugin-jsx-dom-expressions/src/config.ts
use serde::{Deserialize, Serialize};

fn as_true() -> bool {
    true
}

fn as_once() -> String {
    r#"@once"#.into()
}

fn as_dom() -> String {
    "dom".into()
}

fn as_effect() -> String {
    "effect".into()
}

fn as_memo() -> String {
    "memo".into()
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct PluginArgs {
    #[serde(default = "as_dom")]
    pub module_name: String,

    #[serde(default = "as_dom")]
    pub generate: String,

    #[serde(default)]
    pub hydratable: bool,

    #[serde(default = "as_true")]
    pub delegate_events: bool,

    #[serde(default)]
    pub delegated_events: HashSet<String>,

    #[serde(default)]
    pub built_ins: HashSet<String>,

    #[serde(default)]
    pub require_import_source: Option<String>,

    #[serde(default = "as_true")]
    pub wrap_conditionals: bool,

    #[serde(default)]
    pub omit_nested_closing_tags: bool,

    #[serde(default = "as_true")]
    pub omit_last_closing_tag: bool,

    #[serde(default = "as_true")]
    pub omit_quotes: bool,

    #[serde(default)]
    pub context_to_custom_elements: bool,

    #[serde(default = "as_once")]
    pub static_marker: String,

    #[serde(default = "as_effect")]
    pub effect_wrapper: String,

    #[serde(default = "as_memo")]
    pub memo_wrapper: String,

    #[serde(default)]
    pub validate: bool,
}

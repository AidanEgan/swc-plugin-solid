mod builder;
mod config;
mod constants;
mod helpers;
mod transform;
use config::PluginArgs;
use swc_core::{
    ecma::{ast::Program, visit::*},
    plugin::{plugin_transform, proxies::TransformPluginProgramMetadata},
};
use transform::create_solidjs_visitor;

#[plugin_transform]
pub fn process(mut program: Program, metadata: TransformPluginProgramMetadata) -> Program {
    let plugin_config = metadata.get_transform_plugin_config();
    let plugin_options: PluginArgs = if let Some(plugin_config) = plugin_config {
        serde_json::from_str(&plugin_config).unwrap_or_else(|f| {
            println!("Could not deserialize instrumentation option");
            println!("{f:#?}");
            Default::default()
        })
    } else {
        Default::default()
    };

    let mut visitor = create_solidjs_visitor(
        std::sync::Arc::new(metadata.source_map),
        metadata.comments.as_ref(),
        plugin_options,
    );

    program.visit_mut_with(&mut visitor);

    program
}

// Your test suite
#[cfg(test)]
mod tests {
    use std::{collections::HashSet, path::Path};

    use swc_core::ecma::parser::{EsSyntax, Syntax, TsSyntax};
    use swc_core::ecma::{
        transforms::testing::{test_fixture, FixtureTestConfig},
        visit::visit_mut_pass,
    };

    use crate::{config::PluginArgs, transform::create_solidjs_visitor};

    fn options() -> PluginArgs {
        PluginArgs {
            module_name: "solid-js/web".into(),
            generate: "dom".into(),
            hydratable: false,
            delegate_events: false,
            delegated_events: HashSet::new(),
            built_ins: HashSet::new(),
            require_import_source: None,
            wrap_conditionals: false,
            omit_nested_closing_tags: false,
            omit_last_closing_tag: false,
            omit_quotes: false,
            context_to_custom_elements: false,
            static_marker: "__@once__".into(),
            effect_wrapper: "effect".into(),
            memo_wrapper: "memo".into(),
            validate: false,
        }
    }

    fn get_syntax(ts: bool) -> Syntax {
        if !ts {
            Syntax::Es(EsSyntax {
                jsx: true,
                fn_bind: false,
                decorators: false,
                decorators_before_export: false,
                export_default_from: false,
                import_attributes: true,
                allow_super_outside_method: false,
                allow_return_outside_function: false,
                auto_accessors: true,
                explicit_resource_management: true,
            })
        } else {
            Syntax::Typescript(TsSyntax {
                tsx: true,
                decorators: false,
                dts: false,
                no_early_errors: false,
                disallow_ambiguous_jsx_like: false,
            })
        }
    }

    #[test]
    fn basic_template_test_js() {
        test_fixture(
            get_syntax(false),
            &|t| {
                visit_mut_pass(create_solidjs_visitor(
                    t.cm.clone(),
                    t.comments.clone(),
                    options(),
                ))
            },
            Path::new("tests/basic/input.jsx"),
            Path::new("tests/basic/output.js"),
            FixtureTestConfig {
                sourcemap: false,
                allow_error: false,
                module: Some(true),
            },
        );
    }

    #[test]
    fn basic_template_test_ts() {
        test_fixture(
            get_syntax(true),
            &|t| {
                visit_mut_pass(create_solidjs_visitor(
                    t.cm.clone(),
                    t.comments.clone(),
                    options(),
                ))
            },
            Path::new("tests/basic/input.tsx"),
            Path::new("tests/basic/output.ts"),
            FixtureTestConfig {
                sourcemap: false,
                allow_error: false,
                module: Some(true),
            },
        );
    }
}

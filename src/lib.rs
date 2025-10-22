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
            static_marker: "@once".into(),
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

    fn basic_test(input: &str, output: &str, is_ts: bool) {
        test_fixture(
            get_syntax(is_ts),
            &|t| {
                visit_mut_pass(create_solidjs_visitor(
                    t.cm.clone(),
                    t.comments.clone(),
                    options(),
                ))
            },
            Path::new(input),
            Path::new(output),
            FixtureTestConfig {
                sourcemap: false,
                allow_error: false,
                module: Some(true),
            },
        );
    }
    fn std_format_test(folder: &str, input_file: &str) {
        let is_ts = input_file.ends_with("tsx") || input_file.ends_with("ts");
        let input = format!("tests/{0}/{1}", folder, input_file);
        let output = format!(
            "tests/{0}/{1}_output.{2}",
            folder,
            input_file
                .split_at(input_file.find(".").expect("invalid file name"))
                .0,
            if is_ts { "ts" } else { "js" }
        );
        basic_test(input.as_str(), output.as_str(), is_ts);
    }

    #[test]
    fn basic_template_test_js() {
        basic_test("tests/basic/input.jsx", "tests/basic/output.js", false);
    }

    #[test]
    fn basic_template_test_ts() {
        basic_test("tests/basic/input.tsx", "tests/basic/output.ts", true);
    }

    #[test]
    fn basic_custom_component() {
        std_format_test("basic", "custom.jsx");
    }

    #[test]
    fn complex_custom_component() {
        std_format_test("basic", "custom_complex.jsx");
    }

    #[test]
    fn hello_world_component() {
        std_format_test("components", "hello_world.tsx");
    }

    #[test]
    fn hello_world_component_variant_one() {
        std_format_test("components", "hello_world_variant_one.tsx");
    }

    #[test]
    fn basic_ref_test() {
        std_format_test("components", "with_ref.tsx");
    }

    #[test]
    fn class_test() {
        std_format_test("components", "class_name_test.tsx");
    }

    #[test]
    fn style_test() {
        std_format_test("basic", "style.jsx");
    }

    #[test]
    fn class_list_test() {
        std_format_test("components", "class_list_test.tsx");
    }

    #[test]
    fn attribute_expressions_test() {
        // Individual tests help w/debugging
        // (1,10),(11,20)...(71,80)
        (0..=7)
            .map(|v| (v * 10 + 1, v * 10 + 10))
            .chain(vec![(81, 89)])
            .for_each(|(lb, ub)| {
                std_format_test("attribute_expressions", format!("{lb}_{ub}.jsx").as_str());
            });
        std_format_test("attribute_expressions", "code.jsx");
    }

    #[test]
    fn components_test() {
        std_format_test("components", "components_test.jsx");
    }
}

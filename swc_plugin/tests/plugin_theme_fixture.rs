use std::{fs, path::PathBuf};
use swc_core::ecma::{
    parser::{Syntax, TsSyntax},
    transforms::testing::{FixtureTestConfig, test_fixture},
    visit::visit_mut_pass,
};
use swc_core::testing::fixture;
use swc_plugin_debug::{PluginConfig, PluginTheme};
use swc_plugin_debug::file_logger::init_logger;

#[fixture("tests/fixture/plugin_theme/**/input.tsx")]
fn fixture(input: PathBuf) {
    init_logger();
    let output = input.with_file_name("output.js");

    // Read Json file
    let config_str = fs::read_to_string("tests/fixture/plugin_theme/config.json").expect("Failed to read config.json");

    let config: PluginConfig =
        serde_json::from_str(&config_str).expect("Failed to parse config.json");
    
    let filename = "signup/page.tsx".to_string();

    test_fixture(
        Syntax::Typescript(TsSyntax {
            tsx: true,
            ..Default::default()
        }),
        &move |_| visit_mut_pass(PluginTheme::new(config.clone(), filename.clone())),
        &input,
        &output,
        FixtureTestConfig::default(),
    );
}

use std::{fs, path::PathBuf};
use swc_core::ecma::{
    parser::{Syntax, TsSyntax},
    transforms::testing::{FixtureTestConfig, test_fixture},
    visit::visit_mut_pass,
};
use swc_core::testing::fixture;
use swc_plugin_debug::file_logger::init_logger;
use swc_plugin_debug::{plugin3::plugin3::Plugin3, plugin3_config::Plugin3Config};

#[fixture("tests/fixture/plugin3/**/input.tsx")]
fn fixture(input: PathBuf) {
    init_logger();
    let output = input.with_file_name("output.js");

    // Read Json file
    let config_str = fs::read_to_string("tests/fixture/plugin3/config.json")
        .expect("Failed to read config.json");
    print!("vistio");

    let config: Plugin3Config =
        serde_json::from_str(&config_str).expect("Failed to parse config.json");

    
    let filename = "signup/page.tsx".to_string();
    test_fixture(
        Syntax::Typescript(TsSyntax {
            tsx: true,
            ..Default::default()
        }),
        &move |_| visit_mut_pass(Plugin3::new(config.clone(), filename.clone())),
        &input,
        &output,
        FixtureTestConfig::default(),
    );
}

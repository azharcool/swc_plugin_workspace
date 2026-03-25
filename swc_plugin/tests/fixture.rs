mod helpers;

//
use std::{fs, path::PathBuf};
use swc_core::ecma::{
    parser::{Syntax, TsSyntax},
    transforms::testing::{FixtureTestConfig, test_fixture},
    visit::visit_mut_pass,
};
use swc_core::testing::fixture;
use swc_plugin::config::PluginConfig;
use swc_plugin::debug::file_logger::init_logger;
use log::debug;

//
use crate::helpers::plugin_process::PluginProcess;


#[derive(Debug, serde::Deserialize)]
struct FixtureMeta {
    filename: String,
}

#[fixture("tests/fixture/**/input.tsx")]
fn fixture(input: PathBuf) {
    init_logger();
    let output = input.with_file_name("output.js");

    // Read Json file
    let config_str = fs::read_to_string("tests/config.json").expect("Failed to read config.json");

    let config: PluginConfig =
        serde_json::from_str(&config_str).expect("Failed to parse config.json");

    let meta_path = input.with_file_name("fixture.json");

    let filename = if meta_path.exists() {
        let meta_str = fs::read_to_string(&meta_path).expect("Failed to read fixture.json");

        let meta_data: FixtureMeta = serde_json::from_str(&meta_str).expect("Failed to parse fixture.json");

        meta_data.filename
    } else {
        input.to_string_lossy().to_string()
    };
    
    debug!("logical filename: {}", filename);

    test_fixture(
        Syntax::Typescript(TsSyntax {
            tsx: true,
            ..Default::default()
        }),
        &mut |_| visit_mut_pass(PluginProcess::new(config.clone(), filename.to_string())),
        &input,
        &output,
        FixtureTestConfig::default(),
    );
}

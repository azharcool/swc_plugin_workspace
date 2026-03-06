use std::path::PathBuf;

use swc_core::ecma::{
    parser::{Syntax, TsSyntax},
    transforms::testing::{test_fixture, FixtureTestConfig},
    visit::visit_mut_pass,
};

use swc_core::testing::fixture;

//use swc_plugin::plugin1::MyPlugin;
use swc_plugin::MyPlugin2;

#[fixture("tests/fixture/**/input.tsx")]
fn fixture(input: PathBuf) {
    let output = input.with_file_name("output.js");

    test_fixture(
        Syntax::Typescript(TsSyntax {
            tsx: true,
            ..Default::default()
        }),
        &|_| visit_mut_pass(MyPlugin2),
        &input,
        &output,
        FixtureTestConfig::default(),
    );
}
use crate::config::PluginConfig;
use log::debug;
use swc_core::ecma::ast::Module;

pub fn run_pipeline(module: &mut Module, config: PluginConfig, filename: String) {
    debug!("Running pipeline for file: {}", filename);
    debug!("Plugin Config: {:?}", config);
    debug!("Initial AST: {:#?}", module);
    // Analyzer
    // State
    // Transformer
}

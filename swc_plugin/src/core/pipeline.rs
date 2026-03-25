use crate::debug;
use crate::transform::visitor::TransformVisitor;
use crate::{analyze::visitor::AnalyzeVisitor, config::PluginConfig};
use log::debug;
use swc_core::ecma::ast::Module;
use swc_core::ecma::visit::VisitMutWith;
use swc_core::ecma::visit::VisitWith;

pub fn run_pipeline(module: &mut Module, config: PluginConfig, filename: String) {
    debug!("Running pipeline for file: {}", filename);
    debug!("Initial AST: {:#?}", module);

    let mut found_theme_mapping = None;
    for theme_config in &config.theme_config {
        for theme_mapping in &theme_config.theme_mappings {
            if filename.ends_with(&theme_mapping.file) {
                found_theme_mapping = Some(theme_mapping.clone());
                // debug!("Found matching file in config: {:#?}", theme_mapping);
            }
        }
    }

    if found_theme_mapping.is_none() {
        debug!("No matching file found in config for: {}", filename);
        return;
    }
    
    // debug!("Starting Analyzer and Transform for file: {}", filename);

    // Analyzer
    let mut analyzer = AnalyzeVisitor::new(config.clone(), filename, found_theme_mapping.clone().unwrap());
    module.visit_with(&mut analyzer);
    

    // State
    debug!("Analyzer State afterV analysis: {:#?}", analyzer.state);

    // Transformer
    let mut transform = TransformVisitor::new(config.clone(), analyzer.state, found_theme_mapping.unwrap());
    module.visit_mut_with(&mut transform);
}

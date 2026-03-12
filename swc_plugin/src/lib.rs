pub mod file_logger;
pub mod plugin_config;
pub mod plugin_theme;

use swc_core::ecma::ast::*;
use swc_core::ecma::visit::VisitMutWith;
use swc_core::plugin::{
    metadata::TransformPluginMetadataContextKind, plugin_transform,
    proxies::TransformPluginProgramMetadata,
};
pub use plugin_config::PluginConfig;
pub use plugin_theme::PluginTheme;

#[plugin_transform]
pub fn process_program(mut program: Program, metadata: TransformPluginProgramMetadata) -> Program {
    let _ = file_logger::init_logger();
    let raw_config = metadata.get_transform_plugin_config();

    let plugin_config: PluginConfig = raw_config
        .as_deref()
        .and_then(|raw| serde_json::from_str(raw).ok())
        .unwrap_or_default();

    if let Some(filename) = metadata.get_context(&TransformPluginMetadataContextKind::Filename) {
        log::debug!("Processing file: {:?}", filename);
        let mut plugin_theme = PluginTheme::new(plugin_config, filename);
        program.visit_mut_with(&mut plugin_theme);
    }

    program
}

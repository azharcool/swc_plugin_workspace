pub mod analyze;
pub mod config;
pub mod core;
pub mod debug;
pub mod transform;

use swc_core::ecma::ast::*;
use swc_core::plugin::{
    metadata::TransformPluginMetadataContextKind, plugin_transform,
    proxies::TransformPluginProgramMetadata,
};

use crate::core::pipeline::run_pipeline;

#[plugin_transform]
pub fn process_program(mut program: Program, metadata: TransformPluginProgramMetadata) -> Program {
    let raw_config = metadata.get_transform_plugin_config();

    // let plugin_config: PluginConfig = match raw_config {
    //     Some(config) => {
    //         serde_json::from_str(&config).unwrap_or_else(|_| PluginConfig { debug: None })
    //     }
    //     None => PluginConfig { debug: None },
    // };
    let plugin_config = raw_config
        .as_deref()
        .and_then(|raw| serde_json::from_str(raw).ok())
        .unwrap_or_default();

    match &mut program {
        Program::Module(module) => {
            // Run Pipeline
            if let Some(filename) =
                metadata.get_context(&TransformPluginMetadataContextKind::Filename)
            {
                run_pipeline(module, plugin_config, filename);
            };
        }
        Program::Script(_script) => {
            // Optional
        }
    }

    program
}

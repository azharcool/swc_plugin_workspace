pub mod logger;
//pub mod plugin1;
pub mod config;
pub mod plugin2;

use swc_core::ecma::ast::*;
use swc_core::ecma::visit::VisitMutWith;
use swc_core::plugin::{
    metadata::TransformPluginMetadataContextKind, plugin_transform,
    proxies::TransformPluginProgramMetadata,
};

//pub use plugin1::MyPlugin;
use config::PluginConfig;
pub use plugin2::MyPlugin2;


#[plugin_transform]
pub fn process_program(mut program: Program, metadata: TransformPluginProgramMetadata) -> Program {
    //let mut plugin = MyPlugin;
    //program.visit_mut_with(&mut plugin);

    // Read config from next.config.js - { debug: true }
    // If no config is provided, default to PluginConfig { debug: false }
    let raw_config = metadata.get_transform_plugin_config();

    let config: PluginConfig = raw_config.as_deref().and_then(|raw| serde_json::from_str(raw).ok()).unwrap_or_default();
    

    if let Some(filename) = metadata.get_context(&TransformPluginMetadataContextKind::Filename) {
        if filename.ends_with("signup/page.tsx") {
            match &raw_config {
                Some(raw) => log_info!("Config", "Received: {}", raw),
                None => log_info!("Config", "No config provided, using defaults"),
            }
            if config.debug {
                log_info!("process_program", "Processing file: {}", filename);
            }

            let mut plugin2 = MyPlugin2::new(config.debug);
            program.visit_mut_with(&mut plugin2);

            if config.debug {
                log_info!("process_program", "Finished processing file: {}", filename);
            }
        }
    }

    program
}

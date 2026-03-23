use swc_core::ecma::{ast::Module, visit::VisitMut};
use swc_plugin::{config::PluginConfig, core::pipeline::run_pipeline};

pub struct PluginProcess {
    pub config: PluginConfig,
    pub filename: String,
}

impl PluginProcess {
    pub fn new(config: PluginConfig, filename: String) -> Self {
        Self { config, filename }
    }
}

impl VisitMut for PluginProcess {
    fn visit_mut_module(&mut self, module: &mut Module) {
        // Run Pipeline
        run_pipeline(module, self.config.clone(), self.filename.clone());
    }
}

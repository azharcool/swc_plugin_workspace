use swc_core::ecma::ast::{Module};
// Third-party imports
use log::debug;
use swc_core::ecma::visit::Visit;
use swc_core::ecma::visit::VisitWith;

// Local imports
use crate::analyze::state::AnalyzeState;
use crate::config::PluginConfig;
use crate::config::ThemeMapping;
use crate::config::models::DirectiveType;

// Define the AnalyzeVisitor struct
pub struct AnalyzeVisitor {
    pub config: PluginConfig,
    pub state: AnalyzeState,
    pub theme_mapping: ThemeMapping,
}

impl AnalyzeVisitor {
    pub fn new(config: PluginConfig, filename: String, theme_mapping: ThemeMapping) -> Self {
        Self {
            config,
            state: AnalyzeState::new(filename),
            theme_mapping,
        }
    }
}

// Implement the Visit trait for AnalyzeVisitor
impl Visit for AnalyzeVisitor {
    fn visit_program(&mut self, node: &swc_core::ecma::ast::Program) {
        debug!("Program Node Visited: {:#?}", node);
        node.visit_children_with(self);
    }
    fn visit_module(&mut self, node: &Module) {
        
        // USE CASE 2: Directive-Based Analysis
        match &self.theme_mapping.directive {
            DirectiveType::Server | DirectiveType::ServerOnly => {
                if let Some(resolver) = &self.config.theme_name_resolver {
                    self.state.file_directive = Some("server".to_string());
                    self.state.theme_resolver_config = Some(resolver.server.clone());
                    // debug!("Directive {:#?}", resolver.server);
                }
            },
            DirectiveType::Client | DirectiveType::ClientOnly => {
                if let Some(resolver) = &self.config.theme_name_resolver {
                    self.state.file_directive = Some("client".to_string());
                    self.state.theme_resolver_config = Some(resolver.client.clone());
                    // debug!("Directive {:#?}", resolver.client);
                }
            }
        };
        // debug!("Analyzing ThemeMapping: {:#?}", self.theme_mapping);

        node.visit_children_with(self);
    }
}

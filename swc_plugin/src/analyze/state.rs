use swc_core::ecma::ast::{Decl, ModuleItem};

use crate::config::models::ResolverConfig;

#[derive(Debug)]
pub struct AnalyzeState {
    pub filename: String,
    pub file_directive: Option<String>,

    pub theme_resolver_config: Option<ResolverConfig>,
    pub theme_resolve_import: Option<ModuleItem>,
    pub wrapper_component_variable_stmt: Option<Decl>,
    

    pub check_target_specifier: Option<String>,
    pub component_exists: Option<bool>,
    pub theme_imports: Option<Vec<ModuleItem>>,
}

impl AnalyzeState {
    pub fn new(filename: String) -> Self {
        Self {
            filename,
            file_directive: None,
            theme_resolver_config: None,
            theme_resolve_import: None,
            wrapper_component_variable_stmt: None,
            check_target_specifier: None,
            component_exists: None,
            theme_imports: None
        }
    }
}

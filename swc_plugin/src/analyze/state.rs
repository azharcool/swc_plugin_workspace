use swc_core::ecma::ast::*;

use crate::config::models::ResolverConfig;

#[derive(Debug)]
pub struct AnalyzeState {
    pub filename: String,
    pub file_directive: Option<String>,

    pub theme_resolver_config: Option<ResolverConfig>,
    pub theme_resolve_import: Option<ModuleItem>,
    pub wrapper_component_variable_stmt: Option<Decl>,
    pub theme_wrapper_component_fn_decl: Option<FnDecl>,
    pub theme_wrapper_component_if_stmts: Option<Vec<Stmt>>,

    pub check_target_specifier: Option<String>,
    pub component_exists: Option<bool>,
    pub theme_imports: Option<Vec<ModuleItem>>,
    pub theme_jsx_elements: Option<Vec<JSXElement>>,
    pub wrapper_component_variable_ident: Option<String>,
    pub target_jsx_element: Option<JSXElement>,

    pub theme_wrapper_component: Option<FnDecl>,

    pub theme_wrapper_ident: Option<Ident>,
    pub target_component_name: Option<String>,
    pub target_specifier_value_with_theme: Option<String>,
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
            theme_imports: None,
            theme_jsx_elements: None,
            target_jsx_element: None,
            theme_wrapper_component_fn_decl: None,
            theme_wrapper_component_if_stmts: None,
            wrapper_component_variable_ident: None,
            theme_wrapper_component: None,
            theme_wrapper_ident: None,
            target_component_name: None,
            target_specifier_value_with_theme: None,
        }
    }
}

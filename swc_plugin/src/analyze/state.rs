use crate::config::models::ResolverConfig;

#[derive(Debug)]
pub struct AnalyzeState {
    pub filename: String,
    pub file_directive: Option<String>,
    pub theme_resolver_config: Option<ResolverConfig>,
}

impl AnalyzeState {
    pub fn new(filename: String) -> Self {
        Self {
            filename,
            file_directive: None,
            theme_resolver_config: None,
        }
    }
}

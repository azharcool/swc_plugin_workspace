use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct PluginConfig {
    pub debug: Option<bool>,
}

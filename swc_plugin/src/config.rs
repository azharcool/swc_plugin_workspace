use serde::Deserialize;

#[derive(Deserialize, Default)]
pub struct PluginConfig {
    #[serde(default)] // defaults to false if not provided
    pub debug: bool,
}
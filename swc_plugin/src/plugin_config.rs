use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Default, Debug)]
pub struct PluginConfig {
    #[serde(default)]
    pub themes: HashMap<String, ThemeConfig>,

    #[serde(rename = "themeNameResolver")]
    pub theme_name_resolver: Option<ThemeNameResolver>,

    #[serde(default)] // defaults to false if not provided
    pub debug: bool,
}

#[derive(Deserialize, Debug)]
pub struct ThemeConfig {
    pub name: String,
    pub package: String,
    pub overrides: Vec<OverrideConfig>,
    pub description: String
}

#[derive(Deserialize, Debug)]
pub struct OverrideConfig {
    pub directive: DirectiveType,

    #[serde(rename = "type")]
    pub override_type: OverrideType,

    #[serde(rename = "importDeclaration")]
    pub import_declaration:ImportDeclarationConfig,
    pub target: Target,
    pub themes: Vec<Theme>
}

#[derive(Deserialize, Debug)]
pub struct ImportDeclarationConfig {
    pub source: String,
    pub specifier: Specifier,
}

#[derive(Deserialize, Debug)]
pub struct Theme {
    pub package: String,
    #[serde(rename = "type")]
    pub theme_type: OverrideType,
    pub directive: DirectiveType,
    #[serde(rename = "importDeclaration")]
    pub import_declaration: ImportDeclarationConfig
}

#[derive(Deserialize, Debug)]
pub struct Target {
    #[serde(rename = "type")]
    pub target_type: OverrideType,
    pub directive: DirectiveType,
    #[serde(rename = "importDeclaration")]
    pub import_declaration: ImportDeclarationConfig,
}

#[derive(Deserialize, Debug)]
pub struct ThemeNameResolver {
    pub server: ResolverConfig,
    pub client: ResolverConfig,
}

#[derive(Deserialize, Debug)]
pub struct ResolverConfig {
    pub directive: DirectiveType,
    #[serde(rename = "importDeclaration")]
    pub import_declaration: ImportDeclarationConfig,
}

#[derive(Deserialize, PartialEq, Debug, Clone)]
pub enum DirectiveType {
    #[serde(rename = "server")]
    Server,

    #[serde(rename = "client")]
    Client,

    #[serde(rename = "server-only")]
    ServerOnly,

    #[serde(rename = "client-only")]
    ClientOnly,
}

impl DirectiveType {
    pub fn as_str(&self) -> &'static str {
        match self {
            DirectiveType::Server => "server",
            DirectiveType::Client => "client",
            DirectiveType::ServerOnly => "server-only",
            DirectiveType::ClientOnly => "client-only",
        }
    }
}

#[derive(Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum OverrideType {
    Page,
    Component,
}

#[derive(Deserialize, Debug)]
pub struct Specifier {
    #[serde(rename = "type")]
    pub specifier_type: SpecifierType,
    pub name: String,
}

#[derive(Deserialize, PartialEq, Debug)]
pub enum SpecifierType {
    ImportSpecifier,
    ImportDefaultSpecifier,
}


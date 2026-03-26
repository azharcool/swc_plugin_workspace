use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct PluginConfig {
    #[serde(default)]
    pub debug: Option<bool>,

    #[serde(rename = "themeConfig", default)]
    pub theme_config: Vec<ThemeConfig>,

    #[serde(rename = "themeNameResolver")]
    pub theme_name_resolver: Option<ThemeNameResolver>,
}
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ThemeNameResolver {
    pub server: ResolverConfig,
    pub client: ResolverConfig,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ResolverConfig {
    pub directive: Directive,
    pub import: Import,
    pub variable: Variable,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Variable {
    pub kind: String,
    pub declarations: Vec<Declaration>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Declaration {
    pub name: String,
    pub nature: DeclarationNature,
    pub value: String,
    pub arguments: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum DeclarationNature {
    Await,
    Call,
    Literal,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Init {
    #[serde(rename = "type")]
    pub init_type: String,
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ThemeConfig {
    pub target: String,
    pub description: String,
    #[serde(rename = "themeMappings")]
    pub theme_mappings: Vec<ThemeMapping>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ThemeMapping {
    pub file: String,
    pub directive: Directive,
    pub targets: Vec<Target>,
    pub nature: TargetNature,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Target {
    pub nature: TargetNature,
    pub directive: Directive,
    #[serde(rename = "themeName")]
    pub theme_name: String,
    pub package: String,
    pub action: String,
    pub import: Import,
    pub themes: Vec<Theme>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Theme {
    pub nature: TargetNature,
    pub directive: Directive,
    #[serde(rename = "themeName")]
    pub theme_name: String,
    pub package: String,
    pub action: String,
    pub import: Import,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum Directive {
    #[serde(rename = "server")]
    Server,

    #[serde(rename = "client")]
    Client,

    #[serde(rename = "server-only")]
    ServerOnly,

    #[serde(rename = "client-only")]
    ClientOnly,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum TargetNature {
    Component,
    Agents,
    Page
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Import {
    pub source: String,
    pub specifiers: Vec<Specifier>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Specifier {
    pub nature: SpecifierNature,
    pub value: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum SpecifierNature {
    ImportSpecifier,
    ImportDefaultSpecifier,
    ImportNamespaceSpecifier,
}

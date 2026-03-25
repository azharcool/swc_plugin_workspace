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
    pub directive: DirectiveType,
    #[serde(rename = "importDeclaration")]
    pub import_declaration: ImportDeclaration,

    #[serde(rename = "variableDeclaration")]
    pub variable_declaration: VariableDeclaration,

    pub variable: Variable,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Variable {
    pub kind: String,
    pub declarations: Vec<Decl>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Decl {
    pub name: String,
    pub nature: Nature,
    pub value: String,
    pub arguments: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum Nature {
    Await,
    Call,
    Literal,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct VariableDeclaration {
    pub kind: String,
    pub declarations: Vec<Declaration>,
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Declaration {
    #[serde(rename = "type")]
    pub declaration_type: String,
    #[serde(rename = "id")]
    pub declaration_id: DeclarationId,
    pub init: Init,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DeclarationId {
    #[serde(rename = "type")]
    pub declaration_id_type: String,
    pub value: String,
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
    pub directive: DirectiveType,
    pub targets: Vec<Target>,
    #[serde(rename = "type")]
    pub theme_mapping_type: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Target {
    #[serde(rename = "type")]
    pub target_type: TargetType,
    pub directive: DirectiveType,
    #[serde(rename = "themeName")]
    pub theme_name: String,
    pub package: String,
    pub action: String,
    #[serde(rename = "importDeclaration")]
    pub import_declaration: ImportDeclaration,
    pub themes: Vec<Theme>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Theme {
    #[serde(rename = "type")]
    pub target_type: TargetType,
    pub directive: DirectiveType,
    #[serde(rename = "themeName")]
    pub theme_name: String,
    pub package: String,
    pub action: String,
    #[serde(rename = "importDeclaration")]
    pub import_declaration: ImportDeclaration,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
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

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum TargetType {
    Component,
    Agents,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct ImportDeclaration {
    pub source: String,
    pub specifiers: Vec<Specifier>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Specifier {
    #[serde(rename = "type")]
    pub specifier_type: SpecifierType,
    pub value: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum SpecifierType {
    ImportSpecifier,
    ImportDefaultSpecifier,
    ImportNamespaceSpecifier,
}

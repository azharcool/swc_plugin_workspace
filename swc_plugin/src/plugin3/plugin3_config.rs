use serde::Deserialize;

#[derive(Deserialize, Default, Debug, Clone)]
pub struct Plugin3Config {
    #[serde(rename = "themeNameResolver")]
    pub theme_name_resolver: Option<ThemeNameResolver>,

    #[serde(default)] // defaults to false if not provided
    pub debug: bool,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ThemeNameResolver {
    pub server: Option<ResolverConfig>,
    pub client: Option<ResolverConfig>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ResolverConfig {
    pub directive: Option<DirectiveType>,
    #[serde(rename = "importDeclaration")]
    pub import_declaration: Option<ImportDeclarationConfig>,
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

#[derive(Deserialize, Debug, Clone)]
pub struct ImportDeclarationConfig {
    pub source: String,
    pub specifier: Option<Specifier>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Specifier {
    #[serde(rename = "type")]
    pub specifier_type: SpecifierType,
    pub name: String,
}

#[derive(Deserialize, PartialEq, Debug, Clone)]
pub enum SpecifierType {
    ImportSpecifier,
    ImportDefaultSpecifier,
}






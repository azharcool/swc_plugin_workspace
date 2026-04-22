use crate::config::models::ThemeMapping;
use crate::{analyze::state::AnalyzeState, config::PluginConfig};
use log::debug;
use swc_core::ecma::ast::*;
use swc_core::ecma::visit::{VisitMut, VisitMutWith};

pub struct TransformVisitor {
    pub config: PluginConfig,
    pub state: AnalyzeState,
    pub theme_mapping: ThemeMapping,
}

impl TransformVisitor {
    pub fn new(config: PluginConfig, state: AnalyzeState, theme_mapping: ThemeMapping) -> Self {
        Self {
            config,
            state,
            theme_mapping,
        }
    }
    pub fn insert_import(&self, module: &mut Module, import: &ModuleItem) {
        let total_imports_length = module
            .body
            .iter()
            .rposition(|item| matches!(item, ModuleItem::ModuleDecl(ModuleDecl::Import(_))));
        if let Some(last_import_index) = total_imports_length {
            module.body.insert(last_import_index + 1, import.clone());
        }
    }

    pub fn replace_specifier_value(&mut self) {}
}

impl VisitMut for TransformVisitor {
    fn visit_mut_module(&mut self, node: &mut Module) {
        node.visit_mut_children_with(self);

        // Insert: import { getThemeCookieServer } from 'theme-resolver';
        if let Some(theme_resolve_import) = &self.state.theme_resolve_import {
            self.insert_import(node, theme_resolve_import);
        }

        // Remove: Original Target Import
        // target_component_name = SignUp
        // should Remove: import { SignUp } from "@znode/base-components/components/signup";
        if let Some(target_component_name) = &self.state.target_component_name {
            node.body.retain(|item| {
                if let ModuleItem::ModuleDecl(ModuleDecl::Import(import_decl)) = item {
                    let has_target_specifier =
                        import_decl
                            .specifiers
                            .iter()
                            .any(|specifier_item| match specifier_item {
                                ImportSpecifier::Named(named_specifier) => {
                                    named_specifier.local.sym == *target_component_name
                                }
                                ImportSpecifier::Default(default_specifier) => {
                                    default_specifier.local.sym == *target_component_name
                                }
                                ImportSpecifier::Namespace(namespace_specifier) => {
                                    namespace_specifier.local.sym == *target_component_name
                                }
                            });

                    return !has_target_specifier;
                }

                return true;
            })
        }

        // Insert: Import themes
        if let Some(theme_imports) = &self.state.theme_imports {
            for theme_import in theme_imports {
                self.insert_import(node, theme_import);
            }
        }

        // Insert: Theme Component Wrapper
        if let Some(theme_component_wrapper) = &self.state.theme_wrapper_component {
            node.body.insert(
                node.body.len(),
                ModuleItem::Stmt(Stmt::Decl(Decl::Fn(theme_component_wrapper.clone()))),
            );
        }
    }

    fn visit_mut_jsx_element(&mut self, node: &mut JSXElement) {
        node.visit_mut_children_with(self);

        // Replace: Target Component with Theme Component Wrapper Name
        if let (Some(target_component_name), Some(theme_wrapper_ident)) = (
            &self.state.target_component_name,
            &self.state.theme_wrapper_ident,
        ) {
            if let JSXElementName::Ident(ref mut opening_ident) = node.opening.name {
                // Already Replaced, Skip
                if opening_ident.sym == theme_wrapper_ident.sym  {
                    return;
                }

                // Check and Replace: Opening and Closing Tags
                if opening_ident.sym == target_component_name.as_str() {
                    // opening_ident.sym = theme_wrapper_component_name.clone().into();
                   node.opening.name = JSXElementName::Ident(theme_wrapper_ident.clone());

                    if let Some(closing) = &mut node.closing {
                        if let JSXElementName::Ident(_) = closing.name {
                            // closing_ident.sym = theme_wrapper_component_name.clone().into();
                            closing.name = JSXElementName::Ident(theme_wrapper_ident.clone());
                        }
                    }
                }
            }

            // self.replace_component_with_wrapper(module, target_component_name, theme_wrapper_component_name);
        }
    }
}

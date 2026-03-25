use swc_core::ecma::ast::*;
use swc_core::common::DUMMY_SP;
use swc_core::atoms::Atom;
use crate::config::models::{ImportDeclaration, SpecifierType};

pub fn build_import(decl: &ImportDeclaration) -> ModuleItem {
    let specifiers: Vec<ImportSpecifier> = decl.specifiers
        .iter()
        .map(|s| match s.specifier_type {
            // import Value from "source"
            SpecifierType::ImportDefaultSpecifier => {
                ImportSpecifier::Default(ImportDefaultSpecifier {
                    span: DUMMY_SP,
                    local: Ident::new(Atom::from(s.value.as_str()), DUMMY_SP, Default::default()),
                })
            }
            // import { Value } from "source"
            SpecifierType::ImportSpecifier => {
                ImportSpecifier::Named(ImportNamedSpecifier {
                    span: DUMMY_SP,
                    local: Ident::new(Atom::from(s.value.as_str()), DUMMY_SP, Default::default()),
                    imported: None,
                    is_type_only: false,
                })
            }
        })
        .collect();

    ModuleItem::ModuleDecl(ModuleDecl::Import(ImportDecl {
        span: DUMMY_SP,
        specifiers,
        src: Box::new(Str {
            span: DUMMY_SP,
            value: Atom::from(decl.source.as_str()),
            raw: None,
        }),
        type_only: false,
        with: None,
        phase: Default::default(),
    }))
}
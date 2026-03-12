use crate::plugin_config::{PluginConfig, OverrideConfig, SpecifierType};
use log::debug;

use swc_core::common::{sync::Lrc, DUMMY_SP, SourceMap, SyntaxContext};
use swc_core::ecma::{
    ast::*,
    visit::{VisitMut, VisitMutWith},
    codegen::{text_writer::JsWriter, Config, Emitter},
};

pub struct PluginTheme<'a> {
    pub config: &'a PluginConfig,
    pub filename: String,
    pub debug: bool,
}

impl<'a> PluginTheme<'a> {
    pub fn new(config: &'a PluginConfig, filename: String, debug: bool) -> Self {
        Self { config, filename, debug }
    }

    fn capitalize(s: &str) -> String {
        let mut chars = s.chars();
        match chars.next() {
            None => String::new(),
            Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        }
    }

    fn module_to_code(&self, module: &Module) -> String {
        let cm: Lrc<SourceMap> = Default::default();
        let mut buf = vec![];

        {
            let wr = JsWriter::new(cm.clone(), "\n", &mut buf, None);
            let mut emitter = Emitter {
                cfg: Config::default(),
                cm: cm.clone(),
                comments: None,
                wr,
            };

            emitter.emit_module(module).unwrap();
        }

        String::from_utf8(buf).unwrap_or_default()
    }

    fn apply_override(&self, module: &mut Module, override_config: &OverrideConfig, base_name: &str) {
        let filename = &self.filename;

        if !override_config.import_declaration.source.ends_with(filename) {
            return;
        }

        let target_source = &override_config.target.import_declaration.source;
        let base_prefix = Self::capitalize(base_name);

        let mut new_imports: Vec<ModuleItem> = vec![];

        for item in &mut module.body {
            if let ModuleItem::ModuleDecl(ModuleDecl::Import(import)) = item {
                if import.src.value == target_source.as_str() {

                    for spec in &mut import.specifiers {
                        match spec {
                            ImportSpecifier::Named(named) => {
                                let spec_name = named.local.sym.to_string();
                                let new_name = format!("{}{}", base_prefix, spec_name);

                                named.local.sym = new_name.clone().into();
                            }

                            ImportSpecifier::Default(default) => {
                                let spec_name = default.local.sym.to_string();
                                let new_name = format!("{}{}", base_prefix, spec_name);

                                default.local.sym = new_name.clone().into();
                            }

                            _ => {}
                        }
                    }

                    for theme in &override_config.themes {

                        let spec_name = &theme.import_declaration.specifier.name;

                        let component_name =
                            format!("{}{}", Self::capitalize(&theme.package), spec_name);

                        let specifier = match theme.import_declaration.specifier.specifier_type {
                            SpecifierType::ImportSpecifier => {
                                ImportSpecifier::Named(ImportNamedSpecifier {
                                    span: DUMMY_SP,
                                    local: Ident::new(
                                        component_name.clone().into(),
                                        DUMMY_SP,
                                        SyntaxContext::empty(),
                                    ),
                                    imported: Some(ModuleExportName::Ident(
                                        Ident::new(
                                            spec_name.clone().into(),
                                            DUMMY_SP,
                                            SyntaxContext::empty(),
                                        ),
                                    )),
                                    is_type_only: false,
                                })
                            }

                            SpecifierType::ImportDefaultSpecifier => {
                                ImportSpecifier::Default(ImportDefaultSpecifier {
                                    span: DUMMY_SP,
                                    local: Ident::new(
                                        component_name.clone().into(),
                                        DUMMY_SP,
                                        SyntaxContext::empty(),
                                    ),
                                })
                            }
                        };

                        let new_import = ModuleItem::ModuleDecl(ModuleDecl::Import(ImportDecl {
                            span: DUMMY_SP,
                            specifiers: vec![specifier],
                            src: Box::new(Str {
                                span: DUMMY_SP,
                                value: theme.import_declaration.source.clone().into(),
                                raw: None,
                            }),
                            type_only: false,
                            with: None,
                            phase: Default::default(),
                        }));

                        new_imports.push(new_import);
                    }
                }
            }
        }

        for new_import in new_imports {
            module.body.insert(0, new_import);
        }
    }

    fn inject_theme_cookie(&self, module: &mut Module) {
        let stmt = Stmt::Decl(Decl::Var(Box::new(VarDecl {
            span: DUMMY_SP,
            ctxt: SyntaxContext::empty(),
            kind: VarDeclKind::Const,
            declare: false,
            decls: vec![VarDeclarator {
                span: DUMMY_SP,
                name: Pat::Ident(BindingIdent {
                    id: Ident::new("themeName".into(), DUMMY_SP, SyntaxContext::empty()),
                    type_ann: None,
                }),
                init: Some(Box::new(Expr::Await(AwaitExpr {
                    span: DUMMY_SP,
                    arg: Box::new(Expr::Call(CallExpr {
                        span: DUMMY_SP,
                        ctxt: SyntaxContext::empty(),
                        callee: Callee::Expr(Box::new(Expr::Ident(
                            Ident::new("getThemeCookieServer".into(), DUMMY_SP, SyntaxContext::empty()),
                        ))),
                        args: vec![],
                        type_args: None,
                    })),
                }))),
                definite: false,
            }],
        })));

        module.body.insert(0, ModuleItem::Stmt(stmt));
    }
}

impl<'a> VisitMut for PluginTheme<'a> {

    fn visit_mut_module(&mut self, module: &mut Module) {

        let before = if self.debug {
            Some(self.module_to_code(module))
        } else {
            None
        };

        if let Some(base_theme) = self.config.themes.get("base") {
            let base_name = &base_theme.name;

            for override_item in &base_theme.overrides {
                self.apply_override(module, override_item, base_name);
            }

            self.inject_theme_cookie(module);
        }

        module.visit_mut_children_with(self);

        if self.debug {
            if let Some(before_code) = before {
                let after = self.module_to_code(module);

                debug!(
                    "======= BEFORE =======\n{}\n======= AFTER =======\n{}",
                    before_code, after
                );
            }
        }
    }

    fn visit_mut_jsx_element(&mut self, jsx: &mut JSXElement) {

        let mut new_children = vec![];

        for child in jsx.children.drain(..) {

            match child {

                JSXElementChild::JSXElement(el) => {

                    if let JSXElementName::Ident(ident) = &el.opening.name {

                        if let Some(base_theme) = self.config.themes.get("base") {

                            for override_config in &base_theme.overrides {

                                let spec_name =
                                    &override_config.import_declaration.specifier.name;

                                if ident.sym == *spec_name {

                                    let base_component =
                                        format!("{}{}", Self::capitalize(&base_theme.name), spec_name);

                                    let mut cond_expr = Expr::JSXElement(Box::new(JSXElement {
                                        span: DUMMY_SP,
                                        opening: JSXOpeningElement {
                                            name: JSXElementName::Ident(
                                                Ident::new(
                                                    base_component.into(),
                                                    DUMMY_SP,
                                                    SyntaxContext::empty(),
                                                ),
                                            ),
                                            ..el.opening.clone()
                                        },
                                        children: el.children.clone(),
                                        closing: el.closing.clone(),
                                    }));

                                    for theme in &override_config.themes {

                                        let theme_component =
                                            format!("{}{}", Self::capitalize(&theme.package), spec_name);

                                        cond_expr = Expr::Cond(CondExpr {
                                            span: DUMMY_SP,
                                            test: Box::new(Expr::Bin(BinExpr {
                                                span: DUMMY_SP,
                                                op: BinaryOp::EqEqEq,
                                                left: Box::new(Expr::Ident(
                                                    Ident::new(
                                                        "themeName".into(),
                                                        DUMMY_SP,
                                                        SyntaxContext::empty(),
                                                    ),
                                                )),
                                                right: Box::new(Expr::Lit(Lit::Str(Str {
                                                    span: DUMMY_SP,
                                                    value: theme.package.clone().into(),
                                                    raw: None,
                                                }))),
                                            })),
                                            cons: Box::new(Expr::JSXElement(Box::new(JSXElement {
                                                span: DUMMY_SP,
                                                opening: JSXOpeningElement {
                                                    name: JSXElementName::Ident(
                                                        Ident::new(
                                                            theme_component.into(),
                                                            DUMMY_SP,
                                                            SyntaxContext::empty(),
                                                        ),
                                                    ),
                                                    ..el.opening.clone()
                                                },
                                                children: el.children.clone(),
                                                closing: el.closing.clone(),
                                            }))),
                                            alt: Box::new(cond_expr),
                                        });
                                    }

                                    new_children.push(
                                        JSXElementChild::JSXExprContainer(
                                            JSXExprContainer {
                                                span: DUMMY_SP,
                                                expr: JSXExpr::Expr(Box::new(cond_expr)),
                                            },
                                        ),
                                    );

                                    continue;
                                }
                            }
                        }
                    }

                    new_children.push(JSXElementChild::JSXElement(el));
                }

                other => new_children.push(other),
            }
        }

        jsx.children = new_children;

        jsx.visit_mut_children_with(self);
    }
}
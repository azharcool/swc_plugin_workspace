use swc_core::common::DUMMY_SP;
use swc_core::common::SyntaxContext;
use swc_core::ecma::ast::Module;
// Third-party imports
use log::debug;
use swc_core::ecma::ast::*;
use swc_core::ecma::visit::Visit;
use swc_core::ecma::visit::VisitWith;

// Local imports
use crate::analyze::state::AnalyzeState;
use crate::config::DeclarationNature;
use crate::config::Import;
use crate::config::PluginConfig;
use crate::config::Specifier;
use crate::config::SpecifierNature;
use crate::config::TargetNature;
use crate::config::ThemeMapping;
use crate::config::Variable;
use crate::config::models::Directive;

// Define the AnalyzeVisitor struct
pub struct AnalyzeVisitor {
    pub config: PluginConfig,
    pub state: AnalyzeState,
    pub theme_mapping: ThemeMapping,
}

impl AnalyzeVisitor {
    pub fn new(config: PluginConfig, filename: String, theme_mapping: ThemeMapping) -> Self {
        Self {
            config,
            state: AnalyzeState::new(filename),
            theme_mapping,
        }
    }

    pub fn create_import_specifiers(
        &self,
        specifiers: &Vec<Specifier>,
        theme_name: Option<&str>,
    ) -> Vec<ImportSpecifier> {
        let mut ast_specifier: Vec<ImportSpecifier> = vec![];

        for specifier in specifiers {
            let specifier_type = &specifier.nature;
            let specifier_value = &specifier.value;

            let mut local_ident = Ident {
                span: DUMMY_SP,
                sym: specifier_value.clone().into(),
                optional: false,
                ctxt: SyntaxContext::empty(),
            };

            let mut imported_ident = None;

            if let Some(theme_name) = theme_name {
                imported_ident = Some(local_ident.clone().into());
                local_ident = Ident {
                    span: DUMMY_SP,
                    sym: format!("{}__{}", specifier_value, theme_name).into(),
                    optional: false,
                    ctxt: SyntaxContext::empty(),
                };
            }
            ast_specifier.push(match specifier_type {
                SpecifierNature::ImportSpecifier => ImportSpecifier::Named(ImportNamedSpecifier {
                    span: DUMMY_SP,
                    local: local_ident,
                    imported: imported_ident,
                    is_type_only: false,
                }),
                SpecifierNature::ImportDefaultSpecifier => {
                    ImportSpecifier::Default(ImportDefaultSpecifier {
                        span: DUMMY_SP,
                        local: local_ident,
                    })
                }
                SpecifierNature::ImportNamespaceSpecifier => {
                    ImportSpecifier::Namespace(ImportStarAsSpecifier {
                        span: DUMMY_SP,
                        local: local_ident,
                    })
                }
            });
        }

        return ast_specifier;
    }

    pub fn create_import_decl(
        &self,
        import_declaration: &Import,
        theme_name: Option<&str>,
    ) -> ModuleItem {
        let source = &import_declaration.source;

        let specifiers = self.create_import_specifiers(&import_declaration.specifiers, theme_name);

        let src = Box::new(Str {
            span: DUMMY_SP,
            value: source.clone().into(),
            raw: Some(format!("\"{}\"", source).into()),
        });

        let import_decl = ImportDecl {
            span: DUMMY_SP,
            specifiers,
            src,
            type_only: false,
            with: None,
            phase: Default::default(),
        };

        let module_item = ModuleItem::ModuleDecl(ModuleDecl::Import(import_decl));

        return module_item;
    }

    pub fn import_exists(&self, module: &Module, import_declaration: &Import) -> bool {
        let source = &import_declaration.source;
        module.body.iter().any(|item| {
        matches!(item, ModuleItem::ModuleDecl(ModuleDecl::Import(decl)) if decl.src.value.to_string_lossy() == *source)
      })
    }

    pub fn create_variable_decl(&self, variable: &Variable) -> Decl {
        let kind = &variable.kind;
        let declarations = &variable.declarations;

        let mut ast_declarations: Vec<VarDeclarator> = vec![];

        for declaration in declarations {
            // Input
            let name = &declaration.name;
            let nature = &declaration.nature; // Await, Call, Literal
            let value = &declaration.value;
            let arguments = &declaration.arguments; // [id, ...args]

            // e.g. const themeName = await getThemeCookieServer();

            // getThemeCookieServer
            let ident = Ident {
                span: DUMMY_SP,
                sym: value.clone().into(),
                optional: false,
                ctxt: SyntaxContext::empty(),
            };

            // Convert to Expression
            let expr_ident = Expr::Ident(ident);

            let expr: Expr = match nature {
                DeclarationNature::Await => {
                    // getThemeCookieServer()
                    let call_expr = Expr::Call(CallExpr {
                        span: DUMMY_SP,
                        ctxt: SyntaxContext::empty(),
                        callee: Callee::Expr(Box::new(expr_ident)),
                        args: vec![],
                        type_args: None,
                    });

                    // await getThemeCookieServer()
                    Expr::Await(AwaitExpr {
                        span: DUMMY_SP,
                        arg: Box::new(call_expr),
                    })
                }
                DeclarationNature::Call => {
                    // getThemeCookieServer()
                    Expr::Call(CallExpr {
                        span: DUMMY_SP,
                        ctxt: SyntaxContext::empty(),
                        callee: Callee::Expr(Box::new(expr_ident)),
                        args: arguments
                            .iter()
                            .map(|arg| ExprOrSpread {
                                spread: None,
                                expr: Box::new(Expr::Lit(Lit::Str(Str {
                                    span: DUMMY_SP,
                                    value: arg.clone().into(),
                                    raw: Some(format!("\"{}\"", arg).into()),
                                }))),
                            })
                            .collect(),
                        type_args: None,
                    })
                }
                DeclarationNature::Literal => Expr::Lit(Lit::Str(Str {
                    span: DUMMY_SP,
                    value: value.clone().into(),
                    raw: Some(format!("\"{}\"", value).into()),
                })),
            };

            let init = Some(Box::new(expr));

            let ast_name: Pat = Pat::Ident(BindingIdent {
                id: Ident {
                    span: DUMMY_SP,
                    sym: name.clone().into(),
                    optional: false,
                    ctxt: SyntaxContext::empty(),
                },
                type_ann: None,
            });

            let ast_variable: VarDeclarator = VarDeclarator {
                span: DUMMY_SP,
                name: ast_name,
                init: init,
                definite: false,
            };

            ast_declarations.push(ast_variable);
        }

        let decl: Decl = Decl::Var(Box::new(VarDecl {
            kind: match kind.as_str() {
                "var" => VarDeclKind::Var,
                "let" => VarDeclKind::Let,
                "const" => VarDeclKind::Const,
                _ => VarDeclKind::Var, // Default to var if kind is unrecognized
            },
            decls: ast_declarations,
            span: DUMMY_SP,
            declare: false,
            ctxt: SyntaxContext::empty(),
        }));

        return decl;
    }
}

impl Visit for AnalyzeVisitor {
    fn visit_module(&mut self, node: &Module) {
        // USE CASE 2: Directive-Based Analysis
        match &self.theme_mapping.directive {
            Directive::Server | Directive::ServerOnly => {
                if let Some(resolver) = &self.config.theme_name_resolver {
                    self.state.file_directive = Some("server".to_string());
                    self.state.theme_resolver_config = Some(resolver.server.clone());

                    // create import statement and store in state
                    let import = resolver.server.import.clone();
                    let import_exists = self.import_exists(node, &import);
                    if !import_exists {
                        let import_module_item = self.create_import_decl(&import, None);
                        self.state.theme_resolve_import = Some(import_module_item);
                    }

                    // create variable statement and store in state
                    let variable_stmt = resolver.server.variable.clone();
                    let variable_decl = self.create_variable_decl(&variable_stmt);
                    self.state.wrapper_component_variable_stmt = Some(variable_decl);
                }
            }
            Directive::Client | Directive::ClientOnly => {
                if let Some(resolver) = &self.config.theme_name_resolver {
                    self.state.file_directive = Some("client".to_string());
                    self.state.theme_resolver_config = Some(resolver.client.clone());

                    // create import statement and store in state
                    let import = resolver.client.import.clone();
                    let import_exists = self.import_exists(node, &import);
                    if !import_exists {
                        let import_module_item = self.create_import_decl(&import, None);
                        self.state.theme_resolve_import = Some(import_module_item);
                    }

                    // create variable statement and store in state
                    let variable_stmt = resolver.client.variable.clone();
                    let variable_decl = self.create_variable_decl(&variable_stmt);
                    self.state.wrapper_component_variable_stmt = Some(variable_decl);
                }
            }
        };

        // Get Target Themes Mapping
        let targets = &self.theme_mapping.targets;

        for target in targets {
            let target_nature = &target.nature;

            match target_nature {
                TargetNature::Component => {
                    debug!("Found target component: {:#?}", target);

                    // store target specifier value in state for later use when visiting import declarations
                    let target_import = target.import.clone();
                    let target_specifier = target_import.specifiers.first().unwrap();
                    self.state.check_target_specifier = Some(target_specifier.value.clone());

                    // Create ThemeComponent Wrapper

                    // loops themes and create import statements for each theme, store in state
                    let target_themes = &target.themes;
                    for theme in target_themes {
                        let theme_import = theme.import.clone();
                        let theme_name = theme.theme_name.clone();
                        let import_module_item =
                            self.create_import_decl(&theme_import, Some(&theme_name));
                        self.state
                            .theme_imports
                            .get_or_insert_with(Vec::new)
                            .push(import_module_item);
                    }
                }

                TargetNature::Agents => {
                    debug!("Found target agents: {:#?}", target);
                }

                TargetNature::Page => {
                    debug!("Found target page: {:#?}", target);
                }
            }
        }

        node.visit_children_with(self);
    }

    fn visit_import_decl(&mut self, node: &ImportDecl) {
        node.specifiers.iter().for_each(|item| match item {
            ImportSpecifier::Named(named) => {
                if let Some(check_target_specifier) = &self.state.check_target_specifier {
                    if named.local.sym.to_string() == *check_target_specifier {
                        self.state.component_exists = Some(true);
                    }
                }
            }
            ImportSpecifier::Default(default) => {
                if let Some(check_target_specifier) = &self.state.check_target_specifier {
                    if default.local.sym.to_string() == *check_target_specifier {
                        self.state.component_exists = Some(true);
                    }
                }
            }
            ImportSpecifier::Namespace(namespace) => {
                if let Some(check_target_specifier) = &self.state.check_target_specifier {
                    if namespace.local.sym.to_string() == *check_target_specifier {
                        self.state.component_exists = Some(true);
                    }
                }
            }
        });
        node.visit_children_with(self);
    }
    

    fn visit_fn_expr(&mut self, node: &FnExpr) {
        debug!("Visiting function expression: {:#?}", node);
        
        if let Some(body) = &node.function.body {
            body.stmts.iter().for_each(|item| {
                debug!("Function body item: {:#?}", item);

                if let Stmt::Return(return_stmt) = item {
                    debug!("Found return statement: {:#?}", return_stmt);
                }
            });
        }

        // Get Signup jsx 
        // Get Signup 
        node.visit_children_with(self);
    }
}

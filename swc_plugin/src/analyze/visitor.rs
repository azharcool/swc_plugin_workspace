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

    pub fn create_variable_decl(&mut self, variable: &Variable) -> Decl {
        let kind = &variable.kind;
        let declarations = &variable.declarations;

        let mut ast_declarations: Vec<VarDeclarator> = vec![];

        for declaration in declarations {
            // Input
            let name = &declaration.name;
            self.state.wrapper_component_variable_ident = Some(name.clone());

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

    // CREATE JSX ELEMENT with Spread Attribute
    pub fn create_jsx_element(&self, theme_name: Option<String>, value: String) -> JSXElement {
        let spread_element = SpreadElement {
            dot3_token: DUMMY_SP,
            expr: Box::new(Expr::Ident(Ident {
                span: DUMMY_SP,
                sym: "props".into(),
                optional: false,
                ctxt: SyntaxContext::empty(),
            })),
        };

        let jsx_opening_element = JSXOpeningElement {
            span: DUMMY_SP,
            name: JSXElementName::Ident(Ident {
                span: DUMMY_SP,
                sym: match theme_name {
                    Some(name) => format!("{}__{}", value, name).into(),
                    None => value.into(),
                },
                optional: false,
                ctxt: SyntaxContext::empty(),
            }),
            attrs: vec![JSXAttrOrSpread::SpreadElement(spread_element)],
            self_closing: true,
            type_args: None,
        };

        let jsx_element = JSXElement {
            span: DUMMY_SP,
            opening: jsx_opening_element,
            children: vec![],
            closing: None,
        };

        return jsx_element;
    }

    pub fn create_theme_wrapper(&mut self, component_name: String) -> FnDecl {
        let wrapper_name = format!("ThemeWrapper__{}", component_name);

        self.state.theme_wrapper_component_name = Some(wrapper_name.clone());
        self.state.target_component_name = Some(component_name.clone());

        let ident = Ident {
            span: DUMMY_SP,
            sym: wrapper_name.clone().into(),
            optional: false,
            ctxt: SyntaxContext::empty(),
        };

        // Statments add later
        // variable statement
        // condition statement
        // return statement -> default to returning the target component with spread props, e.g. <Component {...props} />
        let stmts: BlockStmt = BlockStmt {
            span: DUMMY_SP,
            stmts: vec![],
            ctxt: SyntaxContext::empty(),
        };

        let function_decl = FnDecl {
            ident: ident,
            declare: false,
            function: Box::new(Function {
                params: vec![],
                decorators: vec![],
                span: DUMMY_SP,
                body: Some(stmts),
                is_generator: false,
                is_async: true,
                type_params: None,
                return_type: None,
                ctxt: SyntaxContext::empty(),
            }),
        };

        return function_decl;
    }

    fn create_condition_stmt(
        &self,
        variable_ident: String,
        theme_name: String,
        theme_jsx_element: JSXElement,
    ) -> Stmt {
        let ast_test = Expr::Bin(BinExpr {
            span: DUMMY_SP,
            op: BinaryOp::EqEqEq,
            left: Box::new(Expr::Ident(Ident {
                span: DUMMY_SP,
                sym: variable_ident.into(),
                optional: false,
                ctxt: SyntaxContext::empty(),
            })),
            right: Box::new(Expr::Lit(Lit::Str(Str {
                span: DUMMY_SP,
                value: theme_name.clone().into(),
                raw: Some(format!("\"{}\"", theme_name).into()),
            }))),
        });

        let if_stmt = IfStmt {
            span: DUMMY_SP,
            test: Box::new(ast_test),
            cons: Box::new(Stmt::Block(BlockStmt {
                span: DUMMY_SP,
                stmts: vec![Stmt::Return(ReturnStmt {
                    span: DUMMY_SP,
                    arg: Some(Box::new(Expr::JSXElement(Box::new(theme_jsx_element)))),
                })],
                ctxt: SyntaxContext::empty(),
            })),
            alt: None,
        };
        return Stmt::If(if_stmt);
    }

    pub fn prepare_theme_wrapper_component(
        &self,
        target_jsx_element: &JSXElement,
        theme_condition_stmts: &Vec<Stmt>,
        theme_variable_stmt: &Decl,
        mut theme_wrapper_fn_decl: FnDecl,
    ) -> FnDecl {
        let mut stmts: Vec<Stmt> = vec![];

        // const theme = await getThemeCookieServer();
        stmts.push(Stmt::Decl(theme_variable_stmt.clone()));

        // theme condition statements
        // stmts.extend(theme_condition_stmts.clone());
        //
        stmts.extend(theme_condition_stmts.clone());

        // FALLBACK: Return target component with spread props, e.g. <Component {...props} />
        stmts.push(Stmt::Return(ReturnStmt {
            span: DUMMY_SP,
            arg: Some(Box::new(Expr::JSXElement(Box::new(
                target_jsx_element.clone(),
            )))),
        }));

        // ADDING STMTS TO FUNCTION BODY
        theme_wrapper_fn_decl.function.body = Some(BlockStmt {
            span: DUMMY_SP,
            ctxt: SyntaxContext::empty(),
            stmts,
        });

        // ADDING PARAMS TO FUNCTION DECLARATION (props)
        theme_wrapper_fn_decl.function.params = vec![Param {
            span: DUMMY_SP,
            decorators: vec![],
            pat: Pat::Ident(BindingIdent {
                id: Ident {
                    span: DUMMY_SP,
                    sym: "props".into(),
                    optional: false,
                    ctxt: SyntaxContext::empty(),
                },
                type_ann: None,
            }),
        }];

        return theme_wrapper_fn_decl;
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
        let targets = self.theme_mapping.targets.clone();

        for target in &targets {
            let target_nature = &target.nature;

            match target_nature {
                TargetNature::Component => {
                    debug!("Found target component: {:#?}", target);

                    // store target specifier value in state for later use when visiting import declarations
                    let target_import = target.import.clone();
                    let target_specifier = target_import.specifiers.first().unwrap();

                    self.state.check_target_specifier = Some(target_specifier.value.clone());
                    // --------------------------------------

                    // Create target JSX element and store in state for later injection
                    let target_jsx_element =
                        self.create_jsx_element(None, target_specifier.value.clone());

                    self.state.target_jsx_element = Some(target_jsx_element);
                    // --------------------------------------

                    // Create ThemeComponent Wrapper and store in state for later injection
                    let wrapper_component_decl =
                        self.create_theme_wrapper(target_specifier.value.clone());

                    self.state.theme_wrapper_component_fn_decl =
                        Some(wrapper_component_decl.clone());
                    // --------------------------------------

                    // loops themes and create import statements for each theme, store in state
                    let target_themes = &target.themes;
                    for theme in target_themes {
                        let theme_import = theme.import.clone();
                        let theme_name = theme.theme_name.clone();

                        // CREATE IMPORT STATEMENT AND STORE IN STATE
                        let import_module_item =
                            self.create_import_decl(&theme_import, Some(&theme_name));

                        self.state
                            .theme_imports
                            .get_or_insert_with(Vec::new)
                            .push(import_module_item);
                        // -------------------------------

                        // CREATE JSX ELEMENT AND STORE IN STATE
                        let theme_jsx_element = self.create_jsx_element(
                            Some(theme_name.clone()),
                            target_specifier.value.clone(),
                        );

                        self.state
                            .theme_jsx_elements
                            .get_or_insert_with(Vec::new)
                            .push(theme_jsx_element.clone());
                        // --------------------------------------

                        // CREATE CONDITION STATEMENT AND STORE IN STATE
                        let variable_ident = &self.state.wrapper_component_variable_ident.clone();
                        if let Some(variable_ident) = variable_ident {
                            let theme_if_stmt = self.create_condition_stmt(
                                variable_ident.to_string(),
                                theme_name.clone(),
                                theme_jsx_element,
                            );
                            self.state
                                .theme_wrapper_component_if_stmts
                                .get_or_insert_with(Vec::new)
                                .push(theme_if_stmt);
                        }
                    }

                    // Prepare the Final theme wrapper component
                    let target_jsx_element = &self.state.target_jsx_element;
                    let theme_condition_stmts = &self.state.theme_wrapper_component_if_stmts;
                    let theme_variable_stmt = &self.state.wrapper_component_variable_stmt;
                    let theme_wrapper_fn_decl = &self.state.theme_wrapper_component_fn_decl;

                    if let (
                        Some(target_jsx_element),
                        Some(theme_condition_stmts),
                        Some(theme_variable_stmt),
                        Some(theme_wrapper_fn_decl),
                    ) = (
                        target_jsx_element,
                        theme_condition_stmts,
                        theme_variable_stmt,
                        theme_wrapper_fn_decl,
                    ) {
                        let theme_wrapper_fn_decl_mut = theme_wrapper_fn_decl.clone();
                        let final_theme_wrapper_component = self.prepare_theme_wrapper_component(
                            target_jsx_element,
                            theme_condition_stmts,
                            theme_variable_stmt,
                            theme_wrapper_fn_decl_mut,
                        );

                        self.state.theme_wrapper_component =
                            Some(final_theme_wrapper_component.clone());
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
}

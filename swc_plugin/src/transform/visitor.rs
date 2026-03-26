use std::fmt::format;

use crate::config::models::{
    Declaration, DeclarationNature, Directive, Import, Specifier, SpecifierNature, TargetNature,
    ThemeMapping, Variable,
};
// use crate::config::{ImportDeclaration, TargetType, ThemeMapping};
use crate::{analyze::state::AnalyzeState, config::PluginConfig};
use log::debug;
use swc_core::common::{DUMMY_SP, SyntaxContext};
use swc_core::ecma::ast::*;
use swc_core::ecma::{
    ast::{Ident, ImportSpecifier, ModuleDecl, ModuleItem, Str},
    visit::{VisitMut, VisitMutWith},
};

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

    pub fn get_all_imported_idents(&self, module: &Module) -> Vec<String> {
        let mut idents: Vec<String> = vec![];

        for item in &module.body {
            if let ModuleItem::ModuleDecl(ModuleDecl::Import(decl)) = item {
                for specifier in &decl.specifiers {
                    match specifier {
                        ImportSpecifier::Named(name) => idents.push(name.local.sym.to_string()),
                        ImportSpecifier::Default(default) => {
                            idents.push(default.local.sym.to_string())
                        }
                        ImportSpecifier::Namespace(namespace) => {
                            idents.push(namespace.local.sym.to_string())
                        }
                    }
                }
            }
        }

        return idents;
    }

    pub fn import_exists(&self, module: &Module, import_declaration: &Import) -> bool {
        let source = &import_declaration.source;
        module.body.iter().any(|item| {
        matches!(item, ModuleItem::ModuleDecl(ModuleDecl::Import(decl)) if decl.src.value.to_string_lossy() == *source)
      })
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

    pub fn insert_import(&self, module: &mut Module, import: &ModuleItem) {
        let total_imports_length = module
            .body
            .iter()
            .rposition(|item| matches!(item, ModuleItem::ModuleDecl(ModuleDecl::Import(_))));
        if let Some(last_import_index) = total_imports_length {
            module.body.insert(last_import_index + 1, import.clone());
        }
    }

    pub fn create_variable_stmt(&self, variable: &Variable) -> Decl {
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

            // debug!("Created callee identifier: {:?}", ast_variable);
        }

        // let stmt = Stmt::Decl(Decl::Var(Box::new(VarDecl {
        //     kind: match kind.as_str() {
        //         "var" => VarDeclKind::Var,
        //         "let" => VarDeclKind::Let,
        //         "const" => VarDeclKind::Const,
        //         _ => VarDeclKind::Var, // Default to var if kind is unrecognized
        //     },
        //     decls: ast_declarations,
        //     span: DUMMY_SP,
        //     declare: false,
        //     ctxt: SyntaxContext::empty(),
        // })));

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

    pub fn create_theme_component_wrapper(&self, variable_decl: Decl) -> Decl {
        // Input
        let theme_name = "base";
        let wrapper_name = "ThemeWrapper";

        // ThemeWrapper__base
        let component_name = format!("{}__{}", wrapper_name, theme_name);

        let ident = Ident {
            span: DUMMY_SP,
            sym: component_name.clone().into(),
            optional: false,
            ctxt: SyntaxContext::empty(),
        };

        let variable_stmt = Stmt::Decl(variable_decl.clone());

        let stmts: BlockStmt = BlockStmt {
            span: DUMMY_SP,
            stmts: vec![variable_stmt],
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

        return Decl::Fn(function_decl);
    }
}

impl VisitMut for TransformVisitor {
    fn visit_mut_module(&mut self, node: &mut Module) {
        node.visit_mut_children_with(self);
        
        if let Some(theme_resolve_import) = &self.state.theme_resolve_import {
            
            self.insert_import(node, theme_resolve_import);
        }

        if let Some(theme_resolver_config) = &self.state.theme_resolver_config {
            let import_declaration = &theme_resolver_config.import;

            let already_exists = self.import_exists(node, import_declaration);

            // If the import declaration already exists, we skip adding it again
            if already_exists {
                return;
            }

           // let import_module_item = self.create_import_decl(import_declaration, None);
            //self.insert_import(node, &import_module_item);

            //let variable = &theme_resolver_config.variable;
            //let variable_stmt = self.create_variable_stmt(variable);

           // self.state.wrapper_component_variable_stmt = Some(variable_stmt.clone());
            // let theme_component_wrapper = self.create_theme_component_wrapper(variable_stmt);

            // node.body.insert(
            //     node.body.len(),
            //     ModuleItem::Stmt(Stmt::Decl(theme_component_wrapper)),
            // );
            // debug!(
            //     "Inserting variable declaration statement: {:?}",
            //     variable_stmt
            // );
            // node.body.insert(0, ModuleItem::Stmt(Stmt::Decl(variable_stmt)));
        };

        let targets = &self.theme_mapping.targets;

        for target in targets {
            let target_type = &target.nature;

            match target_type {
                TargetNature::Component => {
                    // let target_source = &target.import.source;
                    let target_specifiers = &target.import.specifiers;
                    let existing_idents = &self.get_all_imported_idents(node);

                    let mut target_component = vec![];
                    target_specifiers.iter().for_each(|item| {
                        let matches: Vec<_> = existing_idents
                            .iter()
                            .filter(|ident| *ident == &item.value)
                            .collect();
                        target_component.extend(matches);
                    });

                 
                    
                    if target_component.iter().all(|matches| matches.is_empty())  {
                        continue;
                    }
                    debug!("Matched Componenttarget: {:#?}", target_component);

                    // let theme_component_wrapper = self.create_theme_component_wrapper(variable_stmt);

                    /*
                    debug!(
                        "Existing imported identifiers in the module: {:?}",
                        existing_idents
                    ); */

                    // let already_exists = target_specifiers.iter().all(|specifier| {
                    //     existing_idents
                    //         .iter()
                    //         .any(|ident| ident == &specifier.value)
                    // });

                    // if !already_exists {
                    //     continue;
                    // }

                    // target_specifiers.iter().for_each(|specifier| {
                    //     let mut idents: Vec<String> = vec![];

                    //     for item in &node.body {
                    //         if let ModuleItem::ModuleDecl(ModuleDecl::Import(decl)) = item {
                    //             for specifier in &decl.specifiers {
                    //                 match specifier {
                    //                     ImportSpecifier::Named(name) => {
                    //                         specifier.value
                    //                         // specifiernode name.local.sym.to_string()
                    //                         //     && idents.push(name.local.sym.to_string())
                    //                     }
                    //                     ImportSpecifier::Default(default) => {}
                    //                     ImportSpecifier::Namespace(namespace) => {
                    //                     }
                    //                 }
                    //             }
                    //         }
                    //     }

                    //     debug!(
                    //         "Processing target specifier: {:?} with nature: {:?}",
                    //         specifier.value, specifier.nature
                    //     );
                    // });
                    let target_themes = &target.themes;

                    for target_theme in target_themes {
                        let import_declaration = &target_theme.import;
                        let theme_name = &target_theme.theme_name;
                        //   let already_exists = self.import_exists(node, &import_declaration);

                        //   if !already_exists {
                        //     continue;
                        //   }

                        let import_module_item =
                            self.create_import_decl(&import_declaration, Some(theme_name));

                        self.insert_import(node, &import_module_item);
                    }
                }
                TargetNature::Agents => {
                    debug!("Processing Agents target: {}", target.theme_name);
                }
                TargetNature::Page => {
                    debug!("Processing Page target: {}", target.theme_name);
                }
            }
        }
    }
}

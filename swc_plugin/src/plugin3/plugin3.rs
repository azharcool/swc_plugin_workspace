use crate::plugin3_config::{Plugin3Config, SpecifierType};
use log::debug;
use swc_core::{
    common::{DUMMY_SP, SyntaxContext},
    ecma::{
        ast::{
            AwaitExpr, BindingIdent, BlockStmt, CallExpr, Callee, Decl, Expr, FnDecl, Function,
            Ident, ImportDecl, ImportDefaultSpecifier, ImportNamedSpecifier, ImportSpecifier, Lit,
            Module, ModuleDecl, ModuleItem, Param, Pat, Program, Stmt, Str, TsEntityName, TsType,
            TsTypeAnn, TsTypeRef, VarDecl, VarDeclKind, VarDeclarator,
        },
        visit::{VisitMut, VisitMutWith},
    },
};

pub struct Plugin3 {
    pub config: Plugin3Config,
    pub filename: String,
}

impl Plugin3 {
    pub fn new(config: Plugin3Config, filename: String) -> Self {
        Self {
            config,
            filename,
        }
    }
}

impl VisitMut for Plugin3 {
    fn visit_mut_program(&mut self, node: &mut Program) {
        debug!("Visiting Program: {:#?}", node);
        node.visit_mut_children_with(self);
    }

    fn visit_mut_module(&mut self, module: &mut Module) {
        // ── 1. visit children FIRST ───────────────────────────────────────
        module.visit_mut_children_with(self);

        // ── 2. then mutate body ───────────────────────────────────────────
        let Some(resolver) = &self.config.theme_name_resolver else {
            return;
        };
        let Some(resolver_config) = &resolver.server else {
            return;
        };
        let Some(import_declaration_config) = &resolver_config.import_declaration else {
            return;
        };

        let source = &import_declaration_config.source;
        let Some(specifier) = &import_declaration_config.specifier else {
            return;
        };

        let specifier_type = &specifier.specifier_type;
        let name = &specifier.name;

        let import_specifier = match specifier_type {
            SpecifierType::ImportSpecifier => ImportSpecifier::Named(ImportNamedSpecifier {
                span: DUMMY_SP,
                local: Ident::new(name.clone().into(), DUMMY_SP, Default::default()),
                imported: None,
                is_type_only: false,
            }),
            SpecifierType::ImportDefaultSpecifier => {
                ImportSpecifier::Default(ImportDefaultSpecifier {
                    span: DUMMY_SP,
                    local: Ident::new(name.clone().into(), DUMMY_SP, Default::default()),
                })
            }
            _ => panic!("unsupported specifier type"),
        };

        let import = ModuleItem::ModuleDecl(ModuleDecl::Import(ImportDecl {
            span: DUMMY_SP,
            specifiers: vec![import_specifier],
            src: Box::new(Str {
                span: DUMMY_SP,
                value: source.clone().into(),
                raw: Some(format!("\"{}\"", source).into()),
            }),
            phase: Default::default(),
            type_only: false,
            with: None,
        }));

        let fn_decl = ModuleItem::Stmt(Stmt::Decl(Decl::Fn(FnDecl {
            ident: Ident {
                span: DUMMY_SP,
                ctxt: SyntaxContext::empty(),
                sym: "ThemeWrapperServer".into(),
                optional: false,
            },
            declare: false,
            function: Box::new(Function {
                params: vec![Param {
                    span: DUMMY_SP,
                    decorators: vec![],
                    pat: Pat::Ident(BindingIdent {
                        id: Ident {
                            span: DUMMY_SP,
                            ctxt: SyntaxContext::empty(),
                            sym: "props".into(),
                            optional: false,
                        },
                        type_ann: Some(Box::new(TsTypeAnn {
                            span: DUMMY_SP,
                            type_ann: Box::new(TsType::TsTypeRef(TsTypeRef {
                                span: DUMMY_SP,
                                type_name: TsEntityName::Ident(Ident {
                                    span: DUMMY_SP,
                                    ctxt: SyntaxContext::empty(),
                                    sym: "IProps".into(),
                                    optional: false,
                                }),
                                type_params: None,
                            })),
                        })),
                    }),
                }],
                decorators: vec![],
                span: DUMMY_SP,
                ctxt: SyntaxContext::empty(),
                body: Some(BlockStmt {
                    span: DUMMY_SP,
                    ctxt: SyntaxContext::empty(),
                    stmts: vec![
                        // const themeName = await name();
                        Stmt::Decl(Decl::Var(Box::new(VarDecl {
                            span: DUMMY_SP,
                            ctxt: SyntaxContext::empty(),
                            kind: VarDeclKind::Const,
                            declare: false,
                            decls: vec![VarDeclarator {
                                span: DUMMY_SP,
                                name: Pat::Ident(BindingIdent {
                                    id: Ident {
                                        span: DUMMY_SP,
                                        ctxt: SyntaxContext::empty(),
                                        sym: "themeName".into(),
                                        optional: false,
                                    },
                                    type_ann: None,
                                }),
                                init: Some(Box::new(Expr::Await(AwaitExpr {
                                    span: DUMMY_SP,
                                    arg: Box::new(Expr::Call(CallExpr {
                                        span: DUMMY_SP,
                                        ctxt: SyntaxContext::empty(),
                                        callee: Callee::Expr(Box::new(Expr::Ident(Ident {
                                            span: DUMMY_SP,
                                            ctxt: SyntaxContext::empty(),
                                            sym: name.clone().into(),
                                            optional: false,
                                        }))),
                                        args: vec![],
                                        type_args: None,
                                    })),
                                }))),
                                definite: false,
                            }],
                        }))),
                    ],
                }),
                is_generator: false,
                is_async: true,
                type_params: None,
                return_type: None,
            }),
        })));

        // ── insert import after last existing import ───────────────────────
        let insert_pos = module
            .body
            .iter()
            .rposition(|item| matches!(item, ModuleItem::ModuleDecl(ModuleDecl::Import(_))))
            .map(|i| i + 1)
            .unwrap_or_else(|| {
                // no imports found — insert AFTER any leading directives
                module
                    .body
                    .iter()
                    .position(|item| {
                        !matches!(
                            item,
                            ModuleItem::Stmt(Stmt::Expr(expr_stmt))
                            if matches!(&*expr_stmt.expr, Expr::Lit(Lit::Str(_)))
                        )
                    })
                    .unwrap_or(0)
            });

        module.body.insert(insert_pos, import);
        module.body.push(fn_decl);
    }

}

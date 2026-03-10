use swc_core::ecma::{
    ast::*,
    visit::{VisitMut, VisitMutWith},
};
use swc_core::common::{DUMMY_SP, SyntaxContext, SourceMap};
use swc_core::common::sync::Lrc;
use swc_core::ecma::codegen::{text_writer::JsWriter, Emitter, Config};

use crate::log_info;

pub struct MyPlugin2 {
    debug: bool,
}

impl MyPlugin2 {
    pub fn new(debug: bool) -> Self {
        MyPlugin2 { debug }
    }
    

    // Helper function to convert AST back to code for logging
    fn module_to_code(&self, module: &Module) -> String {
        let cm: Lrc<SourceMap> = Default::default();
        let mut buf = vec![];
        {
            let wr = JsWriter::new(cm.clone(), "\n",&mut buf, None);
            let mut emitter = Emitter {
                cfg: Config::default(),
                cm: cm.clone(),
                comments: None,
                wr
            };
            
            emitter.emit_module(module).unwrap_or_else(|e| {
                log_info!("module_to_code", "Error generating code: {:?}", e);
            });
        }
        
        String::from_utf8(buf).unwrap_or_else(|_| "<invalid UTF-8>".into())
    }
}

impl VisitMut for MyPlugin2 {
    
fn visit_mut_module(&mut self, module: &mut Module) {
    
    // --- Capture BEFORE code for debugging ---
    let before_code = if self.debug {
        Some(self.module_to_code(module))
    } else {
        None
    };
    let mut signup_import_index: Option<usize> = None;

    for (i, item) in module.body.iter_mut().enumerate() {
        if let ModuleItem::ModuleDecl(ModuleDecl::Import(import)) = item {
            if import.src.value == "@znode/base-components/components/signup" {
                signup_import_index = Some(i);

                for spec in &mut import.specifiers {
                    if let ImportSpecifier::Named(named) = spec {
                        if named.local.sym == *"SignUp" {

                            // preserve original export
                            named.imported = Some(ModuleExportName::Ident(
                                Ident::new("SignUp".into(), DUMMY_SP, SyntaxContext::empty())
                            ));

                            // rename local variable
                            named.local = Ident::new(
                                "BaseSignUp".into(),
                                DUMMY_SP,
                                SyntaxContext::empty(),
                            );
                        }
                    }
                }
            }
        }
    }

    if let Some(index) = signup_import_index {

        // Custom theme signup import
        let custom_import = ModuleItem::ModuleDecl(ModuleDecl::Import(ImportDecl {
            span: DUMMY_SP,
            specifiers: vec![ImportSpecifier::Named(ImportNamedSpecifier {
                span: DUMMY_SP,
                local: Ident::new("Custom1SignUp".into(), DUMMY_SP, SyntaxContext::empty()),
                imported: Some(ModuleExportName::Ident(
                    Ident::new("SignUp".into(), DUMMY_SP, SyntaxContext::empty()),
                )),
                is_type_only: false,
            })],
            src: Box::new(Str {
                span: DUMMY_SP,
                value: "@znode/custom1/components/signup".into(),
                raw: None,
            }),
            type_only: false,
            with: None,
            phase: Default::default(),
        }));

        module.body.insert(index + 1, custom_import);

        // Theme cookie import
        let theme_cookie_import = ModuleItem::ModuleDecl(ModuleDecl::Import(ImportDecl {
            span: DUMMY_SP,
            specifiers: vec![ImportSpecifier::Named(ImportNamedSpecifier {
                span: DUMMY_SP,
                local: Ident::new(
                    "getThemeCookieServer".into(),
                    DUMMY_SP,
                    SyntaxContext::empty(),
                ),
                imported: None,
                is_type_only: false,
            })],
            src: Box::new(Str {
                span: DUMMY_SP,
                value: "@znode/utils/theme-cookie/theme-cookie.server".into(),
                raw: None,
            }),
            type_only: false,
            with: None,
            phase: Default::default(),
        }));

        module.body.insert(index + 2, theme_cookie_import);
    }

    module.visit_mut_children_with(self);
    
    if self.debug {
        let after_code = self.module_to_code(module);

        if let Some(before) = before_code {
            log_info!("transform", "\n\n====================== BEFORE======================\n\n{}\n\n====================== AFTER ======================\n\n{}\n\n", before, after_code);
        }
    }
}   

fn visit_mut_jsx_element(&mut self, jsx: &mut JSXElement) {

    let mut new_children = vec![];

    for child in jsx.children.drain(..) {

        match child {

            JSXElementChild::JSXElement(el) => {

                if let JSXElementName::Ident(ident) = &el.opening.name {

                    if ident.sym == *"SignUp" {

                        let base = JSXElement {
                            span: DUMMY_SP,
                            opening: JSXOpeningElement {
                                name: JSXElementName::Ident(
                                    Ident::new("BaseSignUp".into(), DUMMY_SP, SyntaxContext::empty())
                                ),
                                ..el.opening.clone()
                            },
                            children: el.children.clone(),
                            closing: el.closing.clone(),
                        };

                        let custom = JSXElement {
                            span: DUMMY_SP,
                            opening: JSXOpeningElement {
                                name: JSXElementName::Ident(
                                    Ident::new("Custom1SignUp".into(), DUMMY_SP, SyntaxContext::empty())
                                ),
                                ..el.opening.clone()
                            },
                            children: el.children.clone(),
                            closing: el.closing.clone(),
                        };

                        let cond = Expr::Cond(CondExpr {
                            span: DUMMY_SP,
                            test: Box::new(Expr::Bin(BinExpr {
                                span: DUMMY_SP,
                                op: BinaryOp::EqEqEq,
                                left: Box::new(Expr::Ident(
                                    Ident::new("themeName".into(), DUMMY_SP, SyntaxContext::empty())
                                )),
                                right: Box::new(Expr::Lit(Lit::Str(Str {
                                    span: DUMMY_SP,
                                    value: "custom1".into(),
                                    raw: None,
                                }))),
                            })),
                            cons: Box::new(Expr::JSXElement(Box::new(custom))),
                            alt: Box::new(Expr::JSXElement(Box::new(base))),
                        });

                        new_children.push(JSXElementChild::JSXExprContainer(
                            JSXExprContainer {
                                span: DUMMY_SP,
                                expr: JSXExpr::Expr(Box::new(cond)),
                            }
                        ));

                        continue;
                    }
                }

                new_children.push(JSXElementChild::JSXElement(el));
            }

            _ => new_children.push(child)
        }
    }

    jsx.children = new_children;

    jsx.visit_mut_children_with(self);
}

fn visit_mut_function(&mut self, func: &mut Function) {

    if let Some(body) = &mut func.body {

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
                        callee: Callee::Expr(Box::new(Expr::Ident(Ident::new(
                            "getThemeCookieServer".into(),
                            DUMMY_SP,
                            SyntaxContext::empty(),
                        )))),
                        args: vec![],
                        type_args: None,
                    })),
                }))),
                definite: false,
            }],
        })));

        body.stmts.insert(0, stmt);
    }

    func.visit_mut_children_with(self);
}


}
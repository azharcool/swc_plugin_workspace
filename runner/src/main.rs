use swc_core::{
    common::{FileName, SourceMap, sync::Lrc},
    ecma::{
        parser::{EsSyntax, Parser, StringInput, Syntax},
        visit::VisitMutWith,
    },
};

use swc_plugin::MyPlugin;

fn main(){
    let cm: Lrc<SourceMap> = Default::default();

    let source_code  = r#"
        import React from "react";
        const x = 5;
        function Test(){
            return <div>Hello World</div>;
        }
    "#;
    
    let fm = cm.new_source_file(
        FileName::Custom("test.js".into()).into(),
        source_code,
    );
    
    let mut parser = Parser::new(
        Syntax::Es(EsSyntax {
            jsx: true,
            ..Default::default()
        }),
        StringInput::from(&*fm),
        None,
    );

    let mut program = match parser.parse_program() {
        Ok(p) => p,
        Err(error) => {
            eprintln!("Error parsing JavaScript: {:?}", error);
            return;
        }
    };

    let mut plugin = MyPlugin;
    program.visit_mut_with(&mut plugin);
    


    let mut x = 5;
    println!("Hello, world! The value of x is: {x}");
    x = 6;
    println!("The value of x is now: {x}");
}
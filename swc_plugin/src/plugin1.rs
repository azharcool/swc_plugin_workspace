use swc_core::ecma::{
    ast::*,
    visit::{VisitMut, VisitMutWith},
};

use crate::logger::log;

pub struct MyPlugin;


impl VisitMut for MyPlugin {
    fn visit_mut_program(&mut self, node: &mut Program){
        log("Visiting program node");
        log(&format!("Program node: {:?}", node));
        // You can modify the AST here as needed
        log("program node visited successfully");
        
        // Continue visiting the rest of the AST
        node.visit_mut_children_with(self);
    }
}
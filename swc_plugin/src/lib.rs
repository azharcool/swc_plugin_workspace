use swc_core::ecma::{
    ast::*,
    visit::{VisitMut, VisitMutWith},
};

pub struct MyPlugin;

impl VisitMut for MyPlugin {
    fn visit_mut_program(&mut self, node: &mut Program) {
        println!("Visiting program node: {:?}", node);
        // You can modify the AST here as needed
        println!("program node visited successfully!");
        
        // Continue visiting the rest of the AST
        node.visit_mut_children_with(self);
    }
}
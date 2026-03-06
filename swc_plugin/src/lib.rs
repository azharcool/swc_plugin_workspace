pub mod logger;
//pub mod plugin1;
pub mod plugin2;

use swc_core::ecma::ast::*;
use swc_core::ecma::visit::VisitMutWith;
use swc_core::plugin::{
    metadata::TransformPluginMetadataContextKind, plugin_transform,
    proxies::TransformPluginProgramMetadata,
};

//pub use plugin1::MyPlugin;
pub use plugin2::MyPlugin2;

#[plugin_transform]
pub fn process_program(mut program: Program, metadata: TransformPluginProgramMetadata) -> Program {
    //let mut plugin = MyPlugin;
    //program.visit_mut_with(&mut plugin);

    if let Some(filename) = metadata.get_context(&TransformPluginMetadataContextKind::Filename) {
        if filename.ends_with("signup/page.tsx") {
            let mut plugin2 = MyPlugin2;
            program.visit_mut_with(&mut plugin2);
        }
    }

    program
}

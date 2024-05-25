use anyhow::Result;
use swc_ecma_ast::ModuleDecl;

use super::{GenerateHandlers, GenerationTarget};

impl GenerateHandlers for ModuleDecl {
    fn generate_handlers(&self, gen_target: &mut GenerationTarget) -> Result<()> {
        match self {
            ModuleDecl::ExportDecl(decl) => decl.generate_handlers(gen_target),
            _ => Ok(()),
        }
    }
}

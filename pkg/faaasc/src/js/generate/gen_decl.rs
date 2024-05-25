use anyhow::Result;
use swc_ecma_ast::Decl;

use super::GenerateHandlers;

impl GenerateHandlers for Decl {
    fn generate_handlers(&self, gen_target: &mut super::GenerationTarget) -> Result<()> {
        match self {
            Decl::Fn(fn_decl) => fn_decl.generate_handlers(gen_target),
            _ => Ok(()),
        }
    }
}

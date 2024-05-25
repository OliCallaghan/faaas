use anyhow::Result;
use swc_ecma_ast::ExportDecl;

use super::GenerateHandlers;

impl GenerateHandlers for ExportDecl {
    fn generate_handlers(&self, gen_target: &mut super::GenerationTarget) -> Result<()> {
        self.decl.generate_handlers(gen_target)
    }
}

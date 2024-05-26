use anyhow::Result;
use swc_ecma_ast::ModuleDecl;

use super::GenerateWithTarget;

impl GenerateWithTarget for ModuleDecl {
    fn generate(&self, gen_target: &mut Vec<super::Generation>) -> Result<()> {
        match self {
            ModuleDecl::ExportDecl(decl) => decl.generate(gen_target),
            _ => Ok(()),
        }
    }
}

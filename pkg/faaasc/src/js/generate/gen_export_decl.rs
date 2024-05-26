use anyhow::Result;
use swc_ecma_ast::ExportDecl;

use super::{GenerateWithTarget, Generation};

impl GenerateWithTarget for ExportDecl {
    fn generate(&self, gen_target: &mut Vec<Generation>) -> Result<()> {
        self.decl.generate(gen_target)
    }
}

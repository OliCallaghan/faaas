use anyhow::Result;
use swc_ecma_ast::FnDecl;

use super::{GenerateWithTarget, GenerateWithTargetAndIdent, Generation};

impl GenerateWithTarget for FnDecl {
    fn generate(&self, gen_target: &mut Vec<Generation>) -> Result<()> {
        println!("Found function with name: {}", self.ident);

        self.function.generate(gen_target, self.ident.clone())
    }
}

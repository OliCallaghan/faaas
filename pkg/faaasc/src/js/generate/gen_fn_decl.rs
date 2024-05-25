use anyhow::Result;
use swc_ecma_ast::FnDecl;

use super::{GenerateHandler, GenerateHandlers, GenerationTarget};

impl GenerateHandlers for FnDecl {
    fn generate_handlers(&self, gen_target: &mut GenerationTarget) -> Result<()> {
        let id = self.ident.sym.as_str();
        println!("Found function with name: {}", id);

        let mut gen = gen_target.new_gen(self.ident.clone());

        return self.function.generate_split(&mut gen);
    }
}

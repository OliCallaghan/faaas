use anyhow::Result;
use swc_ecma_ast::FnDecl;

use super::{GenerateSplit, Generation};

impl GenerateSplit for FnDecl {
    fn generate_split(&self, gen: &mut Option<Generation>) -> Result<()> {
        let id = self.ident.sym.as_str();
        println!("Found function with name: {}", id);

        if let Some(_) = gen {
            println!("In a generation already!");

            return self.function.generate_split(gen);
        }

        let mut gen = Some(Generation::new(self.ident.clone()));

        return self.function.generate_split(&mut gen);
    }
}

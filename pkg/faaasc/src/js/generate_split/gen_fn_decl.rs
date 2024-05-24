use anyhow::Result;
use swc_ecma_ast::FnDecl;

use super::GenerateSplit;

impl GenerateSplit for FnDecl {
    fn generate_split(&self) -> Result<()> {
        let id = self.ident.sym.as_str();

        println!("Found function with name: {}", id);

        self.function.generate_split()
    }
}

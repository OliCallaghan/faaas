use anyhow::Result;
use swc_ecma_ast::ExportDecl;

use super::{GenerateSplit, Generation};

impl GenerateSplit for ExportDecl {
    fn generate_split(&self, gen: &mut Option<Generation>) -> Result<()> {
        self.decl.generate_split(gen)
    }
}

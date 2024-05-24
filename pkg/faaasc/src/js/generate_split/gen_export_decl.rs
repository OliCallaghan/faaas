use anyhow::Result;
use swc_ecma_ast::ExportDecl;

use super::GenerateSplit;

impl GenerateSplit for ExportDecl {
    fn generate_split(&self) -> Result<()> {
        self.decl.generate_split()
    }
}

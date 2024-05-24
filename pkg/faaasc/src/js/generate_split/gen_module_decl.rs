use anyhow::Result;
use swc_ecma_ast::ModuleDecl;

use super::GenerateSplit;

impl GenerateSplit for ModuleDecl {
    fn generate_split(&self) -> Result<()> {
        match self {
            ModuleDecl::ExportDecl(decl) => decl.generate_split(),
            _ => Ok(()),
        }
    }
}

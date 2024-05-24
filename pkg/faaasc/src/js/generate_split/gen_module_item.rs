use anyhow::Result;
use swc_ecma_ast::ModuleItem;

use super::GenerateSplit;

impl GenerateSplit for ModuleItem {
    fn generate_split(&self) -> Result<()> {
        match self {
            ModuleItem::ModuleDecl(decl) => decl.generate_split(),
            _ => Ok(()),
        }
    }
}

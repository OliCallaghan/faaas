use anyhow::Result;
use swc_ecma_ast::ModuleItem;

use super::{GenerateSplit, Generation};

impl GenerateSplit for ModuleItem {
    fn generate_split(&self, gen: &mut Option<Generation>) -> Result<()> {
        match self {
            ModuleItem::ModuleDecl(decl) => decl.generate_split(gen),
            _ => Ok(()),
        }
    }
}

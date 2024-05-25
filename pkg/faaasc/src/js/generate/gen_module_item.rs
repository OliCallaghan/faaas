use anyhow::Result;
use swc_ecma_ast::ModuleItem;

use super::{GenerateHandlers, GenerationTarget};

impl GenerateHandlers for ModuleItem {
    fn generate_handlers(&self, gen_target: &mut GenerationTarget) -> Result<()> {
        match self {
            ModuleItem::ModuleDecl(decl) => decl.generate_handlers(gen_target),
            _ => Ok(()),
        }
    }
}

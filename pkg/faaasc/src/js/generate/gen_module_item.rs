use anyhow::Result;
use swc_ecma_ast::ModuleItem;

use super::GenerateWithTarget;

impl GenerateWithTarget for ModuleItem {
    fn generate(&self, gen_target: &mut Vec<super::Generation>) -> Result<()> {
        match self {
            ModuleItem::ModuleDecl(decl) => decl.generate(gen_target),
            _ => Ok(()),
        }
    }
}

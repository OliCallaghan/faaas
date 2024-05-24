use anyhow::Result;
use swc_ecma_ast::Decl;

use super::GenerateSplit;

impl GenerateSplit for Decl {
    fn generate_split(&self) -> Result<()> {
        match self {
            Decl::Fn(fn_decl) => fn_decl.generate_split(),
            _ => Ok(()),
        }
    }
}

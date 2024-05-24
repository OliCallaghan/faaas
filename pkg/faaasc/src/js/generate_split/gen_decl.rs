use anyhow::Result;
use swc_ecma_ast::Decl;

use super::{GenerateSplit, Generation};

impl GenerateSplit for Decl {
    fn generate_split(&self, gen: &mut Option<Generation>) -> Result<()> {
        match self {
            Decl::Fn(fn_decl) => fn_decl.generate_split(gen),
            _ => Ok(()),
        }
    }
}

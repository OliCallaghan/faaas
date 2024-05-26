use anyhow::Result;
use swc_ecma_ast::Decl;

use super::{GenerateWithTarget, Generation};

impl GenerateWithTarget for Decl {
    fn generate(&self, gen_target: &mut Vec<Generation>) -> Result<()> {
        match self {
            Decl::Fn(fn_decl) => fn_decl.generate(gen_target),
            _ => Ok(()),
        }
    }
}

use anyhow::Result;
use swc_ecma_ast::Expr;

use super::{GenerateSplit, Generation};

impl GenerateSplit for Expr {
    fn generate_split(&self, gen: &mut Option<Generation>) -> Result<()> {
        match self {
            // Expr::Fn(fn_decl) => fn_decl.split(),
            _ => Ok(()),
        }
    }
}

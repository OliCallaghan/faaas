use anyhow::Result;
use swc_ecma_ast::Expr;

use super::{GenerateHandler, Generation};

impl GenerateHandler for Expr {
    fn generate_split(&self, gen: &mut Generation) -> Result<()> {
        match self {
            // Expr::Fn(fn_decl) => fn_decl.split(),
            _ => Ok(()),
        }
    }
}

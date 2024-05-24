use anyhow::Result;
use swc_ecma_ast::Expr;

use super::GenerateSplit;

impl GenerateSplit for Expr {
    fn generate_split(&self) -> Result<()> {
        match self {
            // Expr::Fn(fn_decl) => fn_decl.split(),
            _ => Ok(()),
        }
    }
}

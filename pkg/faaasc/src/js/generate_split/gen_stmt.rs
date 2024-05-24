use anyhow::Result;
use swc_ecma_ast::Stmt;

use super::{GenerateSplit, Generation};

impl GenerateSplit for Stmt {
    fn generate_split(&self, gen: &mut Option<Generation>) -> Result<()> {
        match self {
            Stmt::Block(block_stmt) => block_stmt.generate_split(gen),
            Stmt::Expr(expr_stmt) => expr_stmt.generate_split(gen),
            _ => Ok(()),
        }
    }
}

use anyhow::Result;
use swc_ecma_ast::Stmt;

use super::GenerateSplit;

impl GenerateSplit for Stmt {
    fn generate_split(&self) -> Result<()> {
        match self {
            Stmt::Block(block_stmt) => block_stmt.generate_split(),
            Stmt::Expr(expr_stmt) => expr_stmt.generate_split(),
            _ => Ok(()),
        }
    }
}

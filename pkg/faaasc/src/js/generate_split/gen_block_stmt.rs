use anyhow::Result;
use swc_ecma_ast::BlockStmt;

use super::GenerateSplit;

impl GenerateSplit for BlockStmt {
    fn generate_split(&self) -> Result<()> {
        for stmt in &self.stmts {
            stmt.generate_split()?;
        }

        Ok(())
    }
}

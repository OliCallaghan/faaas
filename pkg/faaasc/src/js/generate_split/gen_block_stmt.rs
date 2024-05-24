use anyhow::Result;
use swc_ecma_ast::BlockStmt;

use super::{GenerateSplit, Generation};

impl GenerateSplit for BlockStmt {
    fn generate_split(&self, gen: &mut Option<Generation>) -> Result<()> {
        for stmt in &self.stmts {
            stmt.generate_split(gen)?;
        }

        Ok(())
    }
}

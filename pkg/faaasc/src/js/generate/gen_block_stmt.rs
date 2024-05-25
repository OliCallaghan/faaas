use anyhow::Result;
use swc_ecma_ast::BlockStmt;

use super::{GenerateHandler, Generation};

impl GenerateHandler for BlockStmt {
    fn generate_split(&self, gen: &mut Generation) -> Result<()> {
        for stmt in &self.stmts {
            let is_async_marker = stmt.generate_split(gen)?;

            if is_async_marker != () {
                // Create a new generation and make sure its been added to target.
                gen.push_stmt(stmt);
            }
        }

        Ok(())
    }
}

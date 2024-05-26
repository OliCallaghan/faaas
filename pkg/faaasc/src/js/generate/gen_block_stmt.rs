use anyhow::Result;
use swc_ecma_ast::BlockStmt;
use swc_ecma_ast::Ident;

use crate::js::capture::CaptureFreeVariables;
use crate::js::capture::FreeVariables;
use crate::js::directive::Directive;

use super::{GenerateWithTargetAndIdent, Generation};

impl GenerateWithTargetAndIdent for BlockStmt {
    fn generate(&self, gen_target: &mut Vec<Generation>, id: Ident) -> Result<()> {
        let mut gen = Generation::new(id);

        for (i, stmt) in self.stmts.iter().enumerate() {
            // Is statement a 'use async' directive?
            if stmt.is_directive("use async") {
                println!("Found 'use async' directive!");

                // Compute free variables for the rest of the statements.
                let mut free_vars = FreeVariables::new();
                for stmt in &self.stmts[i + 2..] {
                    stmt.capture_free_vars(&mut free_vars);
                }

                // Compute split
                println!("FV: {:?}", free_vars.get());

                let old_gen = gen;
                gen = old_gen.next();

                gen_target.push(old_gen);
            } else {
                gen.push_stmt(stmt.clone())
            }
        }

        gen_target.push(gen);

        Ok(())
    }
}

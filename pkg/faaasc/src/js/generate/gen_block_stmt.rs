use anyhow::Result;
use swc_ecma_ast::BlockStmt;
use swc_ecma_ast::Ident;

use crate::js::capture::CaptureFreeVariables;
use crate::js::capture::FreeVariables;
use crate::js::directive::Directive;
use crate::js::generate::GenerateContinuation;

use super::{GenerateDeclFromCtxData, GenerateWithTargetAndIdent, Generation};

impl GenerateWithTargetAndIdent for BlockStmt {
    fn generate(&self, gen_target: &mut Vec<Generation>, id: Ident) -> Result<()> {
        let mut gen = Generation::new(id);
        let mut decl_from_ctx = false;

        for (i, stmt) in self.stmts.iter().enumerate() {
            // Is statement a 'use async' directive?
            if stmt.is_directive("use async") {
                println!("Found 'use async' directive!");

                // Compute free variables for the rest of the statements.
                let mut free_vars = FreeVariables::new();
                for stmt in &self.stmts[i + 1..] {
                    stmt.capture_free_vars(&mut free_vars);
                }

                let (cont_task_ident, cont_task_args_ident, stmt) =
                    self.stmts[i + 1].generate_continuation();

                gen.push_stmt(stmt);
                gen.add_continuation_vars(free_vars);
                gen.add_continuation_task(
                    cont_task_ident.expect("task ident"),
                    cont_task_args_ident.expect("task args ident"),
                );
                decl_from_ctx = true;

                let mut old_gen = gen;
                gen = old_gen.next();

                gen_target.push(old_gen);
            } else {
                if decl_from_ctx {
                    gen.push_stmt(stmt.generate_decl_from_ctx_data());
                    decl_from_ctx = false;
                } else {
                    gen.push_stmt(stmt.clone());
                }
            }
        }

        gen_target.push(gen);

        Ok(())
    }
}

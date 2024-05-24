use anyhow::Result;
use swc_ecma_ast::ExprStmt;

use super::EvalLit;
use super::GenerateSplit;
use super::Generation;

impl GenerateSplit for ExprStmt {
    fn generate_split(&self, gen: &mut Option<Generation>) -> Result<()> {
        if let Some(str_val) = self.expr.eval_lit_str() {
            if str_val.value.as_str() == "use async" {
                println!(
                    "Found 'use async' literal inside {}",
                    gen.as_ref().map_or("no scope", |g| g.name.sym.as_str())
                );
            }
        }

        self.expr.generate_split(gen)
    }
}

use anyhow::Result;
use swc_ecma_ast::ExprStmt;

use super::EvalLit;
use super::GenerateHandler;
use super::Generation;

impl GenerateHandler for ExprStmt {
    fn generate_split(&self, gen: &mut Generation) -> Result<()> {
        if let Some(str_val) = self.expr.eval_lit_str() {
            if str_val.value.as_str() == "use async" {
                println!("Found 'use async' literal inside {}", gen.name.sym.as_str());
            }
        }

        self.expr.generate_split(gen)
    }
}

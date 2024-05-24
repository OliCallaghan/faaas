use anyhow::Result;
use swc_ecma_ast::ExprStmt;

use super::EvalLit;
use super::GenerateSplit;

impl GenerateSplit for ExprStmt {
    fn generate_split(&self) -> Result<()> {
        if let Some(str_val) = self.expr.eval_lit_str() {
            if str_val.value.as_str() == "use async" {
                println!("Found 'use async' literal!");
            }
        }

        self.expr.generate_split()
    }
}

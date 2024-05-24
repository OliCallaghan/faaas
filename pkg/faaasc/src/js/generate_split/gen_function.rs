use anyhow::Result;
use swc_ecma_ast::Function;

use super::GenerateSplit;

impl GenerateSplit for Function {
    fn generate_split(&self) -> Result<()> {
        match &self.body {
            Some(body) => body.generate_split(),
            None => Ok(()),
        }
    }
}

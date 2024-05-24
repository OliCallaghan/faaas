use anyhow::Result;
use swc_ecma_ast::Module;

use super::GenerateSplit;

impl GenerateSplit for Module {
    fn generate_split(&self) -> Result<()> {
        for item in &self.body {
            item.generate_split()?;
        }

        Ok(())
    }
}

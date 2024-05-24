use anyhow::Result;
use swc_ecma_ast::Module;

use super::{GenerateSplit, Generation};

impl GenerateSplit for Module {
    fn generate_split(&self, gen: &mut Option<Generation>) -> Result<()> {
        for item in &self.body {
            item.generate_split(gen)?;
        }

        Ok(())
    }
}

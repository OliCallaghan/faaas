use anyhow::Result;
use swc_ecma_ast::Module;

use super::{Generate, GenerateWithTarget};

impl Generate for Module {
    fn generate(&mut self) -> Result<()> {
        let mut target = Vec::new();

        for item in &self.body {
            item.generate(&mut target)?;
        }

        for gen in target {
            self.body.push(gen.to_module_item());
        }

        Ok(())
    }
}

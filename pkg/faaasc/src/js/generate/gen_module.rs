use anyhow::Result;
use swc_ecma_ast::Module;

use crate::js::generate::GenerationTarget;

use super::{Generate, GenerateHandlers};

impl Generate for Module {
    fn generate(&mut self) -> Result<()> {
        let mut target = GenerationTarget::default();

        for item in &self.body {
            item.generate_handlers(&mut target)?;
        }

        Ok(())
    }
}

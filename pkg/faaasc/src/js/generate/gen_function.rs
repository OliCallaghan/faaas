use anyhow::Result;
use swc_ecma_ast::Function;

use super::{GenerateHandler, Generation};

impl GenerateHandler for Function {
    fn generate_split(&self, gen: &mut Generation) -> Result<()> {
        match &self.body {
            Some(body) => body.generate_split(gen),
            None => Ok(()),
        }
    }
}

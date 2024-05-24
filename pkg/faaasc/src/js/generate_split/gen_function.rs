use anyhow::Result;
use swc_ecma_ast::Function;

use super::{GenerateSplit, Generation};

impl GenerateSplit for Function {
    fn generate_split(&self, gen: &mut Option<Generation>) -> Result<()> {
        match &self.body {
            Some(body) => body.generate_split(gen),
            None => Ok(()),
        }
    }
}

use anyhow::Result;
use swc_ecma_ast::{Function, Ident};

use super::{GenerateWithTargetAndIdent, Generation};

impl GenerateWithTargetAndIdent for Function {
    fn generate(&self, gen_target: &mut Vec<Generation>, id: Ident) -> Result<()> {
        match &self.body {
            Some(body) => body.generate(gen_target, id),
            None => Ok(()),
        }
    }
}

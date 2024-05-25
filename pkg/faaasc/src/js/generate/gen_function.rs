use anyhow::Result;
use graphviz_rust::dot_structures::*;
use swc_ecma_ast::Function;

use super::ToGraphvizSubgraph;
use super::{GenerateHandler, Generation};

impl GenerateHandler for Function {
    fn generate_split(&self, gen: &mut Generation) -> Result<()> {
        match &self.body {
            Some(body) => body.generate_split(gen),
            None => Ok(()),
        }
    }
}

impl ToGraphvizSubgraph for Function {
    fn to_subgraph(&self, parent: &str) -> Option<Subgraph> {
        self.body.as_ref().and_then(|body| body.to_subgraph(parent))
    }
}

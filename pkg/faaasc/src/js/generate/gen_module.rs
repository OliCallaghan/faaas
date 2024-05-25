use anyhow::Result;
use graphviz_rust::dot_generator::*;
use graphviz_rust::dot_structures::*;
use swc_ecma_ast::Module;

use crate::js::generate::{GenerationTarget, ToGraphvizSubgraph};

use super::{Generate, GenerateHandlers, ToGraphvizGraph};

impl Generate for Module {
    fn generate(&mut self) -> Result<()> {
        let mut target = GenerationTarget::default();

        for item in &self.body {
            item.generate_handlers(&mut target)?;
        }

        Ok(())
    }
}

impl ToGraphvizGraph for Module {
    fn to_graph(&self) -> Graph {
        let mod_root = stmt!(node!("mod"));
        let mod_items = self
            .body
            .iter()
            .map(|item| item.to_subgraph("mod"))
            .flatten()
            .map(|sg| stmt!(sg))
            .chain([mod_root])
            .collect::<Vec<Stmt>>();

        graph!(strict di id!("t"), mod_items)
    }
}

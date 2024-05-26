use graphviz_rust::dot_generator::*;
use graphviz_rust::dot_structures::*;
use swc_ecma_ast::Module;

use super::{ToGraphvizGraph, ToGraphvizSubgraph};

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

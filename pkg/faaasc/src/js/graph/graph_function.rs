use graphviz_rust::dot_structures::*;
use swc_ecma_ast::Function;

use super::ToGraphvizSubgraph;

impl ToGraphvizSubgraph for Function {
    fn to_subgraph(&self, parent: &str) -> Option<Subgraph> {
        self.body.as_ref().and_then(|body| body.to_subgraph(parent))
    }
}

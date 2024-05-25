use anyhow::Result;
use graphviz_rust::dot_generator::*;
use graphviz_rust::dot_structures::*;
use swc_ecma_ast::FnDecl;

use super::{GenerateHandler, GenerateHandlers, GenerationTarget, ToGraphvizSubgraph};

impl GenerateHandlers for FnDecl {
    fn generate_handlers(&self, gen_target: &mut GenerationTarget) -> Result<()> {
        let id = self.ident.sym.as_str();
        println!("Found function with name: {}", id);

        let mut gen = gen_target.new_gen(self.ident.clone());

        return self.function.generate_split(&mut gen);
    }
}

impl ToGraphvizSubgraph for FnDecl {
    fn to_subgraph(&self, parent: &str) -> Option<Subgraph> {
        let fn_name = &self.ident.sym.as_str();

        let node_id = format!("{}_fn_name_{}", parent, fn_name);
        let subg_id = format!("sg_{}", node_id);

        let fn_subg = self
            .function
            .to_subgraph(&node_id)
            .expect("Function to have a body!");

        Some(subgraph!(&subg_id;
            node!(&node_id; attr!("label", fn_name)),
            edge!(node_id!(&node_id) => node_id!(parent)),
            fn_subg
        ))
    }
}

use graphviz_rust::dot_generator::*;
use graphviz_rust::dot_structures::*;
use swc_ecma_ast::TryStmt;

use super::{construct_id, util, ToGraphvizSubgraph};

impl ToGraphvizSubgraph for TryStmt {
    fn to_subgraph(&self, parent: &str) -> Option<Subgraph> {
        let (node_id, sg_id) = construct_id!(parent, "try");

        let (block_node_id, block_sg_id) = construct_id!(parent, "block");
        let (handler_node_id, handler_sg_id) = construct_id!(parent, "handler");
        let (finalizer_node_id, finalizer_sg_id) = construct_id!(parent, "finalizer");

        let block_node = subgraph!(block_sg_id;
            node!(block_node_id; attr!("label", "block")),
            edge!(node_id!(block_node_id) => node_id!(node_id))
        );
        let handler_node = subgraph!(handler_sg_id;
            node!(handler_node_id; attr!("label", "handler")),
            edge!(node_id!(handler_node_id) => node_id!(node_id))
        );
        let finalizer_node = subgraph!(finalizer_sg_id;
            node!(finalizer_node_id; attr!("label", "finalizer")),
            edge!(node_id!(finalizer_node_id) => node_id!(node_id))
        );

        let block_sg = self
            .block
            .to_subgraph(&block_node_id)
            .unwrap_or_else(|| util::empty_node(&block_node_id, "empty_block"));

        let handler_sg = self
            .handler
            .as_ref()
            .and_then(|handler| handler.body.to_subgraph(&handler_node_id))
            .unwrap_or_else(|| util::empty_node(&handler_node_id, "empty_handler"));

        let finalizer_sg = self
            .finalizer
            .as_ref()
            .and_then(|finalizer| finalizer.to_subgraph(&finalizer_node_id))
            .unwrap_or_else(|| util::empty_node(&finalizer_node_id, "empty_finalizer"));

        Some(subgraph!(&sg_id;
            node!(&node_id; attr!("label", r#""try catch stmt""#)),
            edge!(node_id!(node_id) => node_id!(parent)),
            handler_sg,
            block_sg,
            finalizer_sg,
            handler_node,
            block_node,
            finalizer_node
        ))
    }
}

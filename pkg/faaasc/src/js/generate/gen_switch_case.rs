use graphviz_rust::dot_generator::*;
use graphviz_rust::dot_structures::*;
use swc_ecma_ast::SwitchCase;

use super::{construct_id, util, ToGraphvizSubgraph};

impl ToGraphvizSubgraph for SwitchCase {
    fn to_subgraph(&self, parent: &str) -> Option<Subgraph> {
        let (node_id, sg_id) = construct_id!(parent, "switchcase");

        let (test_node_id, test_sg_id) = construct_id!(parent, "test");
        let (cons_node_id, cons_sg_id) = construct_id!(parent, "cons");

        let test_node = subgraph!(test_sg_id;
            node!(test_node_id; attr!("label", "test")),
            edge!(node_id!(test_node_id) => node_id!(node_id))
        );
        let cons_node = subgraph!(cons_sg_id;
            node!(cons_node_id; attr!("label", "cons")),
            edge!(node_id!(cons_node_id) => node_id!(node_id))
        );

        let test_sg = self
            .test
            .as_ref()
            .and_then(|test| test.to_subgraph(&test_node_id))
            .unwrap_or_else(|| util::empty_node(&test_node_id, "empty_test"));

        let cons_sgs = self
            .cons
            .iter()
            .scan(cons_node_id.clone(), |parent, stmt| {
                let (node_id, _) = construct_id!(parent, "stmt");
                let node = node!(&node_id; attr!("label", "stmt"));
                let edge = edge!(node_id!(node_id) => node_id!(parent); attr!("label", "next"));

                let stmt_sg = stmt.to_subgraph(&node_id);

                *parent = node_id;

                Some([
                    Some(stmt!(node)),
                    Some(stmt!(edge)),
                    stmt_sg.map(|g| stmt!(g)),
                ])
            })
            .flatten()
            .flatten();

        let cons_sg = cons_sgs
            .chain([
                stmt!(node!(&node_id; attr!("label", r#""switch case""#))),
                stmt!(edge!(node_id!(node_id) => node_id!(parent))),
                stmt!(test_sg),
                stmt!(test_node),
                stmt!(cons_node),
                stmt!(edge!(node_id!(test_node_id) => node_id!(node_id))),
                stmt!(edge!(node_id!(cons_node_id) => node_id!(node_id))),
            ])
            .collect::<Vec<Stmt>>();

        Some(subgraph!(&sg_id, cons_sg))
    }
}

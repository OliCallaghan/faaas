use graphviz_rust::dot_generator::*;
use graphviz_rust::dot_structures::*;
use swc_ecma_ast::SwitchStmt;

use super::{construct_id, util, ToGraphvizSubgraph};

impl ToGraphvizSubgraph for SwitchStmt {
    fn to_subgraph(&self, parent: &str) -> Option<Subgraph> {
        let (node_id, sg_id) = construct_id!(parent, "switchstmt");

        let (disc_node_id, disc_sg_id) = construct_id!(parent, "disc");
        let (cases_node_id, cases_sg_id) = construct_id!(parent, "cases");

        let disc_node = subgraph!(disc_sg_id;
            node!(disc_node_id; attr!("label", "disc")),
            edge!(node_id!(disc_node_id) => node_id!(node_id))
        );
        let cases_node = subgraph!(cases_sg_id;
            node!(cases_node_id; attr!("label", "cases")),
            edge!(node_id!(cases_node_id) => node_id!(node_id))
        );

        let disc_sg = self
            .discriminant
            .to_subgraph(&disc_node_id)
            .unwrap_or_else(|| util::empty_node(&disc_node_id, "empty_disc"));

        let cases_sgs = self
            .cases
            .iter()
            .enumerate()
            .map(|(i, c)| {
                let (case_node_id, case_sg_id) = construct_id!(&cases_node_id, i, "case");
                let sg = c.to_subgraph(&case_node_id).expect("case");

                subgraph!(&case_sg_id;
                    sg,
                    node!(&case_node_id; attr!("label", &format!(r#""case {}""#, i))),
                    edge!(node_id!(case_node_id) => node_id!(cases_node_id))
                )
            })
            // .flatten()
            .map(|sg| stmt!(sg));

        let cases_sg = cases_sgs
            .chain([
                stmt!(node!(&node_id; attr!("label", r#""switch stmt""#))),
                stmt!(edge!(node_id!(node_id) => node_id!(parent))),
                stmt!(disc_sg),
                stmt!(disc_node),
                stmt!(cases_node),
                stmt!(edge!(node_id!(disc_node_id) => node_id!(node_id))),
                stmt!(edge!(node_id!(cases_node_id) => node_id!(node_id))),
            ])
            .collect::<Vec<Stmt>>();

        Some(subgraph!(&sg_id, cases_sg))
    }
}

use graphviz_rust::dot_generator::*;
use graphviz_rust::dot_structures::*;
use swc_ecma_ast::VarDecl;

use super::ToGraphvizSubgraph;

impl ToGraphvizSubgraph for VarDecl {
    fn to_subgraph(&self, parent: &str) -> Option<Subgraph> {
        let serialised = format!(
            r#""{} {}""#,
            self.kind.as_str(),
            self.decls
                .iter()
                .map(|decl| format!(
                    "{}",
                    decl.name
                        .clone()
                        .ident()
                        .map_or("{ ... }".to_string(), |id| id.sym.to_string())
                ))
                .collect::<Vec<String>>()
                .join(" "),
        );

        let node_id = format!("{}_var_decl", parent);
        let subg_id = format!("sg_{}", node_id);

        Some(subgraph!(&subg_id;
            node!(&node_id; attr!("label", serialised)),
            edge!(node_id!(&node_id) => node_id!(parent))
        ))
    }
}

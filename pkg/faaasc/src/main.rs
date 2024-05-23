mod workflow;

use std::collections::HashMap;

use workflow::generate::Generate;
use workflow::primitives::{Primitive, Workflow};
use workflow::register::Register;

use anyhow::Result;
use wac_graph::{CompositionGraph, EncodeOptions};

fn main() -> Result<()> {
    // let workflow = Workflow(Primitive::Linear(
    //     Box::new(Primitive::Task("faaas:task1".to_string())),
    //     Box::new(Primitive::Linear(
    //         Box::new(Primitive::Task("faaas:task1".to_string())),
    //         Box::new(Primitive::Linear(
    //             Box::new(Primitive::Task("faaas:task1".to_string())),
    //             Box::new(Primitive::Task("faaas:task1".to_string())),
    //         )),
    //     )),
    // ));

    // let mut graph = CompositionGraph::new();

    // workflow.register(&mut graph, &HashMap::default())?;
    // workflow.generate(&mut graph)?;

    // let encoding = graph.encode(EncodeOptions::default())?;
    // let workflow_json = serde_json::to_string(&workflow).unwrap();

    // std::fs::write("composition.wasm", encoding)?;
    // std::fs::write("composition.json", workflow_json)?;

    Ok(())
}

mod workflow;

use workflow::generate::Generate;
use workflow::primitives::{Primitive, Workflow};
use workflow::register::Register;

use anyhow::Result;
use wac_graph::{CompositionGraph, EncodeOptions};

fn main() -> Result<()> {
    let workflow = Workflow(Primitive::Linear(
        Box::new(Primitive::Task("faaas:task1".to_string())),
        Box::new(Primitive::Linear(
            Box::new(Primitive::Task("faaas:task1".to_string())),
            Box::new(Primitive::Linear(
                Box::new(Primitive::Task("faaas:task1".to_string())),
                Box::new(Primitive::Task("faaas:task1".to_string())),
            )),
        )),
    ));

    let mut graph = CompositionGraph::new();

    workflow.register(&mut graph)?;
    workflow.generate(&mut graph)?;

    let encoding = graph.encode(EncodeOptions::default())?;

    std::fs::write("composition.wasm", encoding)?;

    Ok(())
}

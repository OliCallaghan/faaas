use anyhow::Result;
use wac_graph::{types::Package, CompositionGraph, EncodeOptions};

// const CONDITION_WASM: &[u8] =
//     include_bytes!("../node_modules/@faaas/condition/target/wasm32-wasi/release/condition.wasm");

const LINEAR_WASM: &[u8] =
    include_bytes!("../node_modules/@faaas/linear/target/wasm32-wasi/release/linear.wasm");

const TASK_WASM: &[u8] =
    include_bytes!("../node_modules/@faaas-example/task1/target/wasm32-wasi/release/task1.wasm");

fn main() -> Result<()> {
    let mut graph = CompositionGraph::new();

    let pkg = Package::from_bytes("faaas:linear", None, LINEAR_WASM, graph.types_mut())?;
    let linear = graph.register_package(pkg)?;

    // let pkg = Package::from_bytes("faaas:condition", None, CONDITION_WASM, graph.types_mut())?;
    // let condition = graph.register_package(pkg)?;

    let pkg = Package::from_bytes("faaas-example:task1", None, TASK_WASM, graph.types_mut())?;
    let task1 = graph.register_package(pkg)?;

    // Create 2 instances of linear.
    let wf_l1 = graph.instantiate(linear);
    let wf_l2 = graph.instantiate(linear);

    // Create 3 instances of task 1.
    let wf_t1 = graph.instantiate(task1);
    let wf_t2 = graph.instantiate(task1);
    let wf_t3 = graph.instantiate(task1);

    // Get task call handle aliases.
    let wf_t1_callable_export = graph.alias_instance_export(wf_t1, "faaas:task/callable")?;
    let wf_t2_callable_export = graph.alias_instance_export(wf_t2, "faaas:task/callable")?;
    let wf_t3_callable_export = graph.alias_instance_export(wf_t3, "faaas:task/callable")?;

    let wf_t1_call_export = graph.alias_instance_export(wf_t1_callable_export, "call")?;
    let wf_t2_call_export = graph.alias_instance_export(wf_t2_callable_export, "call")?;
    let wf_t3_call_export = graph.alias_instance_export(wf_t3_callable_export, "call")?;

    // Get linear call handle aliases.
    let wf_l1_callable_export = graph.alias_instance_export(wf_l1, "faaas:task/callable")?;
    let wf_l2_callable_export = graph.alias_instance_export(wf_l2, "faaas:task/callable")?;

    let wf_l1_call_export = graph.alias_instance_export(wf_l1_callable_export, "call")?;
    let wf_l2_call_export = graph.alias_instance_export(wf_l2_callable_export, "call")?;

    // Configure linear 1 to call task 1 and linear 2.
    graph.set_instantiation_argument(wf_l1, "task-fst", wf_t1_call_export)?;
    graph.set_instantiation_argument(wf_l1, "task-snd", wf_l2_call_export)?;

    // Configure linear 2 to call tasks 2 and 3.
    graph.set_instantiation_argument(wf_l2, "task-fst", wf_t2_call_export)?;
    graph.set_instantiation_argument(wf_l2, "task-snd", wf_t3_call_export)?;

    // Configure Composite Export
    graph.export(wf_l1_callable_export, "faaas:task/callable")?;

    let encoding = graph.encode(EncodeOptions::default())?;

    std::fs::write("composition.wasm", encoding)?;

    Ok(())
}

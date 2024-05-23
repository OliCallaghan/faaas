use anyhow::Result;
use wac_graph::{CompositionGraph, NodeId};

use super::{Primitive, Workflow};

pub trait Generate {
    fn generate(&self, graph: &mut CompositionGraph) -> Result<(NodeId, NodeId)>;
}

impl Generate for Primitive {
    fn generate(&self, graph: &mut CompositionGraph) -> Result<(NodeId, NodeId)> {
        match self {
            Primitive::Task(task_id) => {
                let (task_pkg_id, _) = graph
                    .get_package_by_name(task_id, None)
                    .expect("package to be registered");

                let task = graph.instantiate(task_pkg_id);
                let task_callable = graph.alias_instance_export(task, "faaas:task/callable")?;
                let task_identifiable =
                    graph.alias_instance_export(task, "faaas:task/identifiable")?;

                Ok((task_callable, task_identifiable))
            }
            Primitive::Linear(p_fst, p_snd) => {
                let (linear_pkg_id, _) = graph
                    .get_package_by_name("faaas:linear", None)
                    .expect("linear to be registered");

                let linear = graph.instantiate(linear_pkg_id);
                let linear_callable = graph.alias_instance_export(linear, "faaas:task/callable")?;
                let linear_identifiable =
                    graph.alias_instance_export(linear, "faaas:task/identifiable")?;

                let (p1_callable, p1_identifiable) = p_fst.generate(graph)?;
                let (p2_callable, p2_identifiable) = p_snd.generate(graph)?;

                let p1_call = graph.alias_instance_export(p1_callable, "call")?;
                let p2_call = graph.alias_instance_export(p2_callable, "call")?;

                let p1_identify = graph.alias_instance_export(p1_identifiable, "identify")?;
                let p2_identify = graph.alias_instance_export(p2_identifiable, "identify")?;

                graph.set_instantiation_argument(linear, "task-fst", p1_call)?;
                graph.set_instantiation_argument(linear, "task-snd", p2_call)?;

                graph.set_instantiation_argument(linear, "identify-fst", p1_identify)?;
                graph.set_instantiation_argument(linear, "identify-snd", p2_identify)?;

                Ok((linear_callable, linear_identifiable))
            }
        }
    }
}

impl Generate for Workflow {
    fn generate(&self, graph: &mut CompositionGraph) -> Result<(NodeId, NodeId)> {
        let (callable, identifiable) = self.0.generate(graph)?;

        graph.export(callable, "faaas:task/callable")?;
        graph.export(identifiable, "faaas:task/identifiable")?;

        Ok((callable, identifiable))
    }
}

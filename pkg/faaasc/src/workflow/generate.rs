use anyhow::Result;
use wac_graph::{CompositionGraph, NodeId};

use crate::{Primitive, Workflow};

pub trait Generate {
    fn generate(&self, g: &mut CompositionGraph) -> Result<(NodeId, NodeId)>;
}

impl Generate for Primitive {
    fn generate(&self, g: &mut CompositionGraph) -> Result<(NodeId, NodeId)> {
        match self {
            // Ignore task_id and refer to TASK_WASM.
            Primitive::Task(task_id) => {
                let (task_pkg_id, _) = g
                    .get_package_by_name(task_id, None)
                    .expect("package to be registered");

                let task = g.instantiate(task_pkg_id);
                let task_callable = g.alias_instance_export(task, "faaas:task/callable")?;
                let task_identifiable = g.alias_instance_export(task, "faaas:task/identifiable")?;

                Ok((task_callable, task_identifiable))
            }
            Primitive::Linear(p1, p2) => {
                let (linear_pkg_id, _) = g
                    .get_package_by_name("faaas:linear", None)
                    .expect("linear to be registered");

                let linear = g.instantiate(linear_pkg_id);
                let linear_callable = g.alias_instance_export(linear, "faaas:task/callable")?;
                let linear_identifiable =
                    g.alias_instance_export(linear, "faaas:task/identifiable")?;

                let (p1_callable, p1_identifiable) = p1.generate(g)?;
                let (p2_callable, p2_identifiable) = p2.generate(g)?;

                let p1_call = g.alias_instance_export(p1_callable, "call")?;
                let p2_call = g.alias_instance_export(p2_callable, "call")?;

                let p1_identify = g.alias_instance_export(p1_identifiable, "identify")?;
                let p2_identify = g.alias_instance_export(p2_identifiable, "identify")?;

                g.set_instantiation_argument(linear, "task-fst", p1_call)?;
                g.set_instantiation_argument(linear, "task-snd", p2_call)?;

                g.set_instantiation_argument(linear, "identify-fst", p1_identify)?;
                g.set_instantiation_argument(linear, "identify-snd", p2_identify)?;

                Ok((linear_callable, linear_identifiable))
            }
        }
    }
}

impl Generate for Workflow {
    fn generate(&self, g: &mut CompositionGraph) -> Result<(NodeId, NodeId)> {
        let (callable, identifiable) = self.0.generate(g)?;

        g.export(callable, "faaas:task/callable")?;
        g.export(identifiable, "faaas:task/identifiable")?;

        Ok((callable, identifiable))
    }
}

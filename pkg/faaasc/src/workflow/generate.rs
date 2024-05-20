use anyhow::Result;
use wac_graph::{CompositionGraph, NodeId};

use crate::{Primitive, Workflow};

pub trait Generate {
    fn generate(&self, g: &mut CompositionGraph) -> Result<NodeId>;
}

impl Generate for Primitive {
    fn generate(&self, g: &mut CompositionGraph) -> Result<NodeId> {
        match self {
            // Ignore task_id and refer to TASK_WASM.
            Primitive::Task(task_id) => {
                let (task_pkg_id, _) = g
                    .get_package_by_name(task_id, None)
                    .expect("package to be registered");

                let task = g.instantiate(task_pkg_id);
                let task_callable = g.alias_instance_export(task, "faaas:task/callable")?;

                Ok(task_callable)
            }
            Primitive::Linear(p1, p2) => {
                let (linear_pkg_id, _) = g
                    .get_package_by_name("faaas:linear", None)
                    .expect("linear to be registered");

                let linear = g.instantiate(linear_pkg_id);
                let linear_callable = g.alias_instance_export(linear, "faaas:task/callable")?;

                let p1_callable = p1.generate(g)?;
                let p2_callable = p2.generate(g)?;

                let p1_call = g.alias_instance_export(p1_callable, "call")?;
                let p2_call = g.alias_instance_export(p2_callable, "call")?;

                g.set_instantiation_argument(linear, "task-fst", p1_call)?;
                g.set_instantiation_argument(linear, "task-snd", p2_call)?;

                Ok(linear_callable)
            }
        }
    }
}

impl Generate for Workflow {
    fn generate(&self, g: &mut CompositionGraph) -> Result<NodeId> {
        let callable = self.0.generate(g)?;

        g.export(callable, "faaas:task/callable")?;

        Ok(callable)
    }
}

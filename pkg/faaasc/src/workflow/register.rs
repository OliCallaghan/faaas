use anyhow::Result;
use wac_graph::{types::Package, CompositionGraph};

use crate::{Primitive, Workflow};

const LINEAR_WASM: &[u8] =
    include_bytes!("../../node_modules/@faaas/linear/target/wasm32-wasi/release/linear.wasm");

const TASK_WASM: &[u8] =
    include_bytes!("../../node_modules/@faaas-example/task1/target/wasm32-wasi/release/task1.wasm");

pub trait Register {
    fn register(&self, g: &mut CompositionGraph) -> Result<()>;
}

impl Register for Workflow {
    fn register(&self, g: &mut CompositionGraph) -> Result<()> {
        self.0.register(g)
    }
}

impl Register for Primitive {
    fn register(&self, g: &mut CompositionGraph) -> Result<()> {
        match self {
            Primitive::Task(task_id) => {
                if g.get_package_by_name(task_id, None).is_none() {
                    let pkg = Package::from_bytes(task_id, None, TASK_WASM, g.types_mut())?;
                    g.register_package(pkg)?;
                }
                Ok(())
            }
            Primitive::Linear(p1, p2) => {
                if g.get_package_by_name("faaas:linear", None).is_none() {
                    let pkg =
                        Package::from_bytes("faaas:linear", None, LINEAR_WASM, g.types_mut())?;
                    g.register_package(pkg)?;
                }

                p1.register(g)?;
                p2.register(g)?;

                Ok(())
            }
        }
    }
}

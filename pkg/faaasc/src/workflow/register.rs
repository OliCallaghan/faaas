use std::{collections::HashMap, sync::Arc};

use anyhow::Result;
use bytes::Bytes;
use wac_graph::{types::Package, CompositionGraph};

use crate::{Primitive, Workflow};

const LINEAR_WASM: &[u8] =
    include_bytes!("../../node_modules/@faaas/linear/target/wasm32-wasi/release/linear.wasm");

pub trait Register {
    fn register(&self, g: &mut CompositionGraph, d: &HashMap<String, Arc<Bytes>>) -> Result<()>;
}

impl Register for Workflow {
    fn register(&self, g: &mut CompositionGraph, d: &HashMap<String, Arc<Bytes>>) -> Result<()> {
        self.0.register(g, d)
    }
}

impl Register for Primitive {
    fn register(&self, g: &mut CompositionGraph, d: &HashMap<String, Arc<Bytes>>) -> Result<()> {
        match self {
            Primitive::Task(task_id) => {
                if g.get_package_by_name(task_id, None).is_none() {
                    let pkg = Package::from_bytes(
                        task_id,
                        None,
                        d.get(task_id).expect("task to be resolved").to_vec(),
                        g.types_mut(),
                    )?;
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

                p1.register(g, d)?;
                p2.register(g, d)?;

                Ok(())
            }
        }
    }
}

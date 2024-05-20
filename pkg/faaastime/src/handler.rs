use std::sync::Arc;

use wasmtime::component::InstancePre;
use wasmtime::Engine;

use crate::state::FaaastimeState;

#[derive(Clone)]
pub struct FaaasHandler {
    pub engine: Engine,
    pub component_pre: InstancePre<FaaastimeState>,
    pub task_pre: InstancePre<FaaastimeState>,
}

impl FaaasHandler {
    pub fn new(
        engine: &Engine,
        component_pre: &InstancePre<FaaastimeState>,
        task_pre: &InstancePre<FaaastimeState>,
    ) -> Self {
        Self {
            engine: engine.clone(),
            component_pre: component_pre.clone(),
            task_pre: task_pre.clone(),
        }
    }
}

#[derive(Clone)]
pub struct ProxyHandler(pub Arc<FaaasHandler>);

impl ProxyHandler {
    pub fn new(
        engine: &Engine,
        component_pre: &InstancePre<FaaastimeState>,
        task_pre: &InstancePre<FaaastimeState>,
    ) -> Self {
        Self(Arc::new(FaaasHandler::new(engine, component_pre, task_pre)))
    }
}

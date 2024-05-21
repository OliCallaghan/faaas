use std::sync::Arc;

use wasmtime::Engine;

use crate::registry::FaaasRegistry;

#[derive(Clone)]
pub struct FaaasInvocationHandler(pub Arc<FaaasRegistry>, pub Arc<Engine>);

impl FaaasInvocationHandler {
    pub fn new(registry: FaaasRegistry, engine: Engine) -> Self {
        Self(Arc::new(registry), Arc::new(engine))
    }
}

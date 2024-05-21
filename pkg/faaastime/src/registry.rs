use std::sync::Arc;

use anyhow::Result;
use moka::future::Cache;
use wasmtime::{
    component::{Component, InstancePre, Linker},
    Config, Engine,
};

use crate::state::FaaastimeState;

pub struct FaaasRegistry {
    engine: Engine,
    linker: Linker<FaaastimeState>,
    cache: Cache<String, Arc<InstancePre<FaaastimeState>>>,
}

impl FaaasRegistry {
    pub fn new_engine() -> Result<Engine> {
        let mut cfg = Config::new();
        cfg.wasm_component_model(true);
        cfg.async_support(true);

        let engine = Engine::new(&cfg)?;

        Ok(engine)
    }

    pub fn new(engine: &Engine) -> Result<Self> {
        let mut linker = Linker::new(&engine);

        Self::add_to_linker(&mut linker)?;

        Ok(Self {
            engine: engine.clone(),
            linker,
            cache: Cache::new(500),
        })
    }

    fn add_to_linker(linker: &mut Linker<FaaastimeState>) -> Result<()> {
        wasmtime_wasi::add_to_linker_async(linker)?;
        wasmtime_wasi_http::proxy::add_only_http_to_linker(linker)?;

        // Add task to linker
        crate::state::add_to_linker(linker)?;

        Ok(())
    }

    // For caching instantiated modules for use later :)
    pub async fn instantiate_pre(
        &self,
        component_id: &str,
    ) -> Result<Arc<InstancePre<FaaastimeState>>> {
        let cache = self.cache.clone();

        let component = cache
            .get_with(component_id.to_string(), async move {
                let component = self.load_component(component_id).await;
                let instance = self.link_component(&component);

                Arc::new(instance)
            })
            .await;

        Ok(component)
    }

    async fn load_component(&self, component_id: &str) -> Component {
        let filepath = match component_id {
            "faaas:runjs" => "../runjs/target/wasm32-wasi/release/runjs.wasm",
            "task:one" => "../faaasc/composition.wasm",
            _ => panic!("Unknown task"),
        };

        Component::from_file(&self.engine, filepath).unwrap()
    }

    fn link_component(&self, component: &Component) -> InstancePre<FaaastimeState> {
        self.linker.instantiate_pre(component).unwrap()
    }
}

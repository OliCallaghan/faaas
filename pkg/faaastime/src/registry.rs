mod storage;

use std::sync::Arc;

use anyhow::{Error, Result};
use async_trait::async_trait;
use bytes::Bytes;
use moka::future::Cache;
use wasmtime::{
    component::{Component, InstancePre, Linker},
    Config, Engine,
};

use faaasc::workflow::{extract::ComponentResolver, primitives::Workflow};

use crate::state::FaaastimeState;

use self::storage::Storage;

pub struct FaaasRegistry {
    engine: Engine,
    linker: Linker<FaaastimeState>,

    instance_pre_cache: Cache<String, Arc<InstancePre<FaaastimeState>>>,
    component_cache: Cache<String, Arc<Bytes>>,

    storage: Storage,
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
            instance_pre_cache: Cache::new(500),
            component_cache: Cache::new(500),
            storage: Storage::new(),
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
        let instance_pre = self
            .instance_pre_cache
            .try_get_with(
                component_id.to_string(),
                self.resolve_and_instantiate_component(component_id),
            )
            .await
            .map_err(|_| Error::msg("fdsfds"))?;

        Ok(instance_pre)
    }

    async fn resolve_and_instantiate_component(
        &self,
        component_id: &str,
    ) -> Result<Arc<InstancePre<FaaastimeState>>> {
        println!("Looking for {}", component_id);
        // TODO: Sort this shit out
        let bytes = if component_id.starts_with("/workflows/") {
            println!("Workflow");
            let workflow_str = self.storage.get_workflow_str(component_id).await?;
            let workflow = Workflow::from_str(&workflow_str)?;
            let deps = workflow.extract_and_resolve(self).await?;

            let workflow = workflow.compile(&deps)?;

            println!("Found workflow!");
            Arc::new(workflow)
        } else {
            println!("Task");
            self.resolve_component(component_id).await?
        };

        println!("Building component {}!", component_id);

        let component = Component::from_binary(&self.engine, &bytes)?;
        let ip = self.linker.instantiate_pre(&component)?;

        Ok(Arc::new(ip))
    }
}

#[async_trait]
impl ComponentResolver for FaaasRegistry {
    async fn resolve_component(&self, component_id: &str) -> Result<Arc<Bytes>> {
        self.component_cache
            .try_get_with(
                component_id.to_string(),
                self.storage.get_component_bytes(component_id),
            )
            .await
            .map_err(|_| Error::msg("Failed to fetch component"))
    }
}

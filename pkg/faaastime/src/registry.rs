use std::{collections::HashMap, rc::Rc};

use anyhow::Result;
use wasmtime::{
    component::{Component, InstancePre, Linker},
    Config, Engine,
};

use crate::state::FaaastimeState;

pub struct FaaasRegistry<'e> {
    engine: &'e Engine,
    linker: Linker<FaaastimeState>,
    components_cache: HashMap<String, Component>,
    instance_pre_cache: HashMap<String, Rc<InstancePre<FaaastimeState>>>,
}

impl<'e> FaaasRegistry<'e> {
    pub fn new_engine() -> Result<Engine> {
        let mut cfg = Config::new();
        cfg.wasm_component_model(true);
        cfg.async_support(true);

        let engine = Engine::new(&cfg)?;

        Ok(engine)
    }

    pub fn new(engine: &'e Engine) -> Result<Self> {
        let mut linker = Linker::new(&engine);

        Self::add_to_linker(&mut linker)?;

        Ok(Self {
            engine,
            linker,
            components_cache: Default::default(),
            instance_pre_cache: Default::default(),
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
    pub fn instantiate_pre(
        &mut self,
        component_id: &str,
    ) -> Result<Rc<InstancePre<FaaastimeState>>> {
        let component = self.load(component_id)?;
        let instance_pre = self.link(component_id, &component)?;

        Ok(instance_pre)
    }

    fn load(&mut self, component_id: &str) -> Result<Component> {
        let filepath = match component_id {
            "faaas:runjs" => "../runjs/target/wasm32-wasi/release/runjs.wasm",
            "task:one" => "../faaasc/composition.wasm",
            _ => panic!("Unknown task"),
        };

        let cached = self
            .components_cache
            .entry(component_id.to_string())
            .or_insert_with(|| {
                Component::from_file(&self.engine, filepath).expect("WASM to compile")
            })
            .to_owned();

        Ok(cached)
    }

    fn link(
        &mut self,
        component_id: &str,
        component: &Component,
    ) -> Result<Rc<InstancePre<FaaastimeState>>> {
        let cached = self
            .instance_pre_cache
            .entry(component_id.to_string())
            .or_insert_with(|| {
                Rc::new(
                    self.linker
                        .instantiate_pre(component)
                        .expect("WASM to link"),
                )
            });

        Ok(cached.clone())
    }
}

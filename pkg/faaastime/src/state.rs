use wasmtime::component::Resource;
use wasmtime::component::ResourceTable;
use wasmtime_wasi::{WasiCtx, WasiCtxBuilder, WasiView};
use wasmtime_wasi_http::{WasiHttpCtx, WasiHttpView};

use self::bindings::faaas::task::types::Host;
use self::bindings::faaas::task::types::HostTaskContext;
use self::bindings::faaas::task::types::HostTaskError;

use self::bindings::faaas::task::types::TaskContext;
use self::bindings::faaas::task::types::Value;

mod types {
    use std::collections::HashMap;

    use serde::Deserialize;
    use serde::Serialize;

    use super::bindings::faaas::task::types::TaskStatus;
    use super::bindings::faaas::task::types::Value;

    #[derive(Clone, Serialize, Deserialize)]
    pub struct TaskContext {
        pub lenses: Vec<String>,
        pub data: HashMap<String, Value>,
        pub value: u32,
        pub status: TaskStatus,
    }

    impl TaskContext {
        pub fn new() -> Self {
            Self {
                lenses: Vec::new(),
                data: HashMap::new(),
                value: 0,
                status: TaskStatus::Success,
            }
        }

        pub fn to_bytes(&self) -> Vec<u8> {
            serde_json::to_string(&self.data).unwrap().into_bytes()
        }
    }
}

pub mod bindings {
    wasmtime::component::bindgen!({
        path: "wit",
        interfaces: "
            import faaas:task/types;
        ",
        async: false,
        trappable_imports: true,
        with: {
            "faaas:task/types/task-context": super::types::TaskContext,
        },
        additional_derives: [
            serde::Deserialize,
            serde::Serialize,
        ]
    });
}

wasmtime::component::bindgen!({
    world: "faaas:faaastime/faaastime",
    async: true,
    with: {
        "faaas:task": bindings::faaas::task,
    },
});

fn type_annotate_faaas_task<T, F>(val: F) -> F
where
    F: Fn(&mut T) -> &mut dyn FaaasTaskView,
{
    val
}

pub fn add_to_linker<T>(l: &mut wasmtime::component::Linker<T>) -> anyhow::Result<()>
where
    T: FaaasTaskView,
{
    let closure = type_annotate_faaas_task::<T, _>(|t| t);

    self::bindings::faaas::task::types::add_to_linker_get_host(l, closure)?;

    Ok(())
}

#[derive(Default)]
pub struct FaaasTaskCtx {}

pub trait FaaasTaskView: Send {
    fn ctx(&mut self) -> &mut FaaasTaskCtx;
    fn table(&mut self) -> &mut ResourceTable;

    fn new_task_ctx(&mut self) -> wasmtime::Result<Resource<TaskContext>> {
        let task_ctx = TaskContext::new();

        Ok(self.table().push(task_ctx)?)
    }
}

impl HostTaskContext for dyn FaaasTaskView + '_ {
    fn get(
        &mut self,
        rep: wasmtime::component::Resource<TaskContext>,
        key: String,
    ) -> wasmtime::Result<Option<Value>> {
        let ctx = self.table().get(&rep)?;
        let val = ctx.data.get(&key).map(|v| v.to_owned());

        Ok(val)
    }

    fn set(
        &mut self,
        rep: wasmtime::component::Resource<TaskContext>,
        key: String,
        value: Value,
    ) -> wasmtime::Result<()> {
        let ctx = self.table().get_mut(&rep)?;
        ctx.data.insert(key, value);

        Ok(())
    }

    fn drop(&mut self, rep: wasmtime::component::Resource<TaskContext>) -> wasmtime::Result<()> {
        self.table().delete(rep)?;

        Ok(())
    }

    fn clone(
        &mut self,
        rep: wasmtime::component::Resource<TaskContext>,
    ) -> wasmtime::Result<wasmtime::component::Resource<TaskContext>> {
        let ctx = self.table().get_mut(&rep)?;
        let ctx_clone = ctx.clone();

        Ok(self.table().push(ctx_clone)?)
    }

    fn get_status(
        &mut self,
        rep: wasmtime::component::Resource<TaskContext>,
    ) -> wasmtime::Result<bindings::faaas::task::types::TaskStatus> {
        let out = self.table().get(&rep)?;

        Ok(out.status)
    }

    fn set_status(
        &mut self,
        rep: wasmtime::component::Resource<TaskContext>,
        status: bindings::faaas::task::types::TaskStatus,
    ) -> wasmtime::Result<()> {
        let out = self.table().get_mut(&rep)?;
        out.status = status;

        Ok(())
    }

    fn lens(
        &mut self,
        rep: wasmtime::component::Resource<TaskContext>,
        id: String,
    ) -> wasmtime::Result<wasmtime::component::Resource<TaskContext>> {
        let ctx = self.table().get_mut(&rep)?;
        let mut ctx_with_lens = ctx.clone();

        ctx_with_lens.lenses.push(id);

        println!("Creating ctx with lens {:?}", ctx_with_lens.lenses);

        Ok(self.table().push(ctx_with_lens)?)
    }
}

impl HostTaskError for dyn FaaasTaskView + '_ {
    fn drop(
        &mut self,
        rep: wasmtime::component::Resource<bindings::faaas::task::types::TaskError>,
    ) -> wasmtime::Result<()> {
        self.table().delete(rep)?;

        Ok(())
    }
}

pub struct FaaastimeState {
    ctx: WasiCtx,
    ctx_http: WasiHttpCtx,
    ctx_task: FaaasTaskCtx,
    table: ResourceTable,
}

impl FaaasTaskView for FaaastimeState {
    fn ctx(&mut self) -> &mut FaaasTaskCtx {
        &mut self.ctx_task
    }

    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }
}

impl WasiView for FaaastimeState {
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.ctx
    }

    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }
}

impl WasiHttpView for FaaastimeState {
    fn ctx(&mut self) -> &mut WasiHttpCtx {
        &mut self.ctx_http
    }

    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }
}

impl Host for dyn FaaasTaskView + '_ {}

impl FaaastimeState {
    pub fn new() -> Self {
        let mut wasi = WasiCtxBuilder::new();

        wasi.preopened_dir(
            "js",
            "js",
            wasmtime_wasi::DirPerms::all(),
            wasmtime_wasi::FilePerms::all(),
        )
        .expect("open js");

        Self {
            ctx: wasi.build(),
            ctx_http: WasiHttpCtx::new(),
            ctx_task: FaaasTaskCtx::default(),
            table: ResourceTable::new(),
        }
    }
}

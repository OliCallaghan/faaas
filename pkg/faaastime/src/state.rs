use wasmtime::component::bindgen;
use wasmtime::component::Resource;
use wasmtime::component::ResourceTable;
use wasmtime_wasi::{WasiCtx, WasiCtxBuilder, WasiView};
use wasmtime_wasi_http::{WasiHttpCtx, WasiHttpView};

use self::bindings::faaas::task::types::Host;
use self::bindings::faaas::task::types::HostTaskContext;
use self::bindings::faaas::task::types::HostTaskError;
use self::bindings::faaas::task::types::HostTaskOutput;
use self::exports::faaas::task::callable::TaskOutput;

use self::bindings::faaas::task::types::TaskContext;

mod types {
    use super::bindings::faaas::task::types::TaskStatus;

    #[derive(Clone)]
    pub struct TaskContext {
        pub value: u32,
    }

    pub struct TaskOutput {
        pub status: TaskStatus,
    }

    impl TaskOutput {
        pub fn new() -> Self {
            Self {
                status: TaskStatus::Success,
            }
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
        with: {
            "faaas:task/types/task-context": super::types::TaskContext,
            "faaas:task/types/task-output": super::types::TaskOutput
        }
    });
}

bindgen!({
    world: "faaas:faaastime/faaastime",
    async: true,
    with: {
        "faaas:task": bindings::faaas::task,
    },
});

pub fn add_to_linker<T>(l: &mut wasmtime::component::Linker<T>) -> anyhow::Result<()>
where
    T: FaaasTaskView + self::bindings::faaas::task::types::Host,
{
    self::bindings::faaas::task::types::add_to_linker(l, |t| t)?;

    Ok(())
}

#[derive(Default)]
pub struct FaaasTaskCtx {}

pub trait FaaasTaskView: Send {
    fn ctx(&mut self) -> &mut FaaasTaskCtx;
    fn table(&mut self) -> &mut ResourceTable;

    fn new_task_ctx(&mut self) -> wasmtime::Result<Resource<TaskContext>> {
        let task_ctx = TaskContext { value: 5 };

        Ok(self.table().push(task_ctx)?)
    }
}

impl<V: FaaasTaskView> HostTaskContext for V {
    fn get(
        &mut self,
        rep: wasmtime::component::Resource<TaskContext>,
        key: String,
    ) -> wasmtime::Result<u32> {
        let ctx = self.table().get(&rep)?;
        let val = ctx.value; // Need to change internal structure to get key

        Ok(val)
    }

    fn set(
        &mut self,
        rep: wasmtime::component::Resource<TaskContext>,
        key: String,
        value: u32,
    ) -> wasmtime::Result<()> {
        let ctx = self.table().get_mut(&rep)?;
        ctx.value = value; // Need to set key value to this.

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

    fn merge(
        &mut self,
        fst: wasmtime::component::Resource<TaskContext>,
        snd: wasmtime::component::Resource<TaskContext>,
    ) -> wasmtime::Result<wasmtime::component::Resource<TaskContext>> {
        todo!("Need to do this, not sure whether the function signature is correct even actualy.")
    }
}

impl<V: FaaasTaskView> HostTaskOutput for V {
    fn new(
        &mut self,
    ) -> wasmtime::Result<wasmtime::component::Resource<bindings::faaas::task::types::TaskOutput>>
    {
        let out = TaskOutput::new();

        Ok(self.table().push(out)?)
    }

    fn drop(
        &mut self,
        rep: wasmtime::component::Resource<bindings::faaas::task::types::TaskOutput>,
    ) -> wasmtime::Result<()> {
        self.table().delete(rep)?;

        Ok(())
    }

    fn get_status(
        &mut self,
        rep: wasmtime::component::Resource<bindings::faaas::task::types::TaskOutput>,
    ) -> wasmtime::Result<bindings::faaas::task::types::TaskStatus> {
        let out = self.table().get(&rep)?;

        Ok(out.status)
    }

    fn set_status(
        &mut self,
        rep: wasmtime::component::Resource<bindings::faaas::task::types::TaskOutput>,
        status: bindings::faaas::task::types::TaskStatus,
    ) -> wasmtime::Result<()> {
        let out = self.table().get_mut(&rep)?;
        out.status = status;

        Ok(())
    }
}

impl<V: FaaasTaskView> HostTaskError for V {
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

impl Host for FaaastimeState {}

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
            ctx_http: WasiHttpCtx {},
            ctx_task: FaaasTaskCtx::default(),
            table: ResourceTable::new(),
        }
    }
}

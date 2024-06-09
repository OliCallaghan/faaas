use wasmtime::component::Resource;
use wasmtime::component::ResourceTable;
use wasmtime_wasi::{WasiCtx, WasiCtxBuilder, WasiView};
use wasmtime_wasi_http::{WasiHttpCtx, WasiHttpView};

use self::bindings::faaas::task::types::Host;
use self::bindings::faaas::task::types::HostTaskContext;
use self::bindings::faaas::task::types::HostTaskError;

use self::bindings::faaas::task::types::TaskContext;
use self::bindings::faaas::task::types::Value;

pub mod types {
    use std::collections::HashMap;

    use serde::Deserialize;
    use serde::Serialize;

    use faaasmq::{MqTaskContext, MqValue};

    use super::bindings::faaas::task::types::Value;

    impl Into<TaskContext> for MqTaskContext {
        fn into(self) -> TaskContext {
            TaskContext {
                id: self.id,
                task_id: self.task_id,
                args: self.args.into_iter().map(|v| v.into()).collect(),
                data: self.state.into_iter().map(|(k, v)| (k, v.into())).collect(),
                continuation: self.continuation,
                continuation_args: self
                    .continuation_args
                    .into_iter()
                    .map(|v| v.into())
                    .collect(),
            }
        }
    }

    impl Into<Value> for MqValue {
        fn into(self) -> Value {
            match self {
                MqValue::Bool(v) => Value::BoolVal(v),
                MqValue::Int(v) => Value::S32Val(v),
                MqValue::Uint(v) => Value::U32Val(v),
                MqValue::Float(v) => Value::F64Val(v),
                MqValue::String(v) => Value::StrVal(v),
                MqValue::Bytes(v) => Value::BufVal(v),
            }
        }
    }

    impl From<TaskContext> for MqTaskContext {
        fn from(ctx: TaskContext) -> Self {
            MqTaskContext {
                id: ctx.id,
                task_id: ctx.task_id,
                args: ctx.args.into_iter().map(|v| v.into()).collect(),
                state: ctx.data.into_iter().map(|(k, v)| (k, v.into())).collect(),
                continuation: ctx.continuation,
                continuation_args: ctx
                    .continuation_args
                    .into_iter()
                    .map(|v| v.into())
                    .collect(),
            }
        }
    }

    impl From<Value> for MqValue {
        fn from(v: Value) -> Self {
            match v {
                Value::BoolVal(v) => MqValue::Bool(v),
                Value::S32Val(v) => MqValue::Int(v),
                Value::U32Val(v) => MqValue::Uint(v),
                Value::F64Val(v) => MqValue::Float(v),
                Value::StrVal(v) => MqValue::String(v),
                Value::BufVal(v) => MqValue::Bytes(v),
            }
        }
    }

    #[derive(Clone, Serialize, Deserialize)]
    pub struct TaskContext {
        pub id: String,
        pub task_id: String,
        pub args: Vec<Value>,
        pub data: HashMap<String, Value>,
        pub continuation: Option<String>,
        pub continuation_args: Vec<Value>,
    }

    pub enum TaskStatus {
        Continuation(TaskContext),
        Done(TaskContext),
    }

    impl TaskContext {
        pub fn new(id: &str, task_id: &str) -> Self {
            Self {
                id: id.to_owned(),
                task_id: task_id.to_owned(),
                args: Vec::new(),
                data: Default::default(),
                continuation: None,
                continuation_args: Vec::new(),
            }
        }

        pub fn into_continuation(self) -> TaskStatus {
            match self.continuation {
                Some(continuation) => TaskStatus::Continuation(Self {
                    id: self.id,
                    task_id: continuation,
                    args: self.continuation_args,
                    data: self.data,
                    continuation: None,
                    continuation_args: vec![],
                }),
                None => TaskStatus::Done(self),
            }
        }

        pub fn continuation(&mut self) {
            if let Some(continuation) = self.continuation.take() {
                self.task_id = continuation;
                self.args = self.continuation_args.clone();
                self.continuation_args = vec![]
            }
        }

        pub fn set_continuation(&mut self, task_id: &str, args: Vec<Value>) {
            self.continuation = Some(task_id.to_owned());
            self.continuation_args = args;
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

    fn new_task_ctx(&mut self, task_ctx: TaskContext) -> wasmtime::Result<Resource<TaskContext>> {
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

    fn set_continuation_id(
        &mut self,
        rep: wasmtime::component::Resource<TaskContext>,
        task_id: String,
    ) -> wasmtime::Result<()> {
        let ctx = self.table().get_mut(&rep)?;

        println!("Setting continuation id {}", task_id);
        ctx.continuation = Some(task_id);

        Ok(())
    }

    fn set_continuation_args(
        &mut self,
        rep: wasmtime::component::Resource<TaskContext>,
        args: Vec<Value>,
    ) -> wasmtime::Result<()> {
        self.table().get_mut(&rep)?.continuation_args = args;

        Ok(())
    }

    fn lens(
        &mut self,
        rep: wasmtime::component::Resource<TaskContext>,
        id: String,
    ) -> wasmtime::Result<wasmtime::component::Resource<TaskContext>> {
        let ctx = self.table().get_mut(&rep)?;
        let ctx_with_lens = ctx.clone();

        println!("TODO: {}", id);

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

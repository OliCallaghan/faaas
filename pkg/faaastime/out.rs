mod state {
    use wasmtime::component::bindgen;
    use wasmtime::component::Resource;
    use wasmtime::component::ResourceTable;
    use wasmtime_wasi::{WasiCtx, WasiCtxBuilder, WasiView};
    use wasmtime_wasi_http::{WasiHttpCtx, WasiHttpView};
    use self::exports::faaas::task::callable::TaskOutput;
    use self::faaas::task::types::Host;
    use self::faaas::task::types::HostTaskContext;
    use self::faaas::task::types::HostTaskError;
    use self::faaas::task::types::HostTaskOutput;
    use self::faaas::task::types::TaskContext;
    mod types {
        use super::faaas::task::types::TaskStatus;
        pub struct TaskContext {
            pub value: u32,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for TaskContext {
            #[inline]
            fn clone(&self) -> TaskContext {
                TaskContext {
                    value: ::core::clone::Clone::clone(&self.value),
                }
            }
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
    #[doc(hidden)]
    pub use self::types::TaskContext as __with_name0;
    #[doc(hidden)]
    pub use self::types::TaskOutput as __with_name1;
    pub struct Faaastime {
        interface0: exports::faaas::task::callable::Guest,
    }
    const _: () = {
        #[allow(unused_imports)]
        use wasmtime::component::__internal::anyhow;
        impl Faaastime {
            pub fn add_to_linker<T, U>(
                linker: &mut wasmtime::component::Linker<T>,
                get: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
            ) -> wasmtime::Result<()>
            where
                U: faaas::task::types::Host,
            {
                faaas::task::types::add_to_linker(linker, get)?;
                Ok(())
            }
            /// Instantiates the provided `module` using the specified
            /// parameters, wrapping up the result in a structure that
            /// translates between wasm and the host.
            pub fn instantiate<T>(
                mut store: impl wasmtime::AsContextMut<Data = T>,
                component: &wasmtime::component::Component,
                linker: &wasmtime::component::Linker<T>,
            ) -> wasmtime::Result<(Self, wasmtime::component::Instance)> {
                let instance = linker.instantiate(&mut store, component)?;
                Ok((Self::new(store, &instance)?, instance))
            }
            /// Instantiates a pre-instantiated module using the specified
            /// parameters, wrapping up the result in a structure that
            /// translates between wasm and the host.
            pub fn instantiate_pre<T>(
                mut store: impl wasmtime::AsContextMut<Data = T>,
                instance_pre: &wasmtime::component::InstancePre<T>,
            ) -> wasmtime::Result<(Self, wasmtime::component::Instance)> {
                let instance = instance_pre.instantiate(&mut store)?;
                Ok((Self::new(store, &instance)?, instance))
            }
            /// Low-level creation wrapper for wrapping up the exports
            /// of the `instance` provided in this structure of wasm
            /// exports.
            ///
            /// This function will extract exports from the `instance`
            /// defined within `store` and wrap them all up in the
            /// returned structure which can be used to interact with
            /// the wasm module.
            pub fn new(
                mut store: impl wasmtime::AsContextMut,
                instance: &wasmtime::component::Instance,
            ) -> wasmtime::Result<Self> {
                let mut store = store.as_context_mut();
                let mut exports = instance.exports(&mut store);
                let mut __exports = exports.root();
                let interface0 = exports::faaas::task::callable::Guest::new(
                    &mut __exports
                        .instance("faaas:task/callable")
                        .ok_or_else(|| ::anyhow::__private::must_use({
                            let error = ::anyhow::__private::format_err(
                                format_args!(
                                    "exported instance `faaas:task/callable` not present",
                                ),
                            );
                            error
                        }))?,
                )?;
                Ok(Faaastime { interface0 })
            }
            pub fn faaas_task_callable(&self) -> &exports::faaas::task::callable::Guest {
                &self.interface0
            }
        }
    };
    pub mod faaas {
        pub mod task {
            #[allow(clippy::all)]
            pub mod types {
                #[allow(unused_imports)]
                use wasmtime::component::__internal::anyhow;
                pub use super::super::super::__with_name0 as TaskContext;
                pub trait HostTaskContext {
                    fn set(
                        &mut self,
                        self_: wasmtime::component::Resource<TaskContext>,
                        key: String,
                        value: u32,
                    ) -> wasmtime::Result<()>;
                    fn get(
                        &mut self,
                        self_: wasmtime::component::Resource<TaskContext>,
                        key: String,
                    ) -> wasmtime::Result<u32>;
                    fn clone(
                        &mut self,
                        self_: wasmtime::component::Resource<TaskContext>,
                    ) -> wasmtime::Result<wasmtime::component::Resource<TaskContext>>;
                    fn merge(
                        &mut self,
                        fst: wasmtime::component::Resource<TaskContext>,
                        snd: wasmtime::component::Resource<TaskContext>,
                    ) -> wasmtime::Result<wasmtime::component::Resource<TaskContext>>;
                    fn drop(
                        &mut self,
                        rep: wasmtime::component::Resource<TaskContext>,
                    ) -> wasmtime::Result<()>;
                }
                #[component(enum)]
                pub enum TaskStatus {
                    #[component(name = "success")]
                    Success,
                    #[component(name = "error")]
                    Error,
                }
                #[automatically_derived]
                impl ::core::clone::Clone for TaskStatus {
                    #[inline]
                    fn clone(&self) -> TaskStatus {
                        *self
                    }
                }
                #[automatically_derived]
                impl ::core::marker::Copy for TaskStatus {}
                #[automatically_derived]
                impl ::core::marker::StructuralPartialEq for TaskStatus {}
                #[automatically_derived]
                impl ::core::cmp::PartialEq for TaskStatus {
                    #[inline]
                    fn eq(&self, other: &TaskStatus) -> bool {
                        let __self_tag = ::core::intrinsics::discriminant_value(self);
                        let __arg1_tag = ::core::intrinsics::discriminant_value(other);
                        __self_tag == __arg1_tag
                    }
                }
                #[automatically_derived]
                impl ::core::cmp::Eq for TaskStatus {
                    #[inline]
                    #[doc(hidden)]
                    #[coverage(off)]
                    fn assert_receiver_is_total_eq(&self) -> () {}
                }
                unsafe impl wasmtime::component::Lower for TaskStatus {
                    #[inline]
                    fn lower<T>(
                        &self,
                        cx: &mut wasmtime::component::__internal::LowerContext<'_, T>,
                        ty: wasmtime::component::__internal::InterfaceType,
                        dst: &mut std::mem::MaybeUninit<Self::Lower>,
                    ) -> wasmtime::component::__internal::anyhow::Result<()> {
                        let ty = match ty {
                            wasmtime::component::__internal::InterfaceType::Enum(i) => {
                                &cx.types[i]
                            }
                            _ => wasmtime::component::__internal::bad_type_info(),
                        };
                        match self {
                            Self::Success => {
                                {
                                    #[allow(unused_unsafe)]
                                    {
                                        unsafe {
                                            use ::wasmtime::component::__internal::MaybeUninitExt;
                                            let m: &mut std::mem::MaybeUninit<_> = dst;
                                            m.map(|p| &raw mut (*p).tag)
                                        }
                                    }
                                }
                                    .write(wasmtime::ValRaw::u32(0u32));
                                unsafe {
                                    wasmtime::component::__internal::lower_payload(
                                        {
                                            #[allow(unused_unsafe)]
                                            {
                                                unsafe {
                                                    use ::wasmtime::component::__internal::MaybeUninitExt;
                                                    let m: &mut std::mem::MaybeUninit<_> = dst;
                                                    m.map(|p| &raw mut (*p).payload)
                                                }
                                            }
                                        },
                                        |payload| {
                                            #[allow(unused_unsafe)]
                                            {
                                                unsafe {
                                                    use ::wasmtime::component::__internal::MaybeUninitExt;
                                                    let m: &mut std::mem::MaybeUninit<_> = payload;
                                                    m.map(|p| &raw mut (*p).Success)
                                                }
                                            }
                                        },
                                        |dst| Ok(()),
                                    )
                                }
                            }
                            Self::Error => {
                                {
                                    #[allow(unused_unsafe)]
                                    {
                                        unsafe {
                                            use ::wasmtime::component::__internal::MaybeUninitExt;
                                            let m: &mut std::mem::MaybeUninit<_> = dst;
                                            m.map(|p| &raw mut (*p).tag)
                                        }
                                    }
                                }
                                    .write(wasmtime::ValRaw::u32(1u32));
                                unsafe {
                                    wasmtime::component::__internal::lower_payload(
                                        {
                                            #[allow(unused_unsafe)]
                                            {
                                                unsafe {
                                                    use ::wasmtime::component::__internal::MaybeUninitExt;
                                                    let m: &mut std::mem::MaybeUninit<_> = dst;
                                                    m.map(|p| &raw mut (*p).payload)
                                                }
                                            }
                                        },
                                        |payload| {
                                            #[allow(unused_unsafe)]
                                            {
                                                unsafe {
                                                    use ::wasmtime::component::__internal::MaybeUninitExt;
                                                    let m: &mut std::mem::MaybeUninit<_> = payload;
                                                    m.map(|p| &raw mut (*p).Error)
                                                }
                                            }
                                        },
                                        |dst| Ok(()),
                                    )
                                }
                            }
                        }
                    }
                    #[inline]
                    fn store<T>(
                        &self,
                        cx: &mut wasmtime::component::__internal::LowerContext<'_, T>,
                        ty: wasmtime::component::__internal::InterfaceType,
                        mut offset: usize,
                    ) -> wasmtime::component::__internal::anyhow::Result<()> {
                        let ty = match ty {
                            wasmtime::component::__internal::InterfaceType::Enum(i) => {
                                &cx.types[i]
                            }
                            _ => wasmtime::component::__internal::bad_type_info(),
                        };
                        if true {
                            if !(offset
                                % (<Self as wasmtime::component::ComponentType>::ALIGN32
                                    as usize) == 0)
                            {
                                ::core::panicking::panic(
                                    "assertion failed: offset % (<Self as wasmtime::component::ComponentType>::ALIGN32 as usize) == 0",
                                )
                            }
                        }
                        match self {
                            Self::Success => {
                                *cx.get::<1usize>(offset) = 0u8.to_le_bytes();
                                Ok(())
                            }
                            Self::Error => {
                                *cx.get::<1usize>(offset) = 1u8.to_le_bytes();
                                Ok(())
                            }
                        }
                    }
                }
                unsafe impl wasmtime::component::Lift for TaskStatus {
                    #[inline]
                    fn lift(
                        cx: &mut wasmtime::component::__internal::LiftContext<'_>,
                        ty: wasmtime::component::__internal::InterfaceType,
                        src: &Self::Lower,
                    ) -> wasmtime::component::__internal::anyhow::Result<Self> {
                        let ty = match ty {
                            wasmtime::component::__internal::InterfaceType::Enum(i) => {
                                &cx.types[i]
                            }
                            _ => wasmtime::component::__internal::bad_type_info(),
                        };
                        Ok(
                            match src.tag.get_u32() {
                                0u32 => Self::Success,
                                1u32 => Self::Error,
                                discrim => {
                                    return ::anyhow::__private::Err(
                                        ::anyhow::Error::msg({
                                            let res = ::alloc::fmt::format(
                                                format_args!("unexpected discriminant: {0}", discrim),
                                            );
                                            res
                                        }),
                                    );
                                }
                            },
                        )
                    }
                    #[inline]
                    fn load(
                        cx: &mut wasmtime::component::__internal::LiftContext<'_>,
                        ty: wasmtime::component::__internal::InterfaceType,
                        bytes: &[u8],
                    ) -> wasmtime::component::__internal::anyhow::Result<Self> {
                        let align = <Self as wasmtime::component::ComponentType>::ALIGN32;
                        if true {
                            if !((bytes.as_ptr() as usize) % (align as usize) == 0) {
                                ::core::panicking::panic(
                                    "assertion failed: (bytes.as_ptr() as usize) % (align as usize) == 0",
                                )
                            }
                        }
                        let discrim = bytes[0];
                        let payload_offset = <Self as wasmtime::component::__internal::ComponentVariant>::PAYLOAD_OFFSET32;
                        let payload = &bytes[payload_offset..];
                        let ty = match ty {
                            wasmtime::component::__internal::InterfaceType::Enum(i) => {
                                &cx.types[i]
                            }
                            _ => wasmtime::component::__internal::bad_type_info(),
                        };
                        Ok(
                            match discrim {
                                0u8 => Self::Success,
                                1u8 => Self::Error,
                                discrim => {
                                    return ::anyhow::__private::Err(
                                        ::anyhow::Error::msg({
                                            let res = ::alloc::fmt::format(
                                                format_args!("unexpected discriminant: {0}", discrim),
                                            );
                                            res
                                        }),
                                    );
                                }
                            },
                        )
                    }
                }
                const _: () = {
                    #[doc(hidden)]
                    #[repr(C)]
                    pub struct LowerTaskStatus {
                        tag: wasmtime::ValRaw,
                        payload: LowerPayloadTaskStatus,
                    }
                    #[automatically_derived]
                    impl ::core::clone::Clone for LowerTaskStatus {
                        #[inline]
                        fn clone(&self) -> LowerTaskStatus {
                            let _: ::core::clone::AssertParamIsClone<wasmtime::ValRaw>;
                            let _: ::core::clone::AssertParamIsClone<
                                LowerPayloadTaskStatus,
                            >;
                            *self
                        }
                    }
                    #[automatically_derived]
                    impl ::core::marker::Copy for LowerTaskStatus {}
                    #[doc(hidden)]
                    #[allow(non_snake_case)]
                    #[repr(C)]
                    union LowerPayloadTaskStatus {
                        Success: [wasmtime::ValRaw; 0],
                        Error: [wasmtime::ValRaw; 0],
                    }
                    #[automatically_derived]
                    #[allow(non_snake_case)]
                    impl ::core::clone::Clone for LowerPayloadTaskStatus {
                        #[inline]
                        fn clone(&self) -> LowerPayloadTaskStatus {
                            let _: ::core::clone::AssertParamIsCopy<Self>;
                            *self
                        }
                    }
                    #[automatically_derived]
                    #[allow(non_snake_case)]
                    impl ::core::marker::Copy for LowerPayloadTaskStatus {}
                    unsafe impl wasmtime::component::ComponentType for TaskStatus {
                        type Lower = LowerTaskStatus;
                        #[inline]
                        fn typecheck(
                            ty: &wasmtime::component::__internal::InterfaceType,
                            types: &wasmtime::component::__internal::InstanceType<'_>,
                        ) -> wasmtime::component::__internal::anyhow::Result<()> {
                            wasmtime::component::__internal::typecheck_enum(
                                ty,
                                types,
                                &["success", "error"],
                            )
                        }
                        const ABI: wasmtime::component::__internal::CanonicalAbiInfo = wasmtime::component::__internal::CanonicalAbiInfo::variant_static(
                            &[None, None],
                        );
                    }
                    unsafe impl wasmtime::component::__internal::ComponentVariant
                    for TaskStatus {
                        const CASES: &'static [Option<
                            wasmtime::component::__internal::CanonicalAbiInfo,
                        >] = &[None, None];
                    }
                };
                impl core::fmt::Debug for TaskStatus {
                    fn fmt(
                        &self,
                        f: &mut core::fmt::Formatter<'_>,
                    ) -> core::fmt::Result {
                        match self {
                            TaskStatus::Success => {
                                f.debug_tuple("TaskStatus::Success").finish()
                            }
                            TaskStatus::Error => {
                                f.debug_tuple("TaskStatus::Error").finish()
                            }
                        }
                    }
                }
                const _: () = {
                    if !(1 == <TaskStatus as wasmtime::component::ComponentType>::SIZE32)
                    {
                        ::core::panicking::panic(
                            "assertion failed: 1 == <TaskStatus as wasmtime::component::ComponentType>::SIZE32",
                        )
                    }
                    if !(1
                        == <TaskStatus as wasmtime::component::ComponentType>::ALIGN32)
                    {
                        ::core::panicking::panic(
                            "assertion failed: 1 == <TaskStatus as wasmtime::component::ComponentType>::ALIGN32",
                        )
                    }
                };
                pub use super::super::super::__with_name1 as TaskOutput;
                pub trait HostTaskOutput {
                    fn new(
                        &mut self,
                    ) -> wasmtime::Result<wasmtime::component::Resource<TaskOutput>>;
                    fn set_status(
                        &mut self,
                        self_: wasmtime::component::Resource<TaskOutput>,
                        status: TaskStatus,
                    ) -> wasmtime::Result<()>;
                    fn get_status(
                        &mut self,
                        self_: wasmtime::component::Resource<TaskOutput>,
                    ) -> wasmtime::Result<TaskStatus>;
                    fn drop(
                        &mut self,
                        rep: wasmtime::component::Resource<TaskOutput>,
                    ) -> wasmtime::Result<()>;
                }
                pub enum TaskError {}
                pub trait HostTaskError {
                    fn drop(
                        &mut self,
                        rep: wasmtime::component::Resource<TaskError>,
                    ) -> wasmtime::Result<()>;
                }
                pub trait Host: HostTaskContext + HostTaskOutput + HostTaskError {}
                pub fn add_to_linker<T, U>(
                    linker: &mut wasmtime::component::Linker<T>,
                    get: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
                ) -> wasmtime::Result<()>
                where
                    U: Host,
                {
                    let mut inst = linker.instance("faaas:task/types")?;
                    inst.resource(
                        "task-context",
                        wasmtime::component::ResourceType::host::<TaskContext>(),
                        move |mut store, rep| -> wasmtime::Result<()> {
                            HostTaskContext::drop(
                                get(store.data_mut()),
                                wasmtime::component::Resource::new_own(rep),
                            )
                        },
                    )?;
                    inst.resource(
                        "task-output",
                        wasmtime::component::ResourceType::host::<TaskOutput>(),
                        move |mut store, rep| -> wasmtime::Result<()> {
                            HostTaskOutput::drop(
                                get(store.data_mut()),
                                wasmtime::component::Resource::new_own(rep),
                            )
                        },
                    )?;
                    inst.resource(
                        "task-error",
                        wasmtime::component::ResourceType::host::<TaskError>(),
                        move |mut store, rep| -> wasmtime::Result<()> {
                            HostTaskError::drop(
                                get(store.data_mut()),
                                wasmtime::component::Resource::new_own(rep),
                            )
                        },
                    )?;
                    inst.func_wrap(
                        "[method]task-context.set",
                        move |
                            mut caller: wasmtime::StoreContextMut<'_, T>,
                            (
                                arg0,
                                arg1,
                                arg2,
                            ): (wasmtime::component::Resource<TaskContext>, String, u32)|
                        {
                            let host = get(caller.data_mut());
                            let r = HostTaskContext::set(host, arg0, arg1, arg2);
                            r
                        },
                    )?;
                    inst.func_wrap(
                        "[method]task-context.get",
                        move |
                            mut caller: wasmtime::StoreContextMut<'_, T>,
                            (
                                arg0,
                                arg1,
                            ): (wasmtime::component::Resource<TaskContext>, String)|
                        {
                            let host = get(caller.data_mut());
                            let r = HostTaskContext::get(host, arg0, arg1);
                            Ok((r?,))
                        },
                    )?;
                    inst.func_wrap(
                        "[method]task-context.clone",
                        move |
                            mut caller: wasmtime::StoreContextMut<'_, T>,
                            (arg0,): (wasmtime::component::Resource<TaskContext>,)|
                        {
                            let host = get(caller.data_mut());
                            let r = HostTaskContext::clone(host, arg0);
                            Ok((r?,))
                        },
                    )?;
                    inst.func_wrap(
                        "[static]task-context.merge",
                        move |
                            mut caller: wasmtime::StoreContextMut<'_, T>,
                            (
                                arg0,
                                arg1,
                            ): (
                                wasmtime::component::Resource<TaskContext>,
                                wasmtime::component::Resource<TaskContext>,
                            )|
                        {
                            let host = get(caller.data_mut());
                            let r = HostTaskContext::merge(host, arg0, arg1);
                            Ok((r?,))
                        },
                    )?;
                    inst.func_wrap(
                        "[constructor]task-output",
                        move |mut caller: wasmtime::StoreContextMut<'_, T>, (): ()| {
                            let host = get(caller.data_mut());
                            let r = HostTaskOutput::new(host);
                            Ok((r?,))
                        },
                    )?;
                    inst.func_wrap(
                        "[method]task-output.set-status",
                        move |
                            mut caller: wasmtime::StoreContextMut<'_, T>,
                            (
                                arg0,
                                arg1,
                            ): (wasmtime::component::Resource<TaskOutput>, TaskStatus)|
                        {
                            let host = get(caller.data_mut());
                            let r = HostTaskOutput::set_status(host, arg0, arg1);
                            r
                        },
                    )?;
                    inst.func_wrap(
                        "[method]task-output.get-status",
                        move |
                            mut caller: wasmtime::StoreContextMut<'_, T>,
                            (arg0,): (wasmtime::component::Resource<TaskOutput>,)|
                        {
                            let host = get(caller.data_mut());
                            let r = HostTaskOutput::get_status(host, arg0);
                            Ok((r?,))
                        },
                    )?;
                    Ok(())
                }
            }
        }
    }
    pub mod exports {
        pub mod faaas {
            pub mod task {
                #[allow(clippy::all)]
                pub mod callable {
                    #[allow(unused_imports)]
                    use wasmtime::component::__internal::anyhow;
                    pub type TaskContext = super::super::super::super::faaas::task::types::TaskContext;
                    pub type TaskError = super::super::super::super::faaas::task::types::TaskError;
                    pub type TaskOutput = super::super::super::super::faaas::task::types::TaskOutput;
                    pub struct Guest {
                        call: wasmtime::component::Func,
                    }
                    impl Guest {
                        pub fn new(
                            __exports: &mut wasmtime::component::ExportInstance<'_, '_>,
                        ) -> wasmtime::Result<Guest> {
                            let call = *__exports
                                .typed_func::<
                                    (wasmtime::component::Resource<TaskContext>,),
                                    (
                                        Result<
                                            wasmtime::component::Resource<TaskOutput>,
                                            wasmtime::component::Resource<TaskError>,
                                        >,
                                    ),
                                >("call")?
                                .func();
                            Ok(Guest { call })
                        }
                        pub fn call_call<S: wasmtime::AsContextMut>(
                            &self,
                            mut store: S,
                            arg0: wasmtime::component::Resource<TaskContext>,
                        ) -> wasmtime::Result<
                            Result<
                                wasmtime::component::Resource<TaskOutput>,
                                wasmtime::component::Resource<TaskError>,
                            >,
                        > {
                            let callee = unsafe {
                                wasmtime::component::TypedFunc::<
                                    (wasmtime::component::Resource<TaskContext>,),
                                    (
                                        Result<
                                            wasmtime::component::Resource<TaskOutput>,
                                            wasmtime::component::Resource<TaskError>,
                                        >,
                                    ),
                                >::new_unchecked(self.call)
                            };
                            let (ret0,) = callee.call(store.as_context_mut(), (arg0,))?;
                            callee.post_return(store.as_context_mut())?;
                            Ok(ret0)
                        }
                    }
                }
            }
        }
    }
    const _: &str = "package faaas:task;\n\ninterface types {\n    resource task-context {\n        set: func(key: string, value: u32);\n        get: func(key: string) -> u32;\n        clone: func() -> task-context;\n        merge: static func(fst: task-context, snd: task-context) -> task-context;\n    }\n\n    enum task-status {\n        success,\n        error,\n    }\n\n    resource task-output {\n        constructor();\n        set-status: func(status: task-status);\n        get-status: func() -> task-status;\n    }\n\n    resource task-error {}\n}\n\ninterface callable {\n    use types.{task-context, task-error, task-output};\n\n    call: func(ctx: borrow<task-context>) -> result<task-output, task-error>;\n}\n\nworld task {\n    export callable;\n}\n";
    const _: &str = "package faaas:faaastime;\n\nworld faaastime {\n    include faaas:task/task;\n}\n";
    pub fn add_to_linker<T>(l: &mut wasmtime::component::Linker<T>) -> anyhow::Result<()>
    where
        T: FaaasTaskView + self::faaas::task::types::Host,
    {
        self::faaas::task::types::add_to_linker(l, |t| t)?;
        Ok(())
    }
    pub struct FaaasTaskCtx {}
    #[automatically_derived]
    impl ::core::default::Default for FaaasTaskCtx {
        #[inline]
        fn default() -> FaaasTaskCtx {
            FaaasTaskCtx {}
        }
    }
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
            let val = ctx.value;
            Ok(val)
        }
        fn set(
            &mut self,
            rep: wasmtime::component::Resource<TaskContext>,
            key: String,
            value: u32,
        ) -> wasmtime::Result<()> {
            let ctx = self.table().get_mut(&rep)?;
            ctx.value = value;
            Ok(())
        }
        fn drop(
            &mut self,
            rep: wasmtime::component::Resource<TaskContext>,
        ) -> wasmtime::Result<()> {
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
            {
                ::core::panicking::panic_fmt(
                    format_args!(
                        "not yet implemented: {0}",
                        format_args!(
                            "Need to do this, not sure whether the function signature is correct even actualy.",
                        ),
                    ),
                );
            }
        }
    }
    impl<V: FaaasTaskView> HostTaskOutput for V {
        fn new(
            &mut self,
        ) -> wasmtime::Result<
            wasmtime::component::Resource<faaas::task::types::TaskOutput>,
        > {
            let out = TaskOutput::new();
            Ok(self.table().push(out)?)
        }
        fn drop(
            &mut self,
            rep: wasmtime::component::Resource<faaas::task::types::TaskOutput>,
        ) -> wasmtime::Result<()> {
            self.table().delete(rep)?;
            Ok(())
        }
        fn get_status(
            &mut self,
            rep: wasmtime::component::Resource<faaas::task::types::TaskOutput>,
        ) -> wasmtime::Result<faaas::task::types::TaskStatus> {
            let out = self.table().get(&rep)?;
            Ok(out.status)
        }
        fn set_status(
            &mut self,
            rep: wasmtime::component::Resource<faaas::task::types::TaskOutput>,
            status: faaas::task::types::TaskStatus,
        ) -> wasmtime::Result<()> {
            let out = self.table().get_mut(&rep)?;
            out.status = status;
            Ok(())
        }
    }
    impl<V: FaaasTaskView> HostTaskError for V {
        fn drop(
            &mut self,
            rep: wasmtime::component::Resource<faaas::task::types::TaskError>,
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
}

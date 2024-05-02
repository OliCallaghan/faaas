#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use anyhow::Result;
use wasmtime::component::bindgen;
use wasmtime::{Instance, Module, Store};
pub struct Faaastime {
    interface0: exports::wasi::http::incoming_handler::Guest,
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
            U: wasi::io::poll::Host + wasi::clocks::monotonic_clock::Host
                + wasi::io::error::Host + wasi::io::streams::Host
                + wasi::http::types::Host + wasi::random::random::Host
                + wasi::cli::stdout::Host + wasi::cli::stderr::Host
                + wasi::cli::stdin::Host + wasi::http::outgoing_handler::Host
                + wasi::clocks::wall_clock::Host,
        {
            wasi::io::poll::add_to_linker(linker, get)?;
            wasi::clocks::monotonic_clock::add_to_linker(linker, get)?;
            wasi::io::error::add_to_linker(linker, get)?;
            wasi::io::streams::add_to_linker(linker, get)?;
            wasi::http::types::add_to_linker(linker, get)?;
            wasi::random::random::add_to_linker(linker, get)?;
            wasi::cli::stdout::add_to_linker(linker, get)?;
            wasi::cli::stderr::add_to_linker(linker, get)?;
            wasi::cli::stdin::add_to_linker(linker, get)?;
            wasi::http::outgoing_handler::add_to_linker(linker, get)?;
            wasi::clocks::wall_clock::add_to_linker(linker, get)?;
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
            let interface0 = exports::wasi::http::incoming_handler::Guest::new(
                &mut __exports
                    .instance("wasi:http/incoming-handler@0.2.0")
                    .ok_or_else(|| ::anyhow::__private::must_use({
                        let error = ::anyhow::__private::format_err(
                            format_args!(
                                "exported instance `wasi:http/incoming-handler@0.2.0` not present",
                            ),
                        );
                        error
                    }))?,
            )?;
            Ok(Faaastime { interface0 })
        }
        pub fn wasi_http_incoming_handler(
            &self,
        ) -> &exports::wasi::http::incoming_handler::Guest {
            &self.interface0
        }
    }
};
pub mod wasi {
    pub mod cli {
        #[allow(clippy::all)]
        pub mod stdout {
            #[allow(unused_imports)]
            use wasmtime::component::__internal::anyhow;
            pub type OutputStream = super::super::super::wasi::io::streams::OutputStream;
            pub trait Host {
                fn get_stdout(
                    &mut self,
                ) -> wasmtime::Result<wasmtime::component::Resource<OutputStream>>;
            }
            pub fn add_to_linker<T, U>(
                linker: &mut wasmtime::component::Linker<T>,
                get: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
            ) -> wasmtime::Result<()>
            where
                U: Host,
            {
                let mut inst = linker.instance("wasi:cli/stdout@0.2.0")?;
                inst.func_wrap(
                    "get-stdout",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>, (): ()| {
                        let host = get(caller.data_mut());
                        let r = Host::get_stdout(host);
                        Ok((r?,))
                    },
                )?;
                Ok(())
            }
        }
        #[allow(clippy::all)]
        pub mod stderr {
            #[allow(unused_imports)]
            use wasmtime::component::__internal::anyhow;
            pub type OutputStream = super::super::super::wasi::io::streams::OutputStream;
            pub trait Host {
                fn get_stderr(
                    &mut self,
                ) -> wasmtime::Result<wasmtime::component::Resource<OutputStream>>;
            }
            pub fn add_to_linker<T, U>(
                linker: &mut wasmtime::component::Linker<T>,
                get: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
            ) -> wasmtime::Result<()>
            where
                U: Host,
            {
                let mut inst = linker.instance("wasi:cli/stderr@0.2.0")?;
                inst.func_wrap(
                    "get-stderr",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>, (): ()| {
                        let host = get(caller.data_mut());
                        let r = Host::get_stderr(host);
                        Ok((r?,))
                    },
                )?;
                Ok(())
            }
        }
        #[allow(clippy::all)]
        pub mod stdin {
            #[allow(unused_imports)]
            use wasmtime::component::__internal::anyhow;
            pub type InputStream = super::super::super::wasi::io::streams::InputStream;
            pub trait Host {
                fn get_stdin(
                    &mut self,
                ) -> wasmtime::Result<wasmtime::component::Resource<InputStream>>;
            }
            pub fn add_to_linker<T, U>(
                linker: &mut wasmtime::component::Linker<T>,
                get: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
            ) -> wasmtime::Result<()>
            where
                U: Host,
            {
                let mut inst = linker.instance("wasi:cli/stdin@0.2.0")?;
                inst.func_wrap(
                    "get-stdin",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>, (): ()| {
                        let host = get(caller.data_mut());
                        let r = Host::get_stdin(host);
                        Ok((r?,))
                    },
                )?;
                Ok(())
            }
        }
    }
    pub mod clocks {
        #[allow(clippy::all)]
        pub mod monotonic_clock {
            #[allow(unused_imports)]
            use wasmtime::component::__internal::anyhow;
            pub type Pollable = super::super::super::wasi::io::poll::Pollable;
            /// An instant in time, in nanoseconds. An instant is relative to an
            /// unspecified initial value, and can only be compared to instances from
            /// the same monotonic-clock.
            pub type Instant = u64;
            const _: () = {
                if !(8 == <Instant as wasmtime::component::ComponentType>::SIZE32) {
                    ::core::panicking::panic(
                        "assertion failed: 8 == <Instant as wasmtime::component::ComponentType>::SIZE32",
                    )
                }
                if !(8 == <Instant as wasmtime::component::ComponentType>::ALIGN32) {
                    ::core::panicking::panic(
                        "assertion failed: 8 == <Instant as wasmtime::component::ComponentType>::ALIGN32",
                    )
                }
            };
            /// A duration of time, in nanoseconds.
            pub type Duration = u64;
            const _: () = {
                if !(8 == <Duration as wasmtime::component::ComponentType>::SIZE32) {
                    ::core::panicking::panic(
                        "assertion failed: 8 == <Duration as wasmtime::component::ComponentType>::SIZE32",
                    )
                }
                if !(8 == <Duration as wasmtime::component::ComponentType>::ALIGN32) {
                    ::core::panicking::panic(
                        "assertion failed: 8 == <Duration as wasmtime::component::ComponentType>::ALIGN32",
                    )
                }
            };
            pub trait Host {
                /// Read the current value of the clock.
                ///
                /// The clock is monotonic, therefore calling this function repeatedly will
                /// produce a sequence of non-decreasing values.
                fn now(&mut self) -> wasmtime::Result<Instant>;
                /// Query the resolution of the clock. Returns the duration of time
                /// corresponding to a clock tick.
                fn resolution(&mut self) -> wasmtime::Result<Duration>;
                /// Create a `pollable` which will resolve once the specified instant
                /// occured.
                fn subscribe_instant(
                    &mut self,
                    when: Instant,
                ) -> wasmtime::Result<wasmtime::component::Resource<Pollable>>;
                /// Create a `pollable` which will resolve once the given duration has
                /// elapsed, starting at the time at which this function was called.
                /// occured.
                fn subscribe_duration(
                    &mut self,
                    when: Duration,
                ) -> wasmtime::Result<wasmtime::component::Resource<Pollable>>;
            }
            pub fn add_to_linker<T, U>(
                linker: &mut wasmtime::component::Linker<T>,
                get: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
            ) -> wasmtime::Result<()>
            where
                U: Host,
            {
                let mut inst = linker.instance("wasi:clocks/monotonic-clock@0.2.0")?;
                inst.func_wrap(
                    "now",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>, (): ()| {
                        let host = get(caller.data_mut());
                        let r = Host::now(host);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap(
                    "resolution",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>, (): ()| {
                        let host = get(caller.data_mut());
                        let r = Host::resolution(host);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap(
                    "subscribe-instant",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (arg0,): (Instant,)|
                    {
                        let host = get(caller.data_mut());
                        let r = Host::subscribe_instant(host, arg0);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap(
                    "subscribe-duration",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (arg0,): (Duration,)|
                    {
                        let host = get(caller.data_mut());
                        let r = Host::subscribe_duration(host, arg0);
                        Ok((r?,))
                    },
                )?;
                Ok(())
            }
        }
        #[allow(clippy::all)]
        pub mod wall_clock {
            #[allow(unused_imports)]
            use wasmtime::component::__internal::anyhow;
            /// A time and date in seconds plus nanoseconds.
            #[component(record)]
            pub struct Datetime {
                #[component(name = "seconds")]
                pub seconds: u64,
                #[component(name = "nanoseconds")]
                pub nanoseconds: u32,
            }
            #[automatically_derived]
            impl ::core::marker::Copy for Datetime {}
            #[automatically_derived]
            impl ::core::clone::Clone for Datetime {
                #[inline]
                fn clone(&self) -> Datetime {
                    let _: ::core::clone::AssertParamIsClone<u64>;
                    let _: ::core::clone::AssertParamIsClone<u32>;
                    *self
                }
            }
            unsafe impl wasmtime::component::Lower for Datetime {
                #[inline]
                fn lower<T>(
                    &self,
                    cx: &mut wasmtime::component::__internal::LowerContext<'_, T>,
                    ty: wasmtime::component::__internal::InterfaceType,
                    dst: &mut std::mem::MaybeUninit<Self::Lower>,
                ) -> wasmtime::component::__internal::anyhow::Result<()> {
                    let ty = match ty {
                        wasmtime::component::__internal::InterfaceType::Record(i) => {
                            &cx.types[i]
                        }
                        _ => wasmtime::component::__internal::bad_type_info(),
                    };
                    wasmtime::component::Lower::lower(
                        &self.seconds,
                        cx,
                        ty.fields[0usize].ty,
                        {
                            #[allow(unused_unsafe)]
                            {
                                unsafe {
                                    use ::wasmtime::component::__internal::MaybeUninitExt;
                                    let m: &mut std::mem::MaybeUninit<_> = dst;
                                    m.map(|p| &raw mut (*p).seconds)
                                }
                            }
                        },
                    )?;
                    wasmtime::component::Lower::lower(
                        &self.nanoseconds,
                        cx,
                        ty.fields[1usize].ty,
                        {
                            #[allow(unused_unsafe)]
                            {
                                unsafe {
                                    use ::wasmtime::component::__internal::MaybeUninitExt;
                                    let m: &mut std::mem::MaybeUninit<_> = dst;
                                    m.map(|p| &raw mut (*p).nanoseconds)
                                }
                            }
                        },
                    )?;
                    Ok(())
                }
                #[inline]
                fn store<T>(
                    &self,
                    cx: &mut wasmtime::component::__internal::LowerContext<'_, T>,
                    ty: wasmtime::component::__internal::InterfaceType,
                    mut offset: usize,
                ) -> wasmtime::component::__internal::anyhow::Result<()> {
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
                    let ty = match ty {
                        wasmtime::component::__internal::InterfaceType::Record(i) => {
                            &cx.types[i]
                        }
                        _ => wasmtime::component::__internal::bad_type_info(),
                    };
                    wasmtime::component::Lower::store(
                        &self.seconds,
                        cx,
                        ty.fields[0usize].ty,
                        <u64 as wasmtime::component::ComponentType>::ABI
                            .next_field32_size(&mut offset),
                    )?;
                    wasmtime::component::Lower::store(
                        &self.nanoseconds,
                        cx,
                        ty.fields[1usize].ty,
                        <u32 as wasmtime::component::ComponentType>::ABI
                            .next_field32_size(&mut offset),
                    )?;
                    Ok(())
                }
            }
            unsafe impl wasmtime::component::Lift for Datetime {
                #[inline]
                fn lift(
                    cx: &mut wasmtime::component::__internal::LiftContext<'_>,
                    ty: wasmtime::component::__internal::InterfaceType,
                    src: &Self::Lower,
                ) -> wasmtime::component::__internal::anyhow::Result<Self> {
                    let ty = match ty {
                        wasmtime::component::__internal::InterfaceType::Record(i) => {
                            &cx.types[i]
                        }
                        _ => wasmtime::component::__internal::bad_type_info(),
                    };
                    Ok(Self {
                        seconds: <u64 as wasmtime::component::Lift>::lift(
                            cx,
                            ty.fields[0usize].ty,
                            &src.seconds,
                        )?,
                        nanoseconds: <u32 as wasmtime::component::Lift>::lift(
                            cx,
                            ty.fields[1usize].ty,
                            &src.nanoseconds,
                        )?,
                    })
                }
                #[inline]
                fn load(
                    cx: &mut wasmtime::component::__internal::LiftContext<'_>,
                    ty: wasmtime::component::__internal::InterfaceType,
                    bytes: &[u8],
                ) -> wasmtime::component::__internal::anyhow::Result<Self> {
                    let ty = match ty {
                        wasmtime::component::__internal::InterfaceType::Record(i) => {
                            &cx.types[i]
                        }
                        _ => wasmtime::component::__internal::bad_type_info(),
                    };
                    if true {
                        if !((bytes.as_ptr() as usize)
                            % (<Self as wasmtime::component::ComponentType>::ALIGN32
                                as usize) == 0)
                        {
                            ::core::panicking::panic(
                                "assertion failed: (bytes.as_ptr() as usize) %\n        (<Self as wasmtime::component::ComponentType>::ALIGN32 as usize) == 0",
                            )
                        }
                    }
                    let mut offset = 0;
                    Ok(Self {
                        seconds: <u64 as wasmtime::component::Lift>::load(
                            cx,
                            ty.fields[0usize].ty,
                            &bytes[<u64 as wasmtime::component::ComponentType>::ABI
                                .next_field32_size(
                                    &mut offset,
                                )..][..<u64 as wasmtime::component::ComponentType>::SIZE32],
                        )?,
                        nanoseconds: <u32 as wasmtime::component::Lift>::load(
                            cx,
                            ty.fields[1usize].ty,
                            &bytes[<u32 as wasmtime::component::ComponentType>::ABI
                                .next_field32_size(
                                    &mut offset,
                                )..][..<u32 as wasmtime::component::ComponentType>::SIZE32],
                        )?,
                    })
                }
            }
            const _: () = {
                #[doc(hidden)]
                #[repr(C)]
                pub struct LowerDatetime<T0: Copy, T1: Copy> {
                    seconds: T0,
                    nanoseconds: T1,
                    _align: [wasmtime::ValRaw; 0],
                }
                #[automatically_derived]
                impl<
                    T0: ::core::clone::Clone + Copy,
                    T1: ::core::clone::Clone + Copy,
                > ::core::clone::Clone for LowerDatetime<T0, T1> {
                    #[inline]
                    fn clone(&self) -> LowerDatetime<T0, T1> {
                        LowerDatetime {
                            seconds: ::core::clone::Clone::clone(&self.seconds),
                            nanoseconds: ::core::clone::Clone::clone(&self.nanoseconds),
                            _align: ::core::clone::Clone::clone(&self._align),
                        }
                    }
                }
                #[automatically_derived]
                impl<
                    T0: ::core::marker::Copy + Copy,
                    T1: ::core::marker::Copy + Copy,
                > ::core::marker::Copy for LowerDatetime<T0, T1> {}
                unsafe impl wasmtime::component::ComponentType for Datetime {
                    type Lower = LowerDatetime<
                        <u64 as wasmtime::component::ComponentType>::Lower,
                        <u32 as wasmtime::component::ComponentType>::Lower,
                    >;
                    const ABI: wasmtime::component::__internal::CanonicalAbiInfo = wasmtime::component::__internal::CanonicalAbiInfo::record_static(
                        &[
                            <u64 as wasmtime::component::ComponentType>::ABI,
                            <u32 as wasmtime::component::ComponentType>::ABI,
                        ],
                    );
                    #[inline]
                    fn typecheck(
                        ty: &wasmtime::component::__internal::InterfaceType,
                        types: &wasmtime::component::__internal::InstanceType<'_>,
                    ) -> wasmtime::component::__internal::anyhow::Result<()> {
                        wasmtime::component::__internal::typecheck_record(
                            ty,
                            types,
                            &[
                                (
                                    "seconds",
                                    <u64 as wasmtime::component::ComponentType>::typecheck,
                                ),
                                (
                                    "nanoseconds",
                                    <u32 as wasmtime::component::ComponentType>::typecheck,
                                ),
                            ],
                        )
                    }
                }
            };
            impl core::fmt::Debug for Datetime {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    f.debug_struct("Datetime")
                        .field("seconds", &self.seconds)
                        .field("nanoseconds", &self.nanoseconds)
                        .finish()
                }
            }
            const _: () = {
                if !(16 == <Datetime as wasmtime::component::ComponentType>::SIZE32) {
                    ::core::panicking::panic(
                        "assertion failed: 16 == <Datetime as wasmtime::component::ComponentType>::SIZE32",
                    )
                }
                if !(8 == <Datetime as wasmtime::component::ComponentType>::ALIGN32) {
                    ::core::panicking::panic(
                        "assertion failed: 8 == <Datetime as wasmtime::component::ComponentType>::ALIGN32",
                    )
                }
            };
            pub trait Host {
                /// Read the current value of the clock.
                ///
                /// This clock is not monotonic, therefore calling this function repeatedly
                /// will not necessarily produce a sequence of non-decreasing values.
                ///
                /// The returned timestamps represent the number of seconds since
                /// 1970-01-01T00:00:00Z, also known as [POSIX's Seconds Since the Epoch],
                /// also known as [Unix Time].
                ///
                /// The nanoseconds field of the output is always less than 1000000000.
                ///
                /// [POSIX's Seconds Since the Epoch]: https://pubs.opengroup.org/onlinepubs/9699919799/xrat/V4_xbd_chap04.html#tag_21_04_16
                /// [Unix Time]: https://en.wikipedia.org/wiki/Unix_time
                fn now(&mut self) -> wasmtime::Result<Datetime>;
                /// Query the resolution of the clock.
                ///
                /// The nanoseconds field of the output is always less than 1000000000.
                fn resolution(&mut self) -> wasmtime::Result<Datetime>;
            }
            pub fn add_to_linker<T, U>(
                linker: &mut wasmtime::component::Linker<T>,
                get: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
            ) -> wasmtime::Result<()>
            where
                U: Host,
            {
                let mut inst = linker.instance("wasi:clocks/wall-clock@0.2.0")?;
                inst.func_wrap(
                    "now",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>, (): ()| {
                        let host = get(caller.data_mut());
                        let r = Host::now(host);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap(
                    "resolution",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>, (): ()| {
                        let host = get(caller.data_mut());
                        let r = Host::resolution(host);
                        Ok((r?,))
                    },
                )?;
                Ok(())
            }
        }
    }
    pub mod http {
        #[allow(clippy::all)]
        pub mod types {
            #[allow(unused_imports)]
            use wasmtime::component::__internal::anyhow;
            pub type Duration = super::super::super::wasi::clocks::monotonic_clock::Duration;
            const _: () = {
                if !(8 == <Duration as wasmtime::component::ComponentType>::SIZE32) {
                    ::core::panicking::panic(
                        "assertion failed: 8 == <Duration as wasmtime::component::ComponentType>::SIZE32",
                    )
                }
                if !(8 == <Duration as wasmtime::component::ComponentType>::ALIGN32) {
                    ::core::panicking::panic(
                        "assertion failed: 8 == <Duration as wasmtime::component::ComponentType>::ALIGN32",
                    )
                }
            };
            pub type InputStream = super::super::super::wasi::io::streams::InputStream;
            pub type OutputStream = super::super::super::wasi::io::streams::OutputStream;
            pub type IoError = super::super::super::wasi::io::error::Error;
            pub type Pollable = super::super::super::wasi::io::poll::Pollable;
            /// This type corresponds to HTTP standard Methods.
            #[component(variant)]
            pub enum Method {
                #[component(name = "get")]
                Get,
                #[component(name = "head")]
                Head,
                #[component(name = "post")]
                Post,
                #[component(name = "put")]
                Put,
                #[component(name = "delete")]
                Delete,
                #[component(name = "connect")]
                Connect,
                #[component(name = "options")]
                Options,
                #[component(name = "trace")]
                Trace,
                #[component(name = "patch")]
                Patch,
                #[component(name = "other")]
                Other(String),
            }
            #[automatically_derived]
            impl ::core::clone::Clone for Method {
                #[inline]
                fn clone(&self) -> Method {
                    match self {
                        Method::Get => Method::Get,
                        Method::Head => Method::Head,
                        Method::Post => Method::Post,
                        Method::Put => Method::Put,
                        Method::Delete => Method::Delete,
                        Method::Connect => Method::Connect,
                        Method::Options => Method::Options,
                        Method::Trace => Method::Trace,
                        Method::Patch => Method::Patch,
                        Method::Other(__self_0) => {
                            Method::Other(::core::clone::Clone::clone(__self_0))
                        }
                    }
                }
            }
            unsafe impl wasmtime::component::Lower for Method {
                #[inline]
                fn lower<T>(
                    &self,
                    cx: &mut wasmtime::component::__internal::LowerContext<'_, T>,
                    ty: wasmtime::component::__internal::InterfaceType,
                    dst: &mut std::mem::MaybeUninit<Self::Lower>,
                ) -> wasmtime::component::__internal::anyhow::Result<()> {
                    let ty = match ty {
                        wasmtime::component::__internal::InterfaceType::Variant(i) => {
                            &cx.types[i]
                        }
                        _ => wasmtime::component::__internal::bad_type_info(),
                    };
                    match self {
                        Self::Get => {
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
                                                m.map(|p| &raw mut (*p).Get)
                                            }
                                        }
                                    },
                                    |dst| Ok(()),
                                )
                            }
                        }
                        Self::Head => {
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
                                                m.map(|p| &raw mut (*p).Head)
                                            }
                                        }
                                    },
                                    |dst| Ok(()),
                                )
                            }
                        }
                        Self::Post => {
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
                                .write(wasmtime::ValRaw::u32(2u32));
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
                                                m.map(|p| &raw mut (*p).Post)
                                            }
                                        }
                                    },
                                    |dst| Ok(()),
                                )
                            }
                        }
                        Self::Put => {
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
                                .write(wasmtime::ValRaw::u32(3u32));
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
                                                m.map(|p| &raw mut (*p).Put)
                                            }
                                        }
                                    },
                                    |dst| Ok(()),
                                )
                            }
                        }
                        Self::Delete => {
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
                                .write(wasmtime::ValRaw::u32(4u32));
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
                                                m.map(|p| &raw mut (*p).Delete)
                                            }
                                        }
                                    },
                                    |dst| Ok(()),
                                )
                            }
                        }
                        Self::Connect => {
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
                                .write(wasmtime::ValRaw::u32(5u32));
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
                                                m.map(|p| &raw mut (*p).Connect)
                                            }
                                        }
                                    },
                                    |dst| Ok(()),
                                )
                            }
                        }
                        Self::Options => {
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
                                .write(wasmtime::ValRaw::u32(6u32));
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
                                                m.map(|p| &raw mut (*p).Options)
                                            }
                                        }
                                    },
                                    |dst| Ok(()),
                                )
                            }
                        }
                        Self::Trace => {
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
                                .write(wasmtime::ValRaw::u32(7u32));
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
                                                m.map(|p| &raw mut (*p).Trace)
                                            }
                                        }
                                    },
                                    |dst| Ok(()),
                                )
                            }
                        }
                        Self::Patch => {
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
                                .write(wasmtime::ValRaw::u32(8u32));
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
                                                m.map(|p| &raw mut (*p).Patch)
                                            }
                                        }
                                    },
                                    |dst| Ok(()),
                                )
                            }
                        }
                        Self::Other(value) => {
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
                                .write(wasmtime::ValRaw::u32(9u32));
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
                                                m.map(|p| &raw mut (*p).Other)
                                            }
                                        }
                                    },
                                    |dst| {
                                        value
                                            .lower(
                                                cx,
                                                ty
                                                    .cases[9usize]
                                                    .ty
                                                    .unwrap_or_else(
                                                        wasmtime::component::__internal::bad_type_info,
                                                    ),
                                                dst,
                                            )
                                    },
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
                        wasmtime::component::__internal::InterfaceType::Variant(i) => {
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
                        Self::Get => {
                            *cx.get::<1usize>(offset) = 0u8.to_le_bytes();
                            Ok(())
                        }
                        Self::Head => {
                            *cx.get::<1usize>(offset) = 1u8.to_le_bytes();
                            Ok(())
                        }
                        Self::Post => {
                            *cx.get::<1usize>(offset) = 2u8.to_le_bytes();
                            Ok(())
                        }
                        Self::Put => {
                            *cx.get::<1usize>(offset) = 3u8.to_le_bytes();
                            Ok(())
                        }
                        Self::Delete => {
                            *cx.get::<1usize>(offset) = 4u8.to_le_bytes();
                            Ok(())
                        }
                        Self::Connect => {
                            *cx.get::<1usize>(offset) = 5u8.to_le_bytes();
                            Ok(())
                        }
                        Self::Options => {
                            *cx.get::<1usize>(offset) = 6u8.to_le_bytes();
                            Ok(())
                        }
                        Self::Trace => {
                            *cx.get::<1usize>(offset) = 7u8.to_le_bytes();
                            Ok(())
                        }
                        Self::Patch => {
                            *cx.get::<1usize>(offset) = 8u8.to_le_bytes();
                            Ok(())
                        }
                        Self::Other(value) => {
                            *cx.get::<1usize>(offset) = 9u8.to_le_bytes();
                            value
                                .store(
                                    cx,
                                    ty
                                        .cases[9usize]
                                        .ty
                                        .unwrap_or_else(
                                            wasmtime::component::__internal::bad_type_info,
                                        ),
                                    offset
                                        + <Self as wasmtime::component::__internal::ComponentVariant>::PAYLOAD_OFFSET32,
                                )
                        }
                    }
                }
            }
            unsafe impl wasmtime::component::Lift for Method {
                #[inline]
                fn lift(
                    cx: &mut wasmtime::component::__internal::LiftContext<'_>,
                    ty: wasmtime::component::__internal::InterfaceType,
                    src: &Self::Lower,
                ) -> wasmtime::component::__internal::anyhow::Result<Self> {
                    let ty = match ty {
                        wasmtime::component::__internal::InterfaceType::Variant(i) => {
                            &cx.types[i]
                        }
                        _ => wasmtime::component::__internal::bad_type_info(),
                    };
                    Ok(
                        match src.tag.get_u32() {
                            0u32 => Self::Get,
                            1u32 => Self::Head,
                            2u32 => Self::Post,
                            3u32 => Self::Put,
                            4u32 => Self::Delete,
                            5u32 => Self::Connect,
                            6u32 => Self::Options,
                            7u32 => Self::Trace,
                            8u32 => Self::Patch,
                            9u32 => {
                                Self::Other(
                                    <String as wasmtime::component::Lift>::lift(
                                        cx,
                                        ty
                                            .cases[9usize]
                                            .ty
                                            .unwrap_or_else(
                                                wasmtime::component::__internal::bad_type_info,
                                            ),
                                        unsafe { &src.payload.Other },
                                    )?,
                                )
                            }
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
                        wasmtime::component::__internal::InterfaceType::Variant(i) => {
                            &cx.types[i]
                        }
                        _ => wasmtime::component::__internal::bad_type_info(),
                    };
                    Ok(
                        match discrim {
                            0u8 => Self::Get,
                            1u8 => Self::Head,
                            2u8 => Self::Post,
                            3u8 => Self::Put,
                            4u8 => Self::Delete,
                            5u8 => Self::Connect,
                            6u8 => Self::Options,
                            7u8 => Self::Trace,
                            8u8 => Self::Patch,
                            9u8 => {
                                Self::Other(
                                    <String as wasmtime::component::Lift>::load(
                                        cx,
                                        ty
                                            .cases[9usize]
                                            .ty
                                            .unwrap_or_else(
                                                wasmtime::component::__internal::bad_type_info,
                                            ),
                                        &payload[..<String as wasmtime::component::ComponentType>::SIZE32],
                                    )?,
                                )
                            }
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
                pub struct LowerMethod<T9: Copy> {
                    tag: wasmtime::ValRaw,
                    payload: LowerPayloadMethod<T9>,
                }
                #[automatically_derived]
                impl<T9: ::core::clone::Clone + Copy> ::core::clone::Clone
                for LowerMethod<T9> {
                    #[inline]
                    fn clone(&self) -> LowerMethod<T9> {
                        LowerMethod {
                            tag: ::core::clone::Clone::clone(&self.tag),
                            payload: ::core::clone::Clone::clone(&self.payload),
                        }
                    }
                }
                #[automatically_derived]
                impl<T9: ::core::marker::Copy + Copy> ::core::marker::Copy
                for LowerMethod<T9> {}
                #[doc(hidden)]
                #[allow(non_snake_case)]
                #[repr(C)]
                union LowerPayloadMethod<T9: Copy> {
                    Get: [wasmtime::ValRaw; 0],
                    Head: [wasmtime::ValRaw; 0],
                    Post: [wasmtime::ValRaw; 0],
                    Put: [wasmtime::ValRaw; 0],
                    Delete: [wasmtime::ValRaw; 0],
                    Connect: [wasmtime::ValRaw; 0],
                    Options: [wasmtime::ValRaw; 0],
                    Trace: [wasmtime::ValRaw; 0],
                    Patch: [wasmtime::ValRaw; 0],
                    Other: T9,
                }
                #[automatically_derived]
                #[allow(non_snake_case)]
                impl<
                    T9: ::core::marker::Copy + ::core::clone::Clone + Copy,
                > ::core::clone::Clone for LowerPayloadMethod<T9> {
                    #[inline]
                    fn clone(&self) -> LowerPayloadMethod<T9> {
                        let _: ::core::clone::AssertParamIsCopy<Self>;
                        *self
                    }
                }
                #[automatically_derived]
                #[allow(non_snake_case)]
                impl<T9: ::core::marker::Copy + Copy> ::core::marker::Copy
                for LowerPayloadMethod<T9> {}
                unsafe impl wasmtime::component::ComponentType for Method {
                    type Lower = LowerMethod<
                        <String as wasmtime::component::ComponentType>::Lower,
                    >;
                    #[inline]
                    fn typecheck(
                        ty: &wasmtime::component::__internal::InterfaceType,
                        types: &wasmtime::component::__internal::InstanceType<'_>,
                    ) -> wasmtime::component::__internal::anyhow::Result<()> {
                        wasmtime::component::__internal::typecheck_variant(
                            ty,
                            types,
                            &[
                                ("get", None),
                                ("head", None),
                                ("post", None),
                                ("put", None),
                                ("delete", None),
                                ("connect", None),
                                ("options", None),
                                ("trace", None),
                                ("patch", None),
                                (
                                    "other",
                                    Some(
                                        <String as wasmtime::component::ComponentType>::typecheck,
                                    ),
                                ),
                            ],
                        )
                    }
                    const ABI: wasmtime::component::__internal::CanonicalAbiInfo = wasmtime::component::__internal::CanonicalAbiInfo::variant_static(
                        &[
                            None,
                            None,
                            None,
                            None,
                            None,
                            None,
                            None,
                            None,
                            None,
                            Some(<String as wasmtime::component::ComponentType>::ABI),
                        ],
                    );
                }
                unsafe impl wasmtime::component::__internal::ComponentVariant
                for Method {
                    const CASES: &'static [Option<
                        wasmtime::component::__internal::CanonicalAbiInfo,
                    >] = &[
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                        Some(<String as wasmtime::component::ComponentType>::ABI),
                    ];
                }
            };
            impl core::fmt::Debug for Method {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    match self {
                        Method::Get => f.debug_tuple("Method::Get").finish(),
                        Method::Head => f.debug_tuple("Method::Head").finish(),
                        Method::Post => f.debug_tuple("Method::Post").finish(),
                        Method::Put => f.debug_tuple("Method::Put").finish(),
                        Method::Delete => f.debug_tuple("Method::Delete").finish(),
                        Method::Connect => f.debug_tuple("Method::Connect").finish(),
                        Method::Options => f.debug_tuple("Method::Options").finish(),
                        Method::Trace => f.debug_tuple("Method::Trace").finish(),
                        Method::Patch => f.debug_tuple("Method::Patch").finish(),
                        Method::Other(e) => {
                            f.debug_tuple("Method::Other").field(e).finish()
                        }
                    }
                }
            }
            const _: () = {
                if !(12 == <Method as wasmtime::component::ComponentType>::SIZE32) {
                    ::core::panicking::panic(
                        "assertion failed: 12 == <Method as wasmtime::component::ComponentType>::SIZE32",
                    )
                }
                if !(4 == <Method as wasmtime::component::ComponentType>::ALIGN32) {
                    ::core::panicking::panic(
                        "assertion failed: 4 == <Method as wasmtime::component::ComponentType>::ALIGN32",
                    )
                }
            };
            /// This type corresponds to HTTP standard Related Schemes.
            #[component(variant)]
            pub enum Scheme {
                #[component(name = "HTTP")]
                Http,
                #[component(name = "HTTPS")]
                Https,
                #[component(name = "other")]
                Other(String),
            }
            #[automatically_derived]
            impl ::core::clone::Clone for Scheme {
                #[inline]
                fn clone(&self) -> Scheme {
                    match self {
                        Scheme::Http => Scheme::Http,
                        Scheme::Https => Scheme::Https,
                        Scheme::Other(__self_0) => {
                            Scheme::Other(::core::clone::Clone::clone(__self_0))
                        }
                    }
                }
            }
            unsafe impl wasmtime::component::Lower for Scheme {
                #[inline]
                fn lower<T>(
                    &self,
                    cx: &mut wasmtime::component::__internal::LowerContext<'_, T>,
                    ty: wasmtime::component::__internal::InterfaceType,
                    dst: &mut std::mem::MaybeUninit<Self::Lower>,
                ) -> wasmtime::component::__internal::anyhow::Result<()> {
                    let ty = match ty {
                        wasmtime::component::__internal::InterfaceType::Variant(i) => {
                            &cx.types[i]
                        }
                        _ => wasmtime::component::__internal::bad_type_info(),
                    };
                    match self {
                        Self::Http => {
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
                                                m.map(|p| &raw mut (*p).Http)
                                            }
                                        }
                                    },
                                    |dst| Ok(()),
                                )
                            }
                        }
                        Self::Https => {
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
                                                m.map(|p| &raw mut (*p).Https)
                                            }
                                        }
                                    },
                                    |dst| Ok(()),
                                )
                            }
                        }
                        Self::Other(value) => {
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
                                .write(wasmtime::ValRaw::u32(2u32));
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
                                                m.map(|p| &raw mut (*p).Other)
                                            }
                                        }
                                    },
                                    |dst| {
                                        value
                                            .lower(
                                                cx,
                                                ty
                                                    .cases[2usize]
                                                    .ty
                                                    .unwrap_or_else(
                                                        wasmtime::component::__internal::bad_type_info,
                                                    ),
                                                dst,
                                            )
                                    },
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
                        wasmtime::component::__internal::InterfaceType::Variant(i) => {
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
                        Self::Http => {
                            *cx.get::<1usize>(offset) = 0u8.to_le_bytes();
                            Ok(())
                        }
                        Self::Https => {
                            *cx.get::<1usize>(offset) = 1u8.to_le_bytes();
                            Ok(())
                        }
                        Self::Other(value) => {
                            *cx.get::<1usize>(offset) = 2u8.to_le_bytes();
                            value
                                .store(
                                    cx,
                                    ty
                                        .cases[2usize]
                                        .ty
                                        .unwrap_or_else(
                                            wasmtime::component::__internal::bad_type_info,
                                        ),
                                    offset
                                        + <Self as wasmtime::component::__internal::ComponentVariant>::PAYLOAD_OFFSET32,
                                )
                        }
                    }
                }
            }
            unsafe impl wasmtime::component::Lift for Scheme {
                #[inline]
                fn lift(
                    cx: &mut wasmtime::component::__internal::LiftContext<'_>,
                    ty: wasmtime::component::__internal::InterfaceType,
                    src: &Self::Lower,
                ) -> wasmtime::component::__internal::anyhow::Result<Self> {
                    let ty = match ty {
                        wasmtime::component::__internal::InterfaceType::Variant(i) => {
                            &cx.types[i]
                        }
                        _ => wasmtime::component::__internal::bad_type_info(),
                    };
                    Ok(
                        match src.tag.get_u32() {
                            0u32 => Self::Http,
                            1u32 => Self::Https,
                            2u32 => {
                                Self::Other(
                                    <String as wasmtime::component::Lift>::lift(
                                        cx,
                                        ty
                                            .cases[2usize]
                                            .ty
                                            .unwrap_or_else(
                                                wasmtime::component::__internal::bad_type_info,
                                            ),
                                        unsafe { &src.payload.Other },
                                    )?,
                                )
                            }
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
                        wasmtime::component::__internal::InterfaceType::Variant(i) => {
                            &cx.types[i]
                        }
                        _ => wasmtime::component::__internal::bad_type_info(),
                    };
                    Ok(
                        match discrim {
                            0u8 => Self::Http,
                            1u8 => Self::Https,
                            2u8 => {
                                Self::Other(
                                    <String as wasmtime::component::Lift>::load(
                                        cx,
                                        ty
                                            .cases[2usize]
                                            .ty
                                            .unwrap_or_else(
                                                wasmtime::component::__internal::bad_type_info,
                                            ),
                                        &payload[..<String as wasmtime::component::ComponentType>::SIZE32],
                                    )?,
                                )
                            }
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
                pub struct LowerScheme<T2: Copy> {
                    tag: wasmtime::ValRaw,
                    payload: LowerPayloadScheme<T2>,
                }
                #[automatically_derived]
                impl<T2: ::core::clone::Clone + Copy> ::core::clone::Clone
                for LowerScheme<T2> {
                    #[inline]
                    fn clone(&self) -> LowerScheme<T2> {
                        LowerScheme {
                            tag: ::core::clone::Clone::clone(&self.tag),
                            payload: ::core::clone::Clone::clone(&self.payload),
                        }
                    }
                }
                #[automatically_derived]
                impl<T2: ::core::marker::Copy + Copy> ::core::marker::Copy
                for LowerScheme<T2> {}
                #[doc(hidden)]
                #[allow(non_snake_case)]
                #[repr(C)]
                union LowerPayloadScheme<T2: Copy> {
                    Http: [wasmtime::ValRaw; 0],
                    Https: [wasmtime::ValRaw; 0],
                    Other: T2,
                }
                #[automatically_derived]
                #[allow(non_snake_case)]
                impl<
                    T2: ::core::marker::Copy + ::core::clone::Clone + Copy,
                > ::core::clone::Clone for LowerPayloadScheme<T2> {
                    #[inline]
                    fn clone(&self) -> LowerPayloadScheme<T2> {
                        let _: ::core::clone::AssertParamIsCopy<Self>;
                        *self
                    }
                }
                #[automatically_derived]
                #[allow(non_snake_case)]
                impl<T2: ::core::marker::Copy + Copy> ::core::marker::Copy
                for LowerPayloadScheme<T2> {}
                unsafe impl wasmtime::component::ComponentType for Scheme {
                    type Lower = LowerScheme<
                        <String as wasmtime::component::ComponentType>::Lower,
                    >;
                    #[inline]
                    fn typecheck(
                        ty: &wasmtime::component::__internal::InterfaceType,
                        types: &wasmtime::component::__internal::InstanceType<'_>,
                    ) -> wasmtime::component::__internal::anyhow::Result<()> {
                        wasmtime::component::__internal::typecheck_variant(
                            ty,
                            types,
                            &[
                                ("HTTP", None),
                                ("HTTPS", None),
                                (
                                    "other",
                                    Some(
                                        <String as wasmtime::component::ComponentType>::typecheck,
                                    ),
                                ),
                            ],
                        )
                    }
                    const ABI: wasmtime::component::__internal::CanonicalAbiInfo = wasmtime::component::__internal::CanonicalAbiInfo::variant_static(
                        &[
                            None,
                            None,
                            Some(<String as wasmtime::component::ComponentType>::ABI),
                        ],
                    );
                }
                unsafe impl wasmtime::component::__internal::ComponentVariant
                for Scheme {
                    const CASES: &'static [Option<
                        wasmtime::component::__internal::CanonicalAbiInfo,
                    >] = &[
                        None,
                        None,
                        Some(<String as wasmtime::component::ComponentType>::ABI),
                    ];
                }
            };
            impl core::fmt::Debug for Scheme {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    match self {
                        Scheme::Http => f.debug_tuple("Scheme::Http").finish(),
                        Scheme::Https => f.debug_tuple("Scheme::Https").finish(),
                        Scheme::Other(e) => {
                            f.debug_tuple("Scheme::Other").field(e).finish()
                        }
                    }
                }
            }
            const _: () = {
                if !(12 == <Scheme as wasmtime::component::ComponentType>::SIZE32) {
                    ::core::panicking::panic(
                        "assertion failed: 12 == <Scheme as wasmtime::component::ComponentType>::SIZE32",
                    )
                }
                if !(4 == <Scheme as wasmtime::component::ComponentType>::ALIGN32) {
                    ::core::panicking::panic(
                        "assertion failed: 4 == <Scheme as wasmtime::component::ComponentType>::ALIGN32",
                    )
                }
            };
            /// Defines the case payload type for `DNS-error` above:
            #[component(record)]
            pub struct DnsErrorPayload {
                #[component(name = "rcode")]
                pub rcode: Option<String>,
                #[component(name = "info-code")]
                pub info_code: Option<u16>,
            }
            #[automatically_derived]
            impl ::core::clone::Clone for DnsErrorPayload {
                #[inline]
                fn clone(&self) -> DnsErrorPayload {
                    DnsErrorPayload {
                        rcode: ::core::clone::Clone::clone(&self.rcode),
                        info_code: ::core::clone::Clone::clone(&self.info_code),
                    }
                }
            }
            unsafe impl wasmtime::component::Lower for DnsErrorPayload {
                #[inline]
                fn lower<T>(
                    &self,
                    cx: &mut wasmtime::component::__internal::LowerContext<'_, T>,
                    ty: wasmtime::component::__internal::InterfaceType,
                    dst: &mut std::mem::MaybeUninit<Self::Lower>,
                ) -> wasmtime::component::__internal::anyhow::Result<()> {
                    let ty = match ty {
                        wasmtime::component::__internal::InterfaceType::Record(i) => {
                            &cx.types[i]
                        }
                        _ => wasmtime::component::__internal::bad_type_info(),
                    };
                    wasmtime::component::Lower::lower(
                        &self.rcode,
                        cx,
                        ty.fields[0usize].ty,
                        {
                            #[allow(unused_unsafe)]
                            {
                                unsafe {
                                    use ::wasmtime::component::__internal::MaybeUninitExt;
                                    let m: &mut std::mem::MaybeUninit<_> = dst;
                                    m.map(|p| &raw mut (*p).rcode)
                                }
                            }
                        },
                    )?;
                    wasmtime::component::Lower::lower(
                        &self.info_code,
                        cx,
                        ty.fields[1usize].ty,
                        {
                            #[allow(unused_unsafe)]
                            {
                                unsafe {
                                    use ::wasmtime::component::__internal::MaybeUninitExt;
                                    let m: &mut std::mem::MaybeUninit<_> = dst;
                                    m.map(|p| &raw mut (*p).info_code)
                                }
                            }
                        },
                    )?;
                    Ok(())
                }
                #[inline]
                fn store<T>(
                    &self,
                    cx: &mut wasmtime::component::__internal::LowerContext<'_, T>,
                    ty: wasmtime::component::__internal::InterfaceType,
                    mut offset: usize,
                ) -> wasmtime::component::__internal::anyhow::Result<()> {
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
                    let ty = match ty {
                        wasmtime::component::__internal::InterfaceType::Record(i) => {
                            &cx.types[i]
                        }
                        _ => wasmtime::component::__internal::bad_type_info(),
                    };
                    wasmtime::component::Lower::store(
                        &self.rcode,
                        cx,
                        ty.fields[0usize].ty,
                        <Option<String> as wasmtime::component::ComponentType>::ABI
                            .next_field32_size(&mut offset),
                    )?;
                    wasmtime::component::Lower::store(
                        &self.info_code,
                        cx,
                        ty.fields[1usize].ty,
                        <Option<u16> as wasmtime::component::ComponentType>::ABI
                            .next_field32_size(&mut offset),
                    )?;
                    Ok(())
                }
            }
            unsafe impl wasmtime::component::Lift for DnsErrorPayload {
                #[inline]
                fn lift(
                    cx: &mut wasmtime::component::__internal::LiftContext<'_>,
                    ty: wasmtime::component::__internal::InterfaceType,
                    src: &Self::Lower,
                ) -> wasmtime::component::__internal::anyhow::Result<Self> {
                    let ty = match ty {
                        wasmtime::component::__internal::InterfaceType::Record(i) => {
                            &cx.types[i]
                        }
                        _ => wasmtime::component::__internal::bad_type_info(),
                    };
                    Ok(Self {
                        rcode: <Option<
                            String,
                        > as wasmtime::component::Lift>::lift(
                            cx,
                            ty.fields[0usize].ty,
                            &src.rcode,
                        )?,
                        info_code: <Option<
                            u16,
                        > as wasmtime::component::Lift>::lift(
                            cx,
                            ty.fields[1usize].ty,
                            &src.info_code,
                        )?,
                    })
                }
                #[inline]
                fn load(
                    cx: &mut wasmtime::component::__internal::LiftContext<'_>,
                    ty: wasmtime::component::__internal::InterfaceType,
                    bytes: &[u8],
                ) -> wasmtime::component::__internal::anyhow::Result<Self> {
                    let ty = match ty {
                        wasmtime::component::__internal::InterfaceType::Record(i) => {
                            &cx.types[i]
                        }
                        _ => wasmtime::component::__internal::bad_type_info(),
                    };
                    if true {
                        if !((bytes.as_ptr() as usize)
                            % (<Self as wasmtime::component::ComponentType>::ALIGN32
                                as usize) == 0)
                        {
                            ::core::panicking::panic(
                                "assertion failed: (bytes.as_ptr() as usize) %\n        (<Self as wasmtime::component::ComponentType>::ALIGN32 as usize) == 0",
                            )
                        }
                    }
                    let mut offset = 0;
                    Ok(Self {
                        rcode: <Option<
                            String,
                        > as wasmtime::component::Lift>::load(
                            cx,
                            ty.fields[0usize].ty,
                            &bytes[<Option<
                                String,
                            > as wasmtime::component::ComponentType>::ABI
                                .next_field32_size(
                                    &mut offset,
                                )..][..<Option<
                                String,
                            > as wasmtime::component::ComponentType>::SIZE32],
                        )?,
                        info_code: <Option<
                            u16,
                        > as wasmtime::component::Lift>::load(
                            cx,
                            ty.fields[1usize].ty,
                            &bytes[<Option<
                                u16,
                            > as wasmtime::component::ComponentType>::ABI
                                .next_field32_size(
                                    &mut offset,
                                )..][..<Option<
                                u16,
                            > as wasmtime::component::ComponentType>::SIZE32],
                        )?,
                    })
                }
            }
            const _: () = {
                #[doc(hidden)]
                #[repr(C)]
                pub struct LowerDnsErrorPayload<T0: Copy, T1: Copy> {
                    rcode: T0,
                    info_code: T1,
                    _align: [wasmtime::ValRaw; 0],
                }
                #[automatically_derived]
                impl<
                    T0: ::core::clone::Clone + Copy,
                    T1: ::core::clone::Clone + Copy,
                > ::core::clone::Clone for LowerDnsErrorPayload<T0, T1> {
                    #[inline]
                    fn clone(&self) -> LowerDnsErrorPayload<T0, T1> {
                        LowerDnsErrorPayload {
                            rcode: ::core::clone::Clone::clone(&self.rcode),
                            info_code: ::core::clone::Clone::clone(&self.info_code),
                            _align: ::core::clone::Clone::clone(&self._align),
                        }
                    }
                }
                #[automatically_derived]
                impl<
                    T0: ::core::marker::Copy + Copy,
                    T1: ::core::marker::Copy + Copy,
                > ::core::marker::Copy for LowerDnsErrorPayload<T0, T1> {}
                unsafe impl wasmtime::component::ComponentType for DnsErrorPayload {
                    type Lower = LowerDnsErrorPayload<
                        <Option<String> as wasmtime::component::ComponentType>::Lower,
                        <Option<u16> as wasmtime::component::ComponentType>::Lower,
                    >;
                    const ABI: wasmtime::component::__internal::CanonicalAbiInfo = wasmtime::component::__internal::CanonicalAbiInfo::record_static(
                        &[
                            <Option<String> as wasmtime::component::ComponentType>::ABI,
                            <Option<u16> as wasmtime::component::ComponentType>::ABI,
                        ],
                    );
                    #[inline]
                    fn typecheck(
                        ty: &wasmtime::component::__internal::InterfaceType,
                        types: &wasmtime::component::__internal::InstanceType<'_>,
                    ) -> wasmtime::component::__internal::anyhow::Result<()> {
                        wasmtime::component::__internal::typecheck_record(
                            ty,
                            types,
                            &[
                                (
                                    "rcode",
                                    <Option<
                                        String,
                                    > as wasmtime::component::ComponentType>::typecheck,
                                ),
                                (
                                    "info-code",
                                    <Option<
                                        u16,
                                    > as wasmtime::component::ComponentType>::typecheck,
                                ),
                            ],
                        )
                    }
                }
            };
            impl core::fmt::Debug for DnsErrorPayload {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    f.debug_struct("DnsErrorPayload")
                        .field("rcode", &self.rcode)
                        .field("info-code", &self.info_code)
                        .finish()
                }
            }
            const _: () = {
                if !(16
                    == <DnsErrorPayload as wasmtime::component::ComponentType>::SIZE32)
                {
                    ::core::panicking::panic(
                        "assertion failed: 16 == <DnsErrorPayload as wasmtime::component::ComponentType>::SIZE32",
                    )
                }
                if !(4
                    == <DnsErrorPayload as wasmtime::component::ComponentType>::ALIGN32)
                {
                    ::core::panicking::panic(
                        "assertion failed: 4 == <DnsErrorPayload as wasmtime::component::ComponentType>::ALIGN32",
                    )
                }
            };
            /// Defines the case payload type for `TLS-alert-received` above:
            #[component(record)]
            pub struct TlsAlertReceivedPayload {
                #[component(name = "alert-id")]
                pub alert_id: Option<u8>,
                #[component(name = "alert-message")]
                pub alert_message: Option<String>,
            }
            #[automatically_derived]
            impl ::core::clone::Clone for TlsAlertReceivedPayload {
                #[inline]
                fn clone(&self) -> TlsAlertReceivedPayload {
                    TlsAlertReceivedPayload {
                        alert_id: ::core::clone::Clone::clone(&self.alert_id),
                        alert_message: ::core::clone::Clone::clone(&self.alert_message),
                    }
                }
            }
            unsafe impl wasmtime::component::Lower for TlsAlertReceivedPayload {
                #[inline]
                fn lower<T>(
                    &self,
                    cx: &mut wasmtime::component::__internal::LowerContext<'_, T>,
                    ty: wasmtime::component::__internal::InterfaceType,
                    dst: &mut std::mem::MaybeUninit<Self::Lower>,
                ) -> wasmtime::component::__internal::anyhow::Result<()> {
                    let ty = match ty {
                        wasmtime::component::__internal::InterfaceType::Record(i) => {
                            &cx.types[i]
                        }
                        _ => wasmtime::component::__internal::bad_type_info(),
                    };
                    wasmtime::component::Lower::lower(
                        &self.alert_id,
                        cx,
                        ty.fields[0usize].ty,
                        {
                            #[allow(unused_unsafe)]
                            {
                                unsafe {
                                    use ::wasmtime::component::__internal::MaybeUninitExt;
                                    let m: &mut std::mem::MaybeUninit<_> = dst;
                                    m.map(|p| &raw mut (*p).alert_id)
                                }
                            }
                        },
                    )?;
                    wasmtime::component::Lower::lower(
                        &self.alert_message,
                        cx,
                        ty.fields[1usize].ty,
                        {
                            #[allow(unused_unsafe)]
                            {
                                unsafe {
                                    use ::wasmtime::component::__internal::MaybeUninitExt;
                                    let m: &mut std::mem::MaybeUninit<_> = dst;
                                    m.map(|p| &raw mut (*p).alert_message)
                                }
                            }
                        },
                    )?;
                    Ok(())
                }
                #[inline]
                fn store<T>(
                    &self,
                    cx: &mut wasmtime::component::__internal::LowerContext<'_, T>,
                    ty: wasmtime::component::__internal::InterfaceType,
                    mut offset: usize,
                ) -> wasmtime::component::__internal::anyhow::Result<()> {
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
                    let ty = match ty {
                        wasmtime::component::__internal::InterfaceType::Record(i) => {
                            &cx.types[i]
                        }
                        _ => wasmtime::component::__internal::bad_type_info(),
                    };
                    wasmtime::component::Lower::store(
                        &self.alert_id,
                        cx,
                        ty.fields[0usize].ty,
                        <Option<u8> as wasmtime::component::ComponentType>::ABI
                            .next_field32_size(&mut offset),
                    )?;
                    wasmtime::component::Lower::store(
                        &self.alert_message,
                        cx,
                        ty.fields[1usize].ty,
                        <Option<String> as wasmtime::component::ComponentType>::ABI
                            .next_field32_size(&mut offset),
                    )?;
                    Ok(())
                }
            }
            unsafe impl wasmtime::component::Lift for TlsAlertReceivedPayload {
                #[inline]
                fn lift(
                    cx: &mut wasmtime::component::__internal::LiftContext<'_>,
                    ty: wasmtime::component::__internal::InterfaceType,
                    src: &Self::Lower,
                ) -> wasmtime::component::__internal::anyhow::Result<Self> {
                    let ty = match ty {
                        wasmtime::component::__internal::InterfaceType::Record(i) => {
                            &cx.types[i]
                        }
                        _ => wasmtime::component::__internal::bad_type_info(),
                    };
                    Ok(Self {
                        alert_id: <Option<
                            u8,
                        > as wasmtime::component::Lift>::lift(
                            cx,
                            ty.fields[0usize].ty,
                            &src.alert_id,
                        )?,
                        alert_message: <Option<
                            String,
                        > as wasmtime::component::Lift>::lift(
                            cx,
                            ty.fields[1usize].ty,
                            &src.alert_message,
                        )?,
                    })
                }
                #[inline]
                fn load(
                    cx: &mut wasmtime::component::__internal::LiftContext<'_>,
                    ty: wasmtime::component::__internal::InterfaceType,
                    bytes: &[u8],
                ) -> wasmtime::component::__internal::anyhow::Result<Self> {
                    let ty = match ty {
                        wasmtime::component::__internal::InterfaceType::Record(i) => {
                            &cx.types[i]
                        }
                        _ => wasmtime::component::__internal::bad_type_info(),
                    };
                    if true {
                        if !((bytes.as_ptr() as usize)
                            % (<Self as wasmtime::component::ComponentType>::ALIGN32
                                as usize) == 0)
                        {
                            ::core::panicking::panic(
                                "assertion failed: (bytes.as_ptr() as usize) %\n        (<Self as wasmtime::component::ComponentType>::ALIGN32 as usize) == 0",
                            )
                        }
                    }
                    let mut offset = 0;
                    Ok(Self {
                        alert_id: <Option<
                            u8,
                        > as wasmtime::component::Lift>::load(
                            cx,
                            ty.fields[0usize].ty,
                            &bytes[<Option<
                                u8,
                            > as wasmtime::component::ComponentType>::ABI
                                .next_field32_size(
                                    &mut offset,
                                )..][..<Option<
                                u8,
                            > as wasmtime::component::ComponentType>::SIZE32],
                        )?,
                        alert_message: <Option<
                            String,
                        > as wasmtime::component::Lift>::load(
                            cx,
                            ty.fields[1usize].ty,
                            &bytes[<Option<
                                String,
                            > as wasmtime::component::ComponentType>::ABI
                                .next_field32_size(
                                    &mut offset,
                                )..][..<Option<
                                String,
                            > as wasmtime::component::ComponentType>::SIZE32],
                        )?,
                    })
                }
            }
            const _: () = {
                #[doc(hidden)]
                #[repr(C)]
                pub struct LowerTlsAlertReceivedPayload<T0: Copy, T1: Copy> {
                    alert_id: T0,
                    alert_message: T1,
                    _align: [wasmtime::ValRaw; 0],
                }
                #[automatically_derived]
                impl<
                    T0: ::core::clone::Clone + Copy,
                    T1: ::core::clone::Clone + Copy,
                > ::core::clone::Clone for LowerTlsAlertReceivedPayload<T0, T1> {
                    #[inline]
                    fn clone(&self) -> LowerTlsAlertReceivedPayload<T0, T1> {
                        LowerTlsAlertReceivedPayload {
                            alert_id: ::core::clone::Clone::clone(&self.alert_id),
                            alert_message: ::core::clone::Clone::clone(
                                &self.alert_message,
                            ),
                            _align: ::core::clone::Clone::clone(&self._align),
                        }
                    }
                }
                #[automatically_derived]
                impl<
                    T0: ::core::marker::Copy + Copy,
                    T1: ::core::marker::Copy + Copy,
                > ::core::marker::Copy for LowerTlsAlertReceivedPayload<T0, T1> {}
                unsafe impl wasmtime::component::ComponentType
                for TlsAlertReceivedPayload {
                    type Lower = LowerTlsAlertReceivedPayload<
                        <Option<u8> as wasmtime::component::ComponentType>::Lower,
                        <Option<String> as wasmtime::component::ComponentType>::Lower,
                    >;
                    const ABI: wasmtime::component::__internal::CanonicalAbiInfo = wasmtime::component::__internal::CanonicalAbiInfo::record_static(
                        &[
                            <Option<u8> as wasmtime::component::ComponentType>::ABI,
                            <Option<String> as wasmtime::component::ComponentType>::ABI,
                        ],
                    );
                    #[inline]
                    fn typecheck(
                        ty: &wasmtime::component::__internal::InterfaceType,
                        types: &wasmtime::component::__internal::InstanceType<'_>,
                    ) -> wasmtime::component::__internal::anyhow::Result<()> {
                        wasmtime::component::__internal::typecheck_record(
                            ty,
                            types,
                            &[
                                (
                                    "alert-id",
                                    <Option<
                                        u8,
                                    > as wasmtime::component::ComponentType>::typecheck,
                                ),
                                (
                                    "alert-message",
                                    <Option<
                                        String,
                                    > as wasmtime::component::ComponentType>::typecheck,
                                ),
                            ],
                        )
                    }
                }
            };
            impl core::fmt::Debug for TlsAlertReceivedPayload {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    f.debug_struct("TlsAlertReceivedPayload")
                        .field("alert-id", &self.alert_id)
                        .field("alert-message", &self.alert_message)
                        .finish()
                }
            }
            const _: () = {
                if !(16
                    == <TlsAlertReceivedPayload as wasmtime::component::ComponentType>::SIZE32)
                {
                    ::core::panicking::panic(
                        "assertion failed: 16 == <TlsAlertReceivedPayload as wasmtime::component::ComponentType>::SIZE32",
                    )
                }
                if !(4
                    == <TlsAlertReceivedPayload as wasmtime::component::ComponentType>::ALIGN32)
                {
                    ::core::panicking::panic(
                        "assertion failed: 4 == <TlsAlertReceivedPayload as wasmtime::component::ComponentType>::ALIGN32",
                    )
                }
            };
            /// Defines the case payload type for `HTTP-response-{header,trailer}-size` above:
            #[component(record)]
            pub struct FieldSizePayload {
                #[component(name = "field-name")]
                pub field_name: Option<String>,
                #[component(name = "field-size")]
                pub field_size: Option<u32>,
            }
            #[automatically_derived]
            impl ::core::clone::Clone for FieldSizePayload {
                #[inline]
                fn clone(&self) -> FieldSizePayload {
                    FieldSizePayload {
                        field_name: ::core::clone::Clone::clone(&self.field_name),
                        field_size: ::core::clone::Clone::clone(&self.field_size),
                    }
                }
            }
            unsafe impl wasmtime::component::Lower for FieldSizePayload {
                #[inline]
                fn lower<T>(
                    &self,
                    cx: &mut wasmtime::component::__internal::LowerContext<'_, T>,
                    ty: wasmtime::component::__internal::InterfaceType,
                    dst: &mut std::mem::MaybeUninit<Self::Lower>,
                ) -> wasmtime::component::__internal::anyhow::Result<()> {
                    let ty = match ty {
                        wasmtime::component::__internal::InterfaceType::Record(i) => {
                            &cx.types[i]
                        }
                        _ => wasmtime::component::__internal::bad_type_info(),
                    };
                    wasmtime::component::Lower::lower(
                        &self.field_name,
                        cx,
                        ty.fields[0usize].ty,
                        {
                            #[allow(unused_unsafe)]
                            {
                                unsafe {
                                    use ::wasmtime::component::__internal::MaybeUninitExt;
                                    let m: &mut std::mem::MaybeUninit<_> = dst;
                                    m.map(|p| &raw mut (*p).field_name)
                                }
                            }
                        },
                    )?;
                    wasmtime::component::Lower::lower(
                        &self.field_size,
                        cx,
                        ty.fields[1usize].ty,
                        {
                            #[allow(unused_unsafe)]
                            {
                                unsafe {
                                    use ::wasmtime::component::__internal::MaybeUninitExt;
                                    let m: &mut std::mem::MaybeUninit<_> = dst;
                                    m.map(|p| &raw mut (*p).field_size)
                                }
                            }
                        },
                    )?;
                    Ok(())
                }
                #[inline]
                fn store<T>(
                    &self,
                    cx: &mut wasmtime::component::__internal::LowerContext<'_, T>,
                    ty: wasmtime::component::__internal::InterfaceType,
                    mut offset: usize,
                ) -> wasmtime::component::__internal::anyhow::Result<()> {
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
                    let ty = match ty {
                        wasmtime::component::__internal::InterfaceType::Record(i) => {
                            &cx.types[i]
                        }
                        _ => wasmtime::component::__internal::bad_type_info(),
                    };
                    wasmtime::component::Lower::store(
                        &self.field_name,
                        cx,
                        ty.fields[0usize].ty,
                        <Option<String> as wasmtime::component::ComponentType>::ABI
                            .next_field32_size(&mut offset),
                    )?;
                    wasmtime::component::Lower::store(
                        &self.field_size,
                        cx,
                        ty.fields[1usize].ty,
                        <Option<u32> as wasmtime::component::ComponentType>::ABI
                            .next_field32_size(&mut offset),
                    )?;
                    Ok(())
                }
            }
            unsafe impl wasmtime::component::Lift for FieldSizePayload {
                #[inline]
                fn lift(
                    cx: &mut wasmtime::component::__internal::LiftContext<'_>,
                    ty: wasmtime::component::__internal::InterfaceType,
                    src: &Self::Lower,
                ) -> wasmtime::component::__internal::anyhow::Result<Self> {
                    let ty = match ty {
                        wasmtime::component::__internal::InterfaceType::Record(i) => {
                            &cx.types[i]
                        }
                        _ => wasmtime::component::__internal::bad_type_info(),
                    };
                    Ok(Self {
                        field_name: <Option<
                            String,
                        > as wasmtime::component::Lift>::lift(
                            cx,
                            ty.fields[0usize].ty,
                            &src.field_name,
                        )?,
                        field_size: <Option<
                            u32,
                        > as wasmtime::component::Lift>::lift(
                            cx,
                            ty.fields[1usize].ty,
                            &src.field_size,
                        )?,
                    })
                }
                #[inline]
                fn load(
                    cx: &mut wasmtime::component::__internal::LiftContext<'_>,
                    ty: wasmtime::component::__internal::InterfaceType,
                    bytes: &[u8],
                ) -> wasmtime::component::__internal::anyhow::Result<Self> {
                    let ty = match ty {
                        wasmtime::component::__internal::InterfaceType::Record(i) => {
                            &cx.types[i]
                        }
                        _ => wasmtime::component::__internal::bad_type_info(),
                    };
                    if true {
                        if !((bytes.as_ptr() as usize)
                            % (<Self as wasmtime::component::ComponentType>::ALIGN32
                                as usize) == 0)
                        {
                            ::core::panicking::panic(
                                "assertion failed: (bytes.as_ptr() as usize) %\n        (<Self as wasmtime::component::ComponentType>::ALIGN32 as usize) == 0",
                            )
                        }
                    }
                    let mut offset = 0;
                    Ok(Self {
                        field_name: <Option<
                            String,
                        > as wasmtime::component::Lift>::load(
                            cx,
                            ty.fields[0usize].ty,
                            &bytes[<Option<
                                String,
                            > as wasmtime::component::ComponentType>::ABI
                                .next_field32_size(
                                    &mut offset,
                                )..][..<Option<
                                String,
                            > as wasmtime::component::ComponentType>::SIZE32],
                        )?,
                        field_size: <Option<
                            u32,
                        > as wasmtime::component::Lift>::load(
                            cx,
                            ty.fields[1usize].ty,
                            &bytes[<Option<
                                u32,
                            > as wasmtime::component::ComponentType>::ABI
                                .next_field32_size(
                                    &mut offset,
                                )..][..<Option<
                                u32,
                            > as wasmtime::component::ComponentType>::SIZE32],
                        )?,
                    })
                }
            }
            const _: () = {
                #[doc(hidden)]
                #[repr(C)]
                pub struct LowerFieldSizePayload<T0: Copy, T1: Copy> {
                    field_name: T0,
                    field_size: T1,
                    _align: [wasmtime::ValRaw; 0],
                }
                #[automatically_derived]
                impl<
                    T0: ::core::clone::Clone + Copy,
                    T1: ::core::clone::Clone + Copy,
                > ::core::clone::Clone for LowerFieldSizePayload<T0, T1> {
                    #[inline]
                    fn clone(&self) -> LowerFieldSizePayload<T0, T1> {
                        LowerFieldSizePayload {
                            field_name: ::core::clone::Clone::clone(&self.field_name),
                            field_size: ::core::clone::Clone::clone(&self.field_size),
                            _align: ::core::clone::Clone::clone(&self._align),
                        }
                    }
                }
                #[automatically_derived]
                impl<
                    T0: ::core::marker::Copy + Copy,
                    T1: ::core::marker::Copy + Copy,
                > ::core::marker::Copy for LowerFieldSizePayload<T0, T1> {}
                unsafe impl wasmtime::component::ComponentType for FieldSizePayload {
                    type Lower = LowerFieldSizePayload<
                        <Option<String> as wasmtime::component::ComponentType>::Lower,
                        <Option<u32> as wasmtime::component::ComponentType>::Lower,
                    >;
                    const ABI: wasmtime::component::__internal::CanonicalAbiInfo = wasmtime::component::__internal::CanonicalAbiInfo::record_static(
                        &[
                            <Option<String> as wasmtime::component::ComponentType>::ABI,
                            <Option<u32> as wasmtime::component::ComponentType>::ABI,
                        ],
                    );
                    #[inline]
                    fn typecheck(
                        ty: &wasmtime::component::__internal::InterfaceType,
                        types: &wasmtime::component::__internal::InstanceType<'_>,
                    ) -> wasmtime::component::__internal::anyhow::Result<()> {
                        wasmtime::component::__internal::typecheck_record(
                            ty,
                            types,
                            &[
                                (
                                    "field-name",
                                    <Option<
                                        String,
                                    > as wasmtime::component::ComponentType>::typecheck,
                                ),
                                (
                                    "field-size",
                                    <Option<
                                        u32,
                                    > as wasmtime::component::ComponentType>::typecheck,
                                ),
                            ],
                        )
                    }
                }
            };
            impl core::fmt::Debug for FieldSizePayload {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    f.debug_struct("FieldSizePayload")
                        .field("field-name", &self.field_name)
                        .field("field-size", &self.field_size)
                        .finish()
                }
            }
            const _: () = {
                if !(20
                    == <FieldSizePayload as wasmtime::component::ComponentType>::SIZE32)
                {
                    ::core::panicking::panic(
                        "assertion failed: 20 == <FieldSizePayload as wasmtime::component::ComponentType>::SIZE32",
                    )
                }
                if !(4
                    == <FieldSizePayload as wasmtime::component::ComponentType>::ALIGN32)
                {
                    ::core::panicking::panic(
                        "assertion failed: 4 == <FieldSizePayload as wasmtime::component::ComponentType>::ALIGN32",
                    )
                }
            };
            /// These cases are inspired by the IANA HTTP Proxy Error Types:
            /// https://www.iana.org/assignments/http-proxy-status/http-proxy-status.xhtml#table-http-proxy-error-types
            #[component(variant)]
            pub enum ErrorCode {
                #[component(name = "DNS-timeout")]
                DnsTimeout,
                #[component(name = "DNS-error")]
                DnsError(DnsErrorPayload),
                #[component(name = "destination-not-found")]
                DestinationNotFound,
                #[component(name = "destination-unavailable")]
                DestinationUnavailable,
                #[component(name = "destination-IP-prohibited")]
                DestinationIpProhibited,
                #[component(name = "destination-IP-unroutable")]
                DestinationIpUnroutable,
                #[component(name = "connection-refused")]
                ConnectionRefused,
                #[component(name = "connection-terminated")]
                ConnectionTerminated,
                #[component(name = "connection-timeout")]
                ConnectionTimeout,
                #[component(name = "connection-read-timeout")]
                ConnectionReadTimeout,
                #[component(name = "connection-write-timeout")]
                ConnectionWriteTimeout,
                #[component(name = "connection-limit-reached")]
                ConnectionLimitReached,
                #[component(name = "TLS-protocol-error")]
                TlsProtocolError,
                #[component(name = "TLS-certificate-error")]
                TlsCertificateError,
                #[component(name = "TLS-alert-received")]
                TlsAlertReceived(TlsAlertReceivedPayload),
                #[component(name = "HTTP-request-denied")]
                HttpRequestDenied,
                #[component(name = "HTTP-request-length-required")]
                HttpRequestLengthRequired,
                #[component(name = "HTTP-request-body-size")]
                HttpRequestBodySize(Option<u64>),
                #[component(name = "HTTP-request-method-invalid")]
                HttpRequestMethodInvalid,
                #[component(name = "HTTP-request-URI-invalid")]
                HttpRequestUriInvalid,
                #[component(name = "HTTP-request-URI-too-long")]
                HttpRequestUriTooLong,
                #[component(name = "HTTP-request-header-section-size")]
                HttpRequestHeaderSectionSize(Option<u32>),
                #[component(name = "HTTP-request-header-size")]
                HttpRequestHeaderSize(Option<FieldSizePayload>),
                #[component(name = "HTTP-request-trailer-section-size")]
                HttpRequestTrailerSectionSize(Option<u32>),
                #[component(name = "HTTP-request-trailer-size")]
                HttpRequestTrailerSize(FieldSizePayload),
                #[component(name = "HTTP-response-incomplete")]
                HttpResponseIncomplete,
                #[component(name = "HTTP-response-header-section-size")]
                HttpResponseHeaderSectionSize(Option<u32>),
                #[component(name = "HTTP-response-header-size")]
                HttpResponseHeaderSize(FieldSizePayload),
                #[component(name = "HTTP-response-body-size")]
                HttpResponseBodySize(Option<u64>),
                #[component(name = "HTTP-response-trailer-section-size")]
                HttpResponseTrailerSectionSize(Option<u32>),
                #[component(name = "HTTP-response-trailer-size")]
                HttpResponseTrailerSize(FieldSizePayload),
                #[component(name = "HTTP-response-transfer-coding")]
                HttpResponseTransferCoding(Option<String>),
                #[component(name = "HTTP-response-content-coding")]
                HttpResponseContentCoding(Option<String>),
                #[component(name = "HTTP-response-timeout")]
                HttpResponseTimeout,
                #[component(name = "HTTP-upgrade-failed")]
                HttpUpgradeFailed,
                #[component(name = "HTTP-protocol-error")]
                HttpProtocolError,
                #[component(name = "loop-detected")]
                LoopDetected,
                #[component(name = "configuration-error")]
                ConfigurationError,
                /// This is a catch-all error for anything that doesn't fit cleanly into a
                /// more specific case. It also includes an optional string for an
                /// unstructured description of the error. Users should not depend on the
                /// string for diagnosing errors, as it's not required to be consistent
                /// between implementations.
                #[component(name = "internal-error")]
                InternalError(Option<String>),
            }
            #[automatically_derived]
            impl ::core::clone::Clone for ErrorCode {
                #[inline]
                fn clone(&self) -> ErrorCode {
                    match self {
                        ErrorCode::DnsTimeout => ErrorCode::DnsTimeout,
                        ErrorCode::DnsError(__self_0) => {
                            ErrorCode::DnsError(::core::clone::Clone::clone(__self_0))
                        }
                        ErrorCode::DestinationNotFound => ErrorCode::DestinationNotFound,
                        ErrorCode::DestinationUnavailable => {
                            ErrorCode::DestinationUnavailable
                        }
                        ErrorCode::DestinationIpProhibited => {
                            ErrorCode::DestinationIpProhibited
                        }
                        ErrorCode::DestinationIpUnroutable => {
                            ErrorCode::DestinationIpUnroutable
                        }
                        ErrorCode::ConnectionRefused => ErrorCode::ConnectionRefused,
                        ErrorCode::ConnectionTerminated => {
                            ErrorCode::ConnectionTerminated
                        }
                        ErrorCode::ConnectionTimeout => ErrorCode::ConnectionTimeout,
                        ErrorCode::ConnectionReadTimeout => {
                            ErrorCode::ConnectionReadTimeout
                        }
                        ErrorCode::ConnectionWriteTimeout => {
                            ErrorCode::ConnectionWriteTimeout
                        }
                        ErrorCode::ConnectionLimitReached => {
                            ErrorCode::ConnectionLimitReached
                        }
                        ErrorCode::TlsProtocolError => ErrorCode::TlsProtocolError,
                        ErrorCode::TlsCertificateError => ErrorCode::TlsCertificateError,
                        ErrorCode::TlsAlertReceived(__self_0) => {
                            ErrorCode::TlsAlertReceived(
                                ::core::clone::Clone::clone(__self_0),
                            )
                        }
                        ErrorCode::HttpRequestDenied => ErrorCode::HttpRequestDenied,
                        ErrorCode::HttpRequestLengthRequired => {
                            ErrorCode::HttpRequestLengthRequired
                        }
                        ErrorCode::HttpRequestBodySize(__self_0) => {
                            ErrorCode::HttpRequestBodySize(
                                ::core::clone::Clone::clone(__self_0),
                            )
                        }
                        ErrorCode::HttpRequestMethodInvalid => {
                            ErrorCode::HttpRequestMethodInvalid
                        }
                        ErrorCode::HttpRequestUriInvalid => {
                            ErrorCode::HttpRequestUriInvalid
                        }
                        ErrorCode::HttpRequestUriTooLong => {
                            ErrorCode::HttpRequestUriTooLong
                        }
                        ErrorCode::HttpRequestHeaderSectionSize(__self_0) => {
                            ErrorCode::HttpRequestHeaderSectionSize(
                                ::core::clone::Clone::clone(__self_0),
                            )
                        }
                        ErrorCode::HttpRequestHeaderSize(__self_0) => {
                            ErrorCode::HttpRequestHeaderSize(
                                ::core::clone::Clone::clone(__self_0),
                            )
                        }
                        ErrorCode::HttpRequestTrailerSectionSize(__self_0) => {
                            ErrorCode::HttpRequestTrailerSectionSize(
                                ::core::clone::Clone::clone(__self_0),
                            )
                        }
                        ErrorCode::HttpRequestTrailerSize(__self_0) => {
                            ErrorCode::HttpRequestTrailerSize(
                                ::core::clone::Clone::clone(__self_0),
                            )
                        }
                        ErrorCode::HttpResponseIncomplete => {
                            ErrorCode::HttpResponseIncomplete
                        }
                        ErrorCode::HttpResponseHeaderSectionSize(__self_0) => {
                            ErrorCode::HttpResponseHeaderSectionSize(
                                ::core::clone::Clone::clone(__self_0),
                            )
                        }
                        ErrorCode::HttpResponseHeaderSize(__self_0) => {
                            ErrorCode::HttpResponseHeaderSize(
                                ::core::clone::Clone::clone(__self_0),
                            )
                        }
                        ErrorCode::HttpResponseBodySize(__self_0) => {
                            ErrorCode::HttpResponseBodySize(
                                ::core::clone::Clone::clone(__self_0),
                            )
                        }
                        ErrorCode::HttpResponseTrailerSectionSize(__self_0) => {
                            ErrorCode::HttpResponseTrailerSectionSize(
                                ::core::clone::Clone::clone(__self_0),
                            )
                        }
                        ErrorCode::HttpResponseTrailerSize(__self_0) => {
                            ErrorCode::HttpResponseTrailerSize(
                                ::core::clone::Clone::clone(__self_0),
                            )
                        }
                        ErrorCode::HttpResponseTransferCoding(__self_0) => {
                            ErrorCode::HttpResponseTransferCoding(
                                ::core::clone::Clone::clone(__self_0),
                            )
                        }
                        ErrorCode::HttpResponseContentCoding(__self_0) => {
                            ErrorCode::HttpResponseContentCoding(
                                ::core::clone::Clone::clone(__self_0),
                            )
                        }
                        ErrorCode::HttpResponseTimeout => ErrorCode::HttpResponseTimeout,
                        ErrorCode::HttpUpgradeFailed => ErrorCode::HttpUpgradeFailed,
                        ErrorCode::HttpProtocolError => ErrorCode::HttpProtocolError,
                        ErrorCode::LoopDetected => ErrorCode::LoopDetected,
                        ErrorCode::ConfigurationError => ErrorCode::ConfigurationError,
                        ErrorCode::InternalError(__self_0) => {
                            ErrorCode::InternalError(
                                ::core::clone::Clone::clone(__self_0),
                            )
                        }
                    }
                }
            }
            unsafe impl wasmtime::component::Lower for ErrorCode {
                #[inline]
                fn lower<T>(
                    &self,
                    cx: &mut wasmtime::component::__internal::LowerContext<'_, T>,
                    ty: wasmtime::component::__internal::InterfaceType,
                    dst: &mut std::mem::MaybeUninit<Self::Lower>,
                ) -> wasmtime::component::__internal::anyhow::Result<()> {
                    let ty = match ty {
                        wasmtime::component::__internal::InterfaceType::Variant(i) => {
                            &cx.types[i]
                        }
                        _ => wasmtime::component::__internal::bad_type_info(),
                    };
                    match self {
                        Self::DnsTimeout => {
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
                                                m.map(|p| &raw mut (*p).DnsTimeout)
                                            }
                                        }
                                    },
                                    |dst| Ok(()),
                                )
                            }
                        }
                        Self::DnsError(value) => {
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
                                                m.map(|p| &raw mut (*p).DnsError)
                                            }
                                        }
                                    },
                                    |dst| {
                                        value
                                            .lower(
                                                cx,
                                                ty
                                                    .cases[1usize]
                                                    .ty
                                                    .unwrap_or_else(
                                                        wasmtime::component::__internal::bad_type_info,
                                                    ),
                                                dst,
                                            )
                                    },
                                )
                            }
                        }
                        Self::DestinationNotFound => {
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
                                .write(wasmtime::ValRaw::u32(2u32));
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
                                                m.map(|p| &raw mut (*p).DestinationNotFound)
                                            }
                                        }
                                    },
                                    |dst| Ok(()),
                                )
                            }
                        }
                        Self::DestinationUnavailable => {
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
                                .write(wasmtime::ValRaw::u32(3u32));
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
                                                m.map(|p| &raw mut (*p).DestinationUnavailable)
                                            }
                                        }
                                    },
                                    |dst| Ok(()),
                                )
                            }
                        }
                        Self::DestinationIpProhibited => {
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
                                .write(wasmtime::ValRaw::u32(4u32));
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
                                                m.map(|p| &raw mut (*p).DestinationIpProhibited)
                                            }
                                        }
                                    },
                                    |dst| Ok(()),
                                )
                            }
                        }
                        Self::DestinationIpUnroutable => {
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
                                .write(wasmtime::ValRaw::u32(5u32));
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
                                                m.map(|p| &raw mut (*p).DestinationIpUnroutable)
                                            }
                                        }
                                    },
                                    |dst| Ok(()),
                                )
                            }
                        }
                        Self::ConnectionRefused => {
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
                                .write(wasmtime::ValRaw::u32(6u32));
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
                                                m.map(|p| &raw mut (*p).ConnectionRefused)
                                            }
                                        }
                                    },
                                    |dst| Ok(()),
                                )
                            }
                        }
                        Self::ConnectionTerminated => {
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
                                .write(wasmtime::ValRaw::u32(7u32));
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
                                                m.map(|p| &raw mut (*p).ConnectionTerminated)
                                            }
                                        }
                                    },
                                    |dst| Ok(()),
                                )
                            }
                        }
                        Self::ConnectionTimeout => {
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
                                .write(wasmtime::ValRaw::u32(8u32));
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
                                                m.map(|p| &raw mut (*p).ConnectionTimeout)
                                            }
                                        }
                                    },
                                    |dst| Ok(()),
                                )
                            }
                        }
                        Self::ConnectionReadTimeout => {
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
                                .write(wasmtime::ValRaw::u32(9u32));
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
                                                m.map(|p| &raw mut (*p).ConnectionReadTimeout)
                                            }
                                        }
                                    },
                                    |dst| Ok(()),
                                )
                            }
                        }
                        Self::ConnectionWriteTimeout => {
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
                                .write(wasmtime::ValRaw::u32(10u32));
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
                                                m.map(|p| &raw mut (*p).ConnectionWriteTimeout)
                                            }
                                        }
                                    },
                                    |dst| Ok(()),
                                )
                            }
                        }
                        Self::ConnectionLimitReached => {
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
                                .write(wasmtime::ValRaw::u32(11u32));
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
                                                m.map(|p| &raw mut (*p).ConnectionLimitReached)
                                            }
                                        }
                                    },
                                    |dst| Ok(()),
                                )
                            }
                        }
                        Self::TlsProtocolError => {
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
                                .write(wasmtime::ValRaw::u32(12u32));
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
                                                m.map(|p| &raw mut (*p).TlsProtocolError)
                                            }
                                        }
                                    },
                                    |dst| Ok(()),
                                )
                            }
                        }
                        Self::TlsCertificateError => {
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
                                .write(wasmtime::ValRaw::u32(13u32));
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
                                                m.map(|p| &raw mut (*p).TlsCertificateError)
                                            }
                                        }
                                    },
                                    |dst| Ok(()),
                                )
                            }
                        }
                        Self::TlsAlertReceived(value) => {
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
                                .write(wasmtime::ValRaw::u32(14u32));
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
                                                m.map(|p| &raw mut (*p).TlsAlertReceived)
                                            }
                                        }
                                    },
                                    |dst| {
                                        value
                                            .lower(
                                                cx,
                                                ty
                                                    .cases[14usize]
                                                    .ty
                                                    .unwrap_or_else(
                                                        wasmtime::component::__internal::bad_type_info,
                                                    ),
                                                dst,
                                            )
                                    },
                                )
                            }
                        }
                        Self::HttpRequestDenied => {
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
                                .write(wasmtime::ValRaw::u32(15u32));
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
                                                m.map(|p| &raw mut (*p).HttpRequestDenied)
                                            }
                                        }
                                    },
                                    |dst| Ok(()),
                                )
                            }
                        }
                        Self::HttpRequestLengthRequired => {
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
                                .write(wasmtime::ValRaw::u32(16u32));
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
                                                m.map(|p| &raw mut (*p).HttpRequestLengthRequired)
                                            }
                                        }
                                    },
                                    |dst| Ok(()),
                                )
                            }
                        }
                        Self::HttpRequestBodySize(value) => {
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
                                .write(wasmtime::ValRaw::u32(17u32));
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
                                                m.map(|p| &raw mut (*p).HttpRequestBodySize)
                                            }
                                        }
                                    },
                                    |dst| {
                                        value
                                            .lower(
                                                cx,
                                                ty
                                                    .cases[17usize]
                                                    .ty
                                                    .unwrap_or_else(
                                                        wasmtime::component::__internal::bad_type_info,
                                                    ),
                                                dst,
                                            )
                                    },
                                )
                            }
                        }
                        Self::HttpRequestMethodInvalid => {
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
                                .write(wasmtime::ValRaw::u32(18u32));
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
                                                m.map(|p| &raw mut (*p).HttpRequestMethodInvalid)
                                            }
                                        }
                                    },
                                    |dst| Ok(()),
                                )
                            }
                        }
                        Self::HttpRequestUriInvalid => {
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
                                .write(wasmtime::ValRaw::u32(19u32));
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
                                                m.map(|p| &raw mut (*p).HttpRequestUriInvalid)
                                            }
                                        }
                                    },
                                    |dst| Ok(()),
                                )
                            }
                        }
                        Self::HttpRequestUriTooLong => {
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
                                .write(wasmtime::ValRaw::u32(20u32));
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
                                                m.map(|p| &raw mut (*p).HttpRequestUriTooLong)
                                            }
                                        }
                                    },
                                    |dst| Ok(()),
                                )
                            }
                        }
                        Self::HttpRequestHeaderSectionSize(value) => {
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
                                .write(wasmtime::ValRaw::u32(21u32));
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
                                                m.map(|p| &raw mut (*p).HttpRequestHeaderSectionSize)
                                            }
                                        }
                                    },
                                    |dst| {
                                        value
                                            .lower(
                                                cx,
                                                ty
                                                    .cases[21usize]
                                                    .ty
                                                    .unwrap_or_else(
                                                        wasmtime::component::__internal::bad_type_info,
                                                    ),
                                                dst,
                                            )
                                    },
                                )
                            }
                        }
                        Self::HttpRequestHeaderSize(value) => {
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
                                .write(wasmtime::ValRaw::u32(22u32));
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
                                                m.map(|p| &raw mut (*p).HttpRequestHeaderSize)
                                            }
                                        }
                                    },
                                    |dst| {
                                        value
                                            .lower(
                                                cx,
                                                ty
                                                    .cases[22usize]
                                                    .ty
                                                    .unwrap_or_else(
                                                        wasmtime::component::__internal::bad_type_info,
                                                    ),
                                                dst,
                                            )
                                    },
                                )
                            }
                        }
                        Self::HttpRequestTrailerSectionSize(value) => {
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
                                .write(wasmtime::ValRaw::u32(23u32));
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
                                                m.map(|p| &raw mut (*p).HttpRequestTrailerSectionSize)
                                            }
                                        }
                                    },
                                    |dst| {
                                        value
                                            .lower(
                                                cx,
                                                ty
                                                    .cases[23usize]
                                                    .ty
                                                    .unwrap_or_else(
                                                        wasmtime::component::__internal::bad_type_info,
                                                    ),
                                                dst,
                                            )
                                    },
                                )
                            }
                        }
                        Self::HttpRequestTrailerSize(value) => {
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
                                .write(wasmtime::ValRaw::u32(24u32));
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
                                                m.map(|p| &raw mut (*p).HttpRequestTrailerSize)
                                            }
                                        }
                                    },
                                    |dst| {
                                        value
                                            .lower(
                                                cx,
                                                ty
                                                    .cases[24usize]
                                                    .ty
                                                    .unwrap_or_else(
                                                        wasmtime::component::__internal::bad_type_info,
                                                    ),
                                                dst,
                                            )
                                    },
                                )
                            }
                        }
                        Self::HttpResponseIncomplete => {
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
                                .write(wasmtime::ValRaw::u32(25u32));
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
                                                m.map(|p| &raw mut (*p).HttpResponseIncomplete)
                                            }
                                        }
                                    },
                                    |dst| Ok(()),
                                )
                            }
                        }
                        Self::HttpResponseHeaderSectionSize(value) => {
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
                                .write(wasmtime::ValRaw::u32(26u32));
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
                                                m.map(|p| &raw mut (*p).HttpResponseHeaderSectionSize)
                                            }
                                        }
                                    },
                                    |dst| {
                                        value
                                            .lower(
                                                cx,
                                                ty
                                                    .cases[26usize]
                                                    .ty
                                                    .unwrap_or_else(
                                                        wasmtime::component::__internal::bad_type_info,
                                                    ),
                                                dst,
                                            )
                                    },
                                )
                            }
                        }
                        Self::HttpResponseHeaderSize(value) => {
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
                                .write(wasmtime::ValRaw::u32(27u32));
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
                                                m.map(|p| &raw mut (*p).HttpResponseHeaderSize)
                                            }
                                        }
                                    },
                                    |dst| {
                                        value
                                            .lower(
                                                cx,
                                                ty
                                                    .cases[27usize]
                                                    .ty
                                                    .unwrap_or_else(
                                                        wasmtime::component::__internal::bad_type_info,
                                                    ),
                                                dst,
                                            )
                                    },
                                )
                            }
                        }
                        Self::HttpResponseBodySize(value) => {
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
                                .write(wasmtime::ValRaw::u32(28u32));
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
                                                m.map(|p| &raw mut (*p).HttpResponseBodySize)
                                            }
                                        }
                                    },
                                    |dst| {
                                        value
                                            .lower(
                                                cx,
                                                ty
                                                    .cases[28usize]
                                                    .ty
                                                    .unwrap_or_else(
                                                        wasmtime::component::__internal::bad_type_info,
                                                    ),
                                                dst,
                                            )
                                    },
                                )
                            }
                        }
                        Self::HttpResponseTrailerSectionSize(value) => {
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
                                .write(wasmtime::ValRaw::u32(29u32));
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
                                                m.map(|p| &raw mut (*p).HttpResponseTrailerSectionSize)
                                            }
                                        }
                                    },
                                    |dst| {
                                        value
                                            .lower(
                                                cx,
                                                ty
                                                    .cases[29usize]
                                                    .ty
                                                    .unwrap_or_else(
                                                        wasmtime::component::__internal::bad_type_info,
                                                    ),
                                                dst,
                                            )
                                    },
                                )
                            }
                        }
                        Self::HttpResponseTrailerSize(value) => {
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
                                .write(wasmtime::ValRaw::u32(30u32));
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
                                                m.map(|p| &raw mut (*p).HttpResponseTrailerSize)
                                            }
                                        }
                                    },
                                    |dst| {
                                        value
                                            .lower(
                                                cx,
                                                ty
                                                    .cases[30usize]
                                                    .ty
                                                    .unwrap_or_else(
                                                        wasmtime::component::__internal::bad_type_info,
                                                    ),
                                                dst,
                                            )
                                    },
                                )
                            }
                        }
                        Self::HttpResponseTransferCoding(value) => {
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
                                .write(wasmtime::ValRaw::u32(31u32));
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
                                                m.map(|p| &raw mut (*p).HttpResponseTransferCoding)
                                            }
                                        }
                                    },
                                    |dst| {
                                        value
                                            .lower(
                                                cx,
                                                ty
                                                    .cases[31usize]
                                                    .ty
                                                    .unwrap_or_else(
                                                        wasmtime::component::__internal::bad_type_info,
                                                    ),
                                                dst,
                                            )
                                    },
                                )
                            }
                        }
                        Self::HttpResponseContentCoding(value) => {
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
                                .write(wasmtime::ValRaw::u32(32u32));
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
                                                m.map(|p| &raw mut (*p).HttpResponseContentCoding)
                                            }
                                        }
                                    },
                                    |dst| {
                                        value
                                            .lower(
                                                cx,
                                                ty
                                                    .cases[32usize]
                                                    .ty
                                                    .unwrap_or_else(
                                                        wasmtime::component::__internal::bad_type_info,
                                                    ),
                                                dst,
                                            )
                                    },
                                )
                            }
                        }
                        Self::HttpResponseTimeout => {
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
                                .write(wasmtime::ValRaw::u32(33u32));
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
                                                m.map(|p| &raw mut (*p).HttpResponseTimeout)
                                            }
                                        }
                                    },
                                    |dst| Ok(()),
                                )
                            }
                        }
                        Self::HttpUpgradeFailed => {
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
                                .write(wasmtime::ValRaw::u32(34u32));
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
                                                m.map(|p| &raw mut (*p).HttpUpgradeFailed)
                                            }
                                        }
                                    },
                                    |dst| Ok(()),
                                )
                            }
                        }
                        Self::HttpProtocolError => {
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
                                .write(wasmtime::ValRaw::u32(35u32));
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
                                                m.map(|p| &raw mut (*p).HttpProtocolError)
                                            }
                                        }
                                    },
                                    |dst| Ok(()),
                                )
                            }
                        }
                        Self::LoopDetected => {
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
                                .write(wasmtime::ValRaw::u32(36u32));
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
                                                m.map(|p| &raw mut (*p).LoopDetected)
                                            }
                                        }
                                    },
                                    |dst| Ok(()),
                                )
                            }
                        }
                        Self::ConfigurationError => {
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
                                .write(wasmtime::ValRaw::u32(37u32));
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
                                                m.map(|p| &raw mut (*p).ConfigurationError)
                                            }
                                        }
                                    },
                                    |dst| Ok(()),
                                )
                            }
                        }
                        Self::InternalError(value) => {
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
                                .write(wasmtime::ValRaw::u32(38u32));
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
                                                m.map(|p| &raw mut (*p).InternalError)
                                            }
                                        }
                                    },
                                    |dst| {
                                        value
                                            .lower(
                                                cx,
                                                ty
                                                    .cases[38usize]
                                                    .ty
                                                    .unwrap_or_else(
                                                        wasmtime::component::__internal::bad_type_info,
                                                    ),
                                                dst,
                                            )
                                    },
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
                        wasmtime::component::__internal::InterfaceType::Variant(i) => {
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
                        Self::DnsTimeout => {
                            *cx.get::<1usize>(offset) = 0u8.to_le_bytes();
                            Ok(())
                        }
                        Self::DnsError(value) => {
                            *cx.get::<1usize>(offset) = 1u8.to_le_bytes();
                            value
                                .store(
                                    cx,
                                    ty
                                        .cases[1usize]
                                        .ty
                                        .unwrap_or_else(
                                            wasmtime::component::__internal::bad_type_info,
                                        ),
                                    offset
                                        + <Self as wasmtime::component::__internal::ComponentVariant>::PAYLOAD_OFFSET32,
                                )
                        }
                        Self::DestinationNotFound => {
                            *cx.get::<1usize>(offset) = 2u8.to_le_bytes();
                            Ok(())
                        }
                        Self::DestinationUnavailable => {
                            *cx.get::<1usize>(offset) = 3u8.to_le_bytes();
                            Ok(())
                        }
                        Self::DestinationIpProhibited => {
                            *cx.get::<1usize>(offset) = 4u8.to_le_bytes();
                            Ok(())
                        }
                        Self::DestinationIpUnroutable => {
                            *cx.get::<1usize>(offset) = 5u8.to_le_bytes();
                            Ok(())
                        }
                        Self::ConnectionRefused => {
                            *cx.get::<1usize>(offset) = 6u8.to_le_bytes();
                            Ok(())
                        }
                        Self::ConnectionTerminated => {
                            *cx.get::<1usize>(offset) = 7u8.to_le_bytes();
                            Ok(())
                        }
                        Self::ConnectionTimeout => {
                            *cx.get::<1usize>(offset) = 8u8.to_le_bytes();
                            Ok(())
                        }
                        Self::ConnectionReadTimeout => {
                            *cx.get::<1usize>(offset) = 9u8.to_le_bytes();
                            Ok(())
                        }
                        Self::ConnectionWriteTimeout => {
                            *cx.get::<1usize>(offset) = 10u8.to_le_bytes();
                            Ok(())
                        }
                        Self::ConnectionLimitReached => {
                            *cx.get::<1usize>(offset) = 11u8.to_le_bytes();
                            Ok(())
                        }
                        Self::TlsProtocolError => {
                            *cx.get::<1usize>(offset) = 12u8.to_le_bytes();
                            Ok(())
                        }
                        Self::TlsCertificateError => {
                            *cx.get::<1usize>(offset) = 13u8.to_le_bytes();
                            Ok(())
                        }
                        Self::TlsAlertReceived(value) => {
                            *cx.get::<1usize>(offset) = 14u8.to_le_bytes();
                            value
                                .store(
                                    cx,
                                    ty
                                        .cases[14usize]
                                        .ty
                                        .unwrap_or_else(
                                            wasmtime::component::__internal::bad_type_info,
                                        ),
                                    offset
                                        + <Self as wasmtime::component::__internal::ComponentVariant>::PAYLOAD_OFFSET32,
                                )
                        }
                        Self::HttpRequestDenied => {
                            *cx.get::<1usize>(offset) = 15u8.to_le_bytes();
                            Ok(())
                        }
                        Self::HttpRequestLengthRequired => {
                            *cx.get::<1usize>(offset) = 16u8.to_le_bytes();
                            Ok(())
                        }
                        Self::HttpRequestBodySize(value) => {
                            *cx.get::<1usize>(offset) = 17u8.to_le_bytes();
                            value
                                .store(
                                    cx,
                                    ty
                                        .cases[17usize]
                                        .ty
                                        .unwrap_or_else(
                                            wasmtime::component::__internal::bad_type_info,
                                        ),
                                    offset
                                        + <Self as wasmtime::component::__internal::ComponentVariant>::PAYLOAD_OFFSET32,
                                )
                        }
                        Self::HttpRequestMethodInvalid => {
                            *cx.get::<1usize>(offset) = 18u8.to_le_bytes();
                            Ok(())
                        }
                        Self::HttpRequestUriInvalid => {
                            *cx.get::<1usize>(offset) = 19u8.to_le_bytes();
                            Ok(())
                        }
                        Self::HttpRequestUriTooLong => {
                            *cx.get::<1usize>(offset) = 20u8.to_le_bytes();
                            Ok(())
                        }
                        Self::HttpRequestHeaderSectionSize(value) => {
                            *cx.get::<1usize>(offset) = 21u8.to_le_bytes();
                            value
                                .store(
                                    cx,
                                    ty
                                        .cases[21usize]
                                        .ty
                                        .unwrap_or_else(
                                            wasmtime::component::__internal::bad_type_info,
                                        ),
                                    offset
                                        + <Self as wasmtime::component::__internal::ComponentVariant>::PAYLOAD_OFFSET32,
                                )
                        }
                        Self::HttpRequestHeaderSize(value) => {
                            *cx.get::<1usize>(offset) = 22u8.to_le_bytes();
                            value
                                .store(
                                    cx,
                                    ty
                                        .cases[22usize]
                                        .ty
                                        .unwrap_or_else(
                                            wasmtime::component::__internal::bad_type_info,
                                        ),
                                    offset
                                        + <Self as wasmtime::component::__internal::ComponentVariant>::PAYLOAD_OFFSET32,
                                )
                        }
                        Self::HttpRequestTrailerSectionSize(value) => {
                            *cx.get::<1usize>(offset) = 23u8.to_le_bytes();
                            value
                                .store(
                                    cx,
                                    ty
                                        .cases[23usize]
                                        .ty
                                        .unwrap_or_else(
                                            wasmtime::component::__internal::bad_type_info,
                                        ),
                                    offset
                                        + <Self as wasmtime::component::__internal::ComponentVariant>::PAYLOAD_OFFSET32,
                                )
                        }
                        Self::HttpRequestTrailerSize(value) => {
                            *cx.get::<1usize>(offset) = 24u8.to_le_bytes();
                            value
                                .store(
                                    cx,
                                    ty
                                        .cases[24usize]
                                        .ty
                                        .unwrap_or_else(
                                            wasmtime::component::__internal::bad_type_info,
                                        ),
                                    offset
                                        + <Self as wasmtime::component::__internal::ComponentVariant>::PAYLOAD_OFFSET32,
                                )
                        }
                        Self::HttpResponseIncomplete => {
                            *cx.get::<1usize>(offset) = 25u8.to_le_bytes();
                            Ok(())
                        }
                        Self::HttpResponseHeaderSectionSize(value) => {
                            *cx.get::<1usize>(offset) = 26u8.to_le_bytes();
                            value
                                .store(
                                    cx,
                                    ty
                                        .cases[26usize]
                                        .ty
                                        .unwrap_or_else(
                                            wasmtime::component::__internal::bad_type_info,
                                        ),
                                    offset
                                        + <Self as wasmtime::component::__internal::ComponentVariant>::PAYLOAD_OFFSET32,
                                )
                        }
                        Self::HttpResponseHeaderSize(value) => {
                            *cx.get::<1usize>(offset) = 27u8.to_le_bytes();
                            value
                                .store(
                                    cx,
                                    ty
                                        .cases[27usize]
                                        .ty
                                        .unwrap_or_else(
                                            wasmtime::component::__internal::bad_type_info,
                                        ),
                                    offset
                                        + <Self as wasmtime::component::__internal::ComponentVariant>::PAYLOAD_OFFSET32,
                                )
                        }
                        Self::HttpResponseBodySize(value) => {
                            *cx.get::<1usize>(offset) = 28u8.to_le_bytes();
                            value
                                .store(
                                    cx,
                                    ty
                                        .cases[28usize]
                                        .ty
                                        .unwrap_or_else(
                                            wasmtime::component::__internal::bad_type_info,
                                        ),
                                    offset
                                        + <Self as wasmtime::component::__internal::ComponentVariant>::PAYLOAD_OFFSET32,
                                )
                        }
                        Self::HttpResponseTrailerSectionSize(value) => {
                            *cx.get::<1usize>(offset) = 29u8.to_le_bytes();
                            value
                                .store(
                                    cx,
                                    ty
                                        .cases[29usize]
                                        .ty
                                        .unwrap_or_else(
                                            wasmtime::component::__internal::bad_type_info,
                                        ),
                                    offset
                                        + <Self as wasmtime::component::__internal::ComponentVariant>::PAYLOAD_OFFSET32,
                                )
                        }
                        Self::HttpResponseTrailerSize(value) => {
                            *cx.get::<1usize>(offset) = 30u8.to_le_bytes();
                            value
                                .store(
                                    cx,
                                    ty
                                        .cases[30usize]
                                        .ty
                                        .unwrap_or_else(
                                            wasmtime::component::__internal::bad_type_info,
                                        ),
                                    offset
                                        + <Self as wasmtime::component::__internal::ComponentVariant>::PAYLOAD_OFFSET32,
                                )
                        }
                        Self::HttpResponseTransferCoding(value) => {
                            *cx.get::<1usize>(offset) = 31u8.to_le_bytes();
                            value
                                .store(
                                    cx,
                                    ty
                                        .cases[31usize]
                                        .ty
                                        .unwrap_or_else(
                                            wasmtime::component::__internal::bad_type_info,
                                        ),
                                    offset
                                        + <Self as wasmtime::component::__internal::ComponentVariant>::PAYLOAD_OFFSET32,
                                )
                        }
                        Self::HttpResponseContentCoding(value) => {
                            *cx.get::<1usize>(offset) = 32u8.to_le_bytes();
                            value
                                .store(
                                    cx,
                                    ty
                                        .cases[32usize]
                                        .ty
                                        .unwrap_or_else(
                                            wasmtime::component::__internal::bad_type_info,
                                        ),
                                    offset
                                        + <Self as wasmtime::component::__internal::ComponentVariant>::PAYLOAD_OFFSET32,
                                )
                        }
                        Self::HttpResponseTimeout => {
                            *cx.get::<1usize>(offset) = 33u8.to_le_bytes();
                            Ok(())
                        }
                        Self::HttpUpgradeFailed => {
                            *cx.get::<1usize>(offset) = 34u8.to_le_bytes();
                            Ok(())
                        }
                        Self::HttpProtocolError => {
                            *cx.get::<1usize>(offset) = 35u8.to_le_bytes();
                            Ok(())
                        }
                        Self::LoopDetected => {
                            *cx.get::<1usize>(offset) = 36u8.to_le_bytes();
                            Ok(())
                        }
                        Self::ConfigurationError => {
                            *cx.get::<1usize>(offset) = 37u8.to_le_bytes();
                            Ok(())
                        }
                        Self::InternalError(value) => {
                            *cx.get::<1usize>(offset) = 38u8.to_le_bytes();
                            value
                                .store(
                                    cx,
                                    ty
                                        .cases[38usize]
                                        .ty
                                        .unwrap_or_else(
                                            wasmtime::component::__internal::bad_type_info,
                                        ),
                                    offset
                                        + <Self as wasmtime::component::__internal::ComponentVariant>::PAYLOAD_OFFSET32,
                                )
                        }
                    }
                }
            }
            unsafe impl wasmtime::component::Lift for ErrorCode {
                #[inline]
                fn lift(
                    cx: &mut wasmtime::component::__internal::LiftContext<'_>,
                    ty: wasmtime::component::__internal::InterfaceType,
                    src: &Self::Lower,
                ) -> wasmtime::component::__internal::anyhow::Result<Self> {
                    let ty = match ty {
                        wasmtime::component::__internal::InterfaceType::Variant(i) => {
                            &cx.types[i]
                        }
                        _ => wasmtime::component::__internal::bad_type_info(),
                    };
                    Ok(
                        match src.tag.get_u32() {
                            0u32 => Self::DnsTimeout,
                            1u32 => {
                                Self::DnsError(
                                    <DnsErrorPayload as wasmtime::component::Lift>::lift(
                                        cx,
                                        ty
                                            .cases[1usize]
                                            .ty
                                            .unwrap_or_else(
                                                wasmtime::component::__internal::bad_type_info,
                                            ),
                                        unsafe { &src.payload.DnsError },
                                    )?,
                                )
                            }
                            2u32 => Self::DestinationNotFound,
                            3u32 => Self::DestinationUnavailable,
                            4u32 => Self::DestinationIpProhibited,
                            5u32 => Self::DestinationIpUnroutable,
                            6u32 => Self::ConnectionRefused,
                            7u32 => Self::ConnectionTerminated,
                            8u32 => Self::ConnectionTimeout,
                            9u32 => Self::ConnectionReadTimeout,
                            10u32 => Self::ConnectionWriteTimeout,
                            11u32 => Self::ConnectionLimitReached,
                            12u32 => Self::TlsProtocolError,
                            13u32 => Self::TlsCertificateError,
                            14u32 => {
                                Self::TlsAlertReceived(
                                    <TlsAlertReceivedPayload as wasmtime::component::Lift>::lift(
                                        cx,
                                        ty
                                            .cases[14usize]
                                            .ty
                                            .unwrap_or_else(
                                                wasmtime::component::__internal::bad_type_info,
                                            ),
                                        unsafe { &src.payload.TlsAlertReceived },
                                    )?,
                                )
                            }
                            15u32 => Self::HttpRequestDenied,
                            16u32 => Self::HttpRequestLengthRequired,
                            17u32 => {
                                Self::HttpRequestBodySize(
                                    <Option<
                                        u64,
                                    > as wasmtime::component::Lift>::lift(
                                        cx,
                                        ty
                                            .cases[17usize]
                                            .ty
                                            .unwrap_or_else(
                                                wasmtime::component::__internal::bad_type_info,
                                            ),
                                        unsafe { &src.payload.HttpRequestBodySize },
                                    )?,
                                )
                            }
                            18u32 => Self::HttpRequestMethodInvalid,
                            19u32 => Self::HttpRequestUriInvalid,
                            20u32 => Self::HttpRequestUriTooLong,
                            21u32 => {
                                Self::HttpRequestHeaderSectionSize(
                                    <Option<
                                        u32,
                                    > as wasmtime::component::Lift>::lift(
                                        cx,
                                        ty
                                            .cases[21usize]
                                            .ty
                                            .unwrap_or_else(
                                                wasmtime::component::__internal::bad_type_info,
                                            ),
                                        unsafe { &src.payload.HttpRequestHeaderSectionSize },
                                    )?,
                                )
                            }
                            22u32 => {
                                Self::HttpRequestHeaderSize(
                                    <Option<
                                        FieldSizePayload,
                                    > as wasmtime::component::Lift>::lift(
                                        cx,
                                        ty
                                            .cases[22usize]
                                            .ty
                                            .unwrap_or_else(
                                                wasmtime::component::__internal::bad_type_info,
                                            ),
                                        unsafe { &src.payload.HttpRequestHeaderSize },
                                    )?,
                                )
                            }
                            23u32 => {
                                Self::HttpRequestTrailerSectionSize(
                                    <Option<
                                        u32,
                                    > as wasmtime::component::Lift>::lift(
                                        cx,
                                        ty
                                            .cases[23usize]
                                            .ty
                                            .unwrap_or_else(
                                                wasmtime::component::__internal::bad_type_info,
                                            ),
                                        unsafe { &src.payload.HttpRequestTrailerSectionSize },
                                    )?,
                                )
                            }
                            24u32 => {
                                Self::HttpRequestTrailerSize(
                                    <FieldSizePayload as wasmtime::component::Lift>::lift(
                                        cx,
                                        ty
                                            .cases[24usize]
                                            .ty
                                            .unwrap_or_else(
                                                wasmtime::component::__internal::bad_type_info,
                                            ),
                                        unsafe { &src.payload.HttpRequestTrailerSize },
                                    )?,
                                )
                            }
                            25u32 => Self::HttpResponseIncomplete,
                            26u32 => {
                                Self::HttpResponseHeaderSectionSize(
                                    <Option<
                                        u32,
                                    > as wasmtime::component::Lift>::lift(
                                        cx,
                                        ty
                                            .cases[26usize]
                                            .ty
                                            .unwrap_or_else(
                                                wasmtime::component::__internal::bad_type_info,
                                            ),
                                        unsafe { &src.payload.HttpResponseHeaderSectionSize },
                                    )?,
                                )
                            }
                            27u32 => {
                                Self::HttpResponseHeaderSize(
                                    <FieldSizePayload as wasmtime::component::Lift>::lift(
                                        cx,
                                        ty
                                            .cases[27usize]
                                            .ty
                                            .unwrap_or_else(
                                                wasmtime::component::__internal::bad_type_info,
                                            ),
                                        unsafe { &src.payload.HttpResponseHeaderSize },
                                    )?,
                                )
                            }
                            28u32 => {
                                Self::HttpResponseBodySize(
                                    <Option<
                                        u64,
                                    > as wasmtime::component::Lift>::lift(
                                        cx,
                                        ty
                                            .cases[28usize]
                                            .ty
                                            .unwrap_or_else(
                                                wasmtime::component::__internal::bad_type_info,
                                            ),
                                        unsafe { &src.payload.HttpResponseBodySize },
                                    )?,
                                )
                            }
                            29u32 => {
                                Self::HttpResponseTrailerSectionSize(
                                    <Option<
                                        u32,
                                    > as wasmtime::component::Lift>::lift(
                                        cx,
                                        ty
                                            .cases[29usize]
                                            .ty
                                            .unwrap_or_else(
                                                wasmtime::component::__internal::bad_type_info,
                                            ),
                                        unsafe { &src.payload.HttpResponseTrailerSectionSize },
                                    )?,
                                )
                            }
                            30u32 => {
                                Self::HttpResponseTrailerSize(
                                    <FieldSizePayload as wasmtime::component::Lift>::lift(
                                        cx,
                                        ty
                                            .cases[30usize]
                                            .ty
                                            .unwrap_or_else(
                                                wasmtime::component::__internal::bad_type_info,
                                            ),
                                        unsafe { &src.payload.HttpResponseTrailerSize },
                                    )?,
                                )
                            }
                            31u32 => {
                                Self::HttpResponseTransferCoding(
                                    <Option<
                                        String,
                                    > as wasmtime::component::Lift>::lift(
                                        cx,
                                        ty
                                            .cases[31usize]
                                            .ty
                                            .unwrap_or_else(
                                                wasmtime::component::__internal::bad_type_info,
                                            ),
                                        unsafe { &src.payload.HttpResponseTransferCoding },
                                    )?,
                                )
                            }
                            32u32 => {
                                Self::HttpResponseContentCoding(
                                    <Option<
                                        String,
                                    > as wasmtime::component::Lift>::lift(
                                        cx,
                                        ty
                                            .cases[32usize]
                                            .ty
                                            .unwrap_or_else(
                                                wasmtime::component::__internal::bad_type_info,
                                            ),
                                        unsafe { &src.payload.HttpResponseContentCoding },
                                    )?,
                                )
                            }
                            33u32 => Self::HttpResponseTimeout,
                            34u32 => Self::HttpUpgradeFailed,
                            35u32 => Self::HttpProtocolError,
                            36u32 => Self::LoopDetected,
                            37u32 => Self::ConfigurationError,
                            38u32 => {
                                Self::InternalError(
                                    <Option<
                                        String,
                                    > as wasmtime::component::Lift>::lift(
                                        cx,
                                        ty
                                            .cases[38usize]
                                            .ty
                                            .unwrap_or_else(
                                                wasmtime::component::__internal::bad_type_info,
                                            ),
                                        unsafe { &src.payload.InternalError },
                                    )?,
                                )
                            }
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
                        wasmtime::component::__internal::InterfaceType::Variant(i) => {
                            &cx.types[i]
                        }
                        _ => wasmtime::component::__internal::bad_type_info(),
                    };
                    Ok(
                        match discrim {
                            0u8 => Self::DnsTimeout,
                            1u8 => {
                                Self::DnsError(
                                    <DnsErrorPayload as wasmtime::component::Lift>::load(
                                        cx,
                                        ty
                                            .cases[1usize]
                                            .ty
                                            .unwrap_or_else(
                                                wasmtime::component::__internal::bad_type_info,
                                            ),
                                        &payload[..<DnsErrorPayload as wasmtime::component::ComponentType>::SIZE32],
                                    )?,
                                )
                            }
                            2u8 => Self::DestinationNotFound,
                            3u8 => Self::DestinationUnavailable,
                            4u8 => Self::DestinationIpProhibited,
                            5u8 => Self::DestinationIpUnroutable,
                            6u8 => Self::ConnectionRefused,
                            7u8 => Self::ConnectionTerminated,
                            8u8 => Self::ConnectionTimeout,
                            9u8 => Self::ConnectionReadTimeout,
                            10u8 => Self::ConnectionWriteTimeout,
                            11u8 => Self::ConnectionLimitReached,
                            12u8 => Self::TlsProtocolError,
                            13u8 => Self::TlsCertificateError,
                            14u8 => {
                                Self::TlsAlertReceived(
                                    <TlsAlertReceivedPayload as wasmtime::component::Lift>::load(
                                        cx,
                                        ty
                                            .cases[14usize]
                                            .ty
                                            .unwrap_or_else(
                                                wasmtime::component::__internal::bad_type_info,
                                            ),
                                        &payload[..<TlsAlertReceivedPayload as wasmtime::component::ComponentType>::SIZE32],
                                    )?,
                                )
                            }
                            15u8 => Self::HttpRequestDenied,
                            16u8 => Self::HttpRequestLengthRequired,
                            17u8 => {
                                Self::HttpRequestBodySize(
                                    <Option<
                                        u64,
                                    > as wasmtime::component::Lift>::load(
                                        cx,
                                        ty
                                            .cases[17usize]
                                            .ty
                                            .unwrap_or_else(
                                                wasmtime::component::__internal::bad_type_info,
                                            ),
                                        &payload[..<Option<
                                            u64,
                                        > as wasmtime::component::ComponentType>::SIZE32],
                                    )?,
                                )
                            }
                            18u8 => Self::HttpRequestMethodInvalid,
                            19u8 => Self::HttpRequestUriInvalid,
                            20u8 => Self::HttpRequestUriTooLong,
                            21u8 => {
                                Self::HttpRequestHeaderSectionSize(
                                    <Option<
                                        u32,
                                    > as wasmtime::component::Lift>::load(
                                        cx,
                                        ty
                                            .cases[21usize]
                                            .ty
                                            .unwrap_or_else(
                                                wasmtime::component::__internal::bad_type_info,
                                            ),
                                        &payload[..<Option<
                                            u32,
                                        > as wasmtime::component::ComponentType>::SIZE32],
                                    )?,
                                )
                            }
                            22u8 => {
                                Self::HttpRequestHeaderSize(
                                    <Option<
                                        FieldSizePayload,
                                    > as wasmtime::component::Lift>::load(
                                        cx,
                                        ty
                                            .cases[22usize]
                                            .ty
                                            .unwrap_or_else(
                                                wasmtime::component::__internal::bad_type_info,
                                            ),
                                        &payload[..<Option<
                                            FieldSizePayload,
                                        > as wasmtime::component::ComponentType>::SIZE32],
                                    )?,
                                )
                            }
                            23u8 => {
                                Self::HttpRequestTrailerSectionSize(
                                    <Option<
                                        u32,
                                    > as wasmtime::component::Lift>::load(
                                        cx,
                                        ty
                                            .cases[23usize]
                                            .ty
                                            .unwrap_or_else(
                                                wasmtime::component::__internal::bad_type_info,
                                            ),
                                        &payload[..<Option<
                                            u32,
                                        > as wasmtime::component::ComponentType>::SIZE32],
                                    )?,
                                )
                            }
                            24u8 => {
                                Self::HttpRequestTrailerSize(
                                    <FieldSizePayload as wasmtime::component::Lift>::load(
                                        cx,
                                        ty
                                            .cases[24usize]
                                            .ty
                                            .unwrap_or_else(
                                                wasmtime::component::__internal::bad_type_info,
                                            ),
                                        &payload[..<FieldSizePayload as wasmtime::component::ComponentType>::SIZE32],
                                    )?,
                                )
                            }
                            25u8 => Self::HttpResponseIncomplete,
                            26u8 => {
                                Self::HttpResponseHeaderSectionSize(
                                    <Option<
                                        u32,
                                    > as wasmtime::component::Lift>::load(
                                        cx,
                                        ty
                                            .cases[26usize]
                                            .ty
                                            .unwrap_or_else(
                                                wasmtime::component::__internal::bad_type_info,
                                            ),
                                        &payload[..<Option<
                                            u32,
                                        > as wasmtime::component::ComponentType>::SIZE32],
                                    )?,
                                )
                            }
                            27u8 => {
                                Self::HttpResponseHeaderSize(
                                    <FieldSizePayload as wasmtime::component::Lift>::load(
                                        cx,
                                        ty
                                            .cases[27usize]
                                            .ty
                                            .unwrap_or_else(
                                                wasmtime::component::__internal::bad_type_info,
                                            ),
                                        &payload[..<FieldSizePayload as wasmtime::component::ComponentType>::SIZE32],
                                    )?,
                                )
                            }
                            28u8 => {
                                Self::HttpResponseBodySize(
                                    <Option<
                                        u64,
                                    > as wasmtime::component::Lift>::load(
                                        cx,
                                        ty
                                            .cases[28usize]
                                            .ty
                                            .unwrap_or_else(
                                                wasmtime::component::__internal::bad_type_info,
                                            ),
                                        &payload[..<Option<
                                            u64,
                                        > as wasmtime::component::ComponentType>::SIZE32],
                                    )?,
                                )
                            }
                            29u8 => {
                                Self::HttpResponseTrailerSectionSize(
                                    <Option<
                                        u32,
                                    > as wasmtime::component::Lift>::load(
                                        cx,
                                        ty
                                            .cases[29usize]
                                            .ty
                                            .unwrap_or_else(
                                                wasmtime::component::__internal::bad_type_info,
                                            ),
                                        &payload[..<Option<
                                            u32,
                                        > as wasmtime::component::ComponentType>::SIZE32],
                                    )?,
                                )
                            }
                            30u8 => {
                                Self::HttpResponseTrailerSize(
                                    <FieldSizePayload as wasmtime::component::Lift>::load(
                                        cx,
                                        ty
                                            .cases[30usize]
                                            .ty
                                            .unwrap_or_else(
                                                wasmtime::component::__internal::bad_type_info,
                                            ),
                                        &payload[..<FieldSizePayload as wasmtime::component::ComponentType>::SIZE32],
                                    )?,
                                )
                            }
                            31u8 => {
                                Self::HttpResponseTransferCoding(
                                    <Option<
                                        String,
                                    > as wasmtime::component::Lift>::load(
                                        cx,
                                        ty
                                            .cases[31usize]
                                            .ty
                                            .unwrap_or_else(
                                                wasmtime::component::__internal::bad_type_info,
                                            ),
                                        &payload[..<Option<
                                            String,
                                        > as wasmtime::component::ComponentType>::SIZE32],
                                    )?,
                                )
                            }
                            32u8 => {
                                Self::HttpResponseContentCoding(
                                    <Option<
                                        String,
                                    > as wasmtime::component::Lift>::load(
                                        cx,
                                        ty
                                            .cases[32usize]
                                            .ty
                                            .unwrap_or_else(
                                                wasmtime::component::__internal::bad_type_info,
                                            ),
                                        &payload[..<Option<
                                            String,
                                        > as wasmtime::component::ComponentType>::SIZE32],
                                    )?,
                                )
                            }
                            33u8 => Self::HttpResponseTimeout,
                            34u8 => Self::HttpUpgradeFailed,
                            35u8 => Self::HttpProtocolError,
                            36u8 => Self::LoopDetected,
                            37u8 => Self::ConfigurationError,
                            38u8 => {
                                Self::InternalError(
                                    <Option<
                                        String,
                                    > as wasmtime::component::Lift>::load(
                                        cx,
                                        ty
                                            .cases[38usize]
                                            .ty
                                            .unwrap_or_else(
                                                wasmtime::component::__internal::bad_type_info,
                                            ),
                                        &payload[..<Option<
                                            String,
                                        > as wasmtime::component::ComponentType>::SIZE32],
                                    )?,
                                )
                            }
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
                pub struct LowerErrorCode<
                    T1: Copy,
                    T14: Copy,
                    T17: Copy,
                    T21: Copy,
                    T22: Copy,
                    T23: Copy,
                    T24: Copy,
                    T26: Copy,
                    T27: Copy,
                    T28: Copy,
                    T29: Copy,
                    T30: Copy,
                    T31: Copy,
                    T32: Copy,
                    T38: Copy,
                > {
                    tag: wasmtime::ValRaw,
                    payload: LowerPayloadErrorCode<
                        T1,
                        T14,
                        T17,
                        T21,
                        T22,
                        T23,
                        T24,
                        T26,
                        T27,
                        T28,
                        T29,
                        T30,
                        T31,
                        T32,
                        T38,
                    >,
                }
                #[automatically_derived]
                impl<
                    T1: ::core::clone::Clone + Copy,
                    T14: ::core::clone::Clone + Copy,
                    T17: ::core::clone::Clone + Copy,
                    T21: ::core::clone::Clone + Copy,
                    T22: ::core::clone::Clone + Copy,
                    T23: ::core::clone::Clone + Copy,
                    T24: ::core::clone::Clone + Copy,
                    T26: ::core::clone::Clone + Copy,
                    T27: ::core::clone::Clone + Copy,
                    T28: ::core::clone::Clone + Copy,
                    T29: ::core::clone::Clone + Copy,
                    T30: ::core::clone::Clone + Copy,
                    T31: ::core::clone::Clone + Copy,
                    T32: ::core::clone::Clone + Copy,
                    T38: ::core::clone::Clone + Copy,
                > ::core::clone::Clone
                for LowerErrorCode<
                    T1,
                    T14,
                    T17,
                    T21,
                    T22,
                    T23,
                    T24,
                    T26,
                    T27,
                    T28,
                    T29,
                    T30,
                    T31,
                    T32,
                    T38,
                > {
                    #[inline]
                    fn clone(
                        &self,
                    ) -> LowerErrorCode<
                        T1,
                        T14,
                        T17,
                        T21,
                        T22,
                        T23,
                        T24,
                        T26,
                        T27,
                        T28,
                        T29,
                        T30,
                        T31,
                        T32,
                        T38,
                    > {
                        LowerErrorCode {
                            tag: ::core::clone::Clone::clone(&self.tag),
                            payload: ::core::clone::Clone::clone(&self.payload),
                        }
                    }
                }
                #[automatically_derived]
                impl<
                    T1: ::core::marker::Copy + Copy,
                    T14: ::core::marker::Copy + Copy,
                    T17: ::core::marker::Copy + Copy,
                    T21: ::core::marker::Copy + Copy,
                    T22: ::core::marker::Copy + Copy,
                    T23: ::core::marker::Copy + Copy,
                    T24: ::core::marker::Copy + Copy,
                    T26: ::core::marker::Copy + Copy,
                    T27: ::core::marker::Copy + Copy,
                    T28: ::core::marker::Copy + Copy,
                    T29: ::core::marker::Copy + Copy,
                    T30: ::core::marker::Copy + Copy,
                    T31: ::core::marker::Copy + Copy,
                    T32: ::core::marker::Copy + Copy,
                    T38: ::core::marker::Copy + Copy,
                > ::core::marker::Copy
                for LowerErrorCode<
                    T1,
                    T14,
                    T17,
                    T21,
                    T22,
                    T23,
                    T24,
                    T26,
                    T27,
                    T28,
                    T29,
                    T30,
                    T31,
                    T32,
                    T38,
                > {}
                #[doc(hidden)]
                #[allow(non_snake_case)]
                #[repr(C)]
                union LowerPayloadErrorCode<
                    T1: Copy,
                    T14: Copy,
                    T17: Copy,
                    T21: Copy,
                    T22: Copy,
                    T23: Copy,
                    T24: Copy,
                    T26: Copy,
                    T27: Copy,
                    T28: Copy,
                    T29: Copy,
                    T30: Copy,
                    T31: Copy,
                    T32: Copy,
                    T38: Copy,
                > {
                    DnsTimeout: [wasmtime::ValRaw; 0],
                    DnsError: T1,
                    DestinationNotFound: [wasmtime::ValRaw; 0],
                    DestinationUnavailable: [wasmtime::ValRaw; 0],
                    DestinationIpProhibited: [wasmtime::ValRaw; 0],
                    DestinationIpUnroutable: [wasmtime::ValRaw; 0],
                    ConnectionRefused: [wasmtime::ValRaw; 0],
                    ConnectionTerminated: [wasmtime::ValRaw; 0],
                    ConnectionTimeout: [wasmtime::ValRaw; 0],
                    ConnectionReadTimeout: [wasmtime::ValRaw; 0],
                    ConnectionWriteTimeout: [wasmtime::ValRaw; 0],
                    ConnectionLimitReached: [wasmtime::ValRaw; 0],
                    TlsProtocolError: [wasmtime::ValRaw; 0],
                    TlsCertificateError: [wasmtime::ValRaw; 0],
                    TlsAlertReceived: T14,
                    HttpRequestDenied: [wasmtime::ValRaw; 0],
                    HttpRequestLengthRequired: [wasmtime::ValRaw; 0],
                    HttpRequestBodySize: T17,
                    HttpRequestMethodInvalid: [wasmtime::ValRaw; 0],
                    HttpRequestUriInvalid: [wasmtime::ValRaw; 0],
                    HttpRequestUriTooLong: [wasmtime::ValRaw; 0],
                    HttpRequestHeaderSectionSize: T21,
                    HttpRequestHeaderSize: T22,
                    HttpRequestTrailerSectionSize: T23,
                    HttpRequestTrailerSize: T24,
                    HttpResponseIncomplete: [wasmtime::ValRaw; 0],
                    HttpResponseHeaderSectionSize: T26,
                    HttpResponseHeaderSize: T27,
                    HttpResponseBodySize: T28,
                    HttpResponseTrailerSectionSize: T29,
                    HttpResponseTrailerSize: T30,
                    HttpResponseTransferCoding: T31,
                    HttpResponseContentCoding: T32,
                    HttpResponseTimeout: [wasmtime::ValRaw; 0],
                    HttpUpgradeFailed: [wasmtime::ValRaw; 0],
                    HttpProtocolError: [wasmtime::ValRaw; 0],
                    LoopDetected: [wasmtime::ValRaw; 0],
                    ConfigurationError: [wasmtime::ValRaw; 0],
                    InternalError: T38,
                }
                #[automatically_derived]
                #[allow(non_snake_case)]
                impl<
                    T1: ::core::marker::Copy + ::core::clone::Clone + Copy,
                    T14: ::core::marker::Copy + ::core::clone::Clone + Copy,
                    T17: ::core::marker::Copy + ::core::clone::Clone + Copy,
                    T21: ::core::marker::Copy + ::core::clone::Clone + Copy,
                    T22: ::core::marker::Copy + ::core::clone::Clone + Copy,
                    T23: ::core::marker::Copy + ::core::clone::Clone + Copy,
                    T24: ::core::marker::Copy + ::core::clone::Clone + Copy,
                    T26: ::core::marker::Copy + ::core::clone::Clone + Copy,
                    T27: ::core::marker::Copy + ::core::clone::Clone + Copy,
                    T28: ::core::marker::Copy + ::core::clone::Clone + Copy,
                    T29: ::core::marker::Copy + ::core::clone::Clone + Copy,
                    T30: ::core::marker::Copy + ::core::clone::Clone + Copy,
                    T31: ::core::marker::Copy + ::core::clone::Clone + Copy,
                    T32: ::core::marker::Copy + ::core::clone::Clone + Copy,
                    T38: ::core::marker::Copy + ::core::clone::Clone + Copy,
                > ::core::clone::Clone
                for LowerPayloadErrorCode<
                    T1,
                    T14,
                    T17,
                    T21,
                    T22,
                    T23,
                    T24,
                    T26,
                    T27,
                    T28,
                    T29,
                    T30,
                    T31,
                    T32,
                    T38,
                > {
                    #[inline]
                    fn clone(
                        &self,
                    ) -> LowerPayloadErrorCode<
                        T1,
                        T14,
                        T17,
                        T21,
                        T22,
                        T23,
                        T24,
                        T26,
                        T27,
                        T28,
                        T29,
                        T30,
                        T31,
                        T32,
                        T38,
                    > {
                        let _: ::core::clone::AssertParamIsCopy<Self>;
                        *self
                    }
                }
                #[automatically_derived]
                #[allow(non_snake_case)]
                impl<
                    T1: ::core::marker::Copy + Copy,
                    T14: ::core::marker::Copy + Copy,
                    T17: ::core::marker::Copy + Copy,
                    T21: ::core::marker::Copy + Copy,
                    T22: ::core::marker::Copy + Copy,
                    T23: ::core::marker::Copy + Copy,
                    T24: ::core::marker::Copy + Copy,
                    T26: ::core::marker::Copy + Copy,
                    T27: ::core::marker::Copy + Copy,
                    T28: ::core::marker::Copy + Copy,
                    T29: ::core::marker::Copy + Copy,
                    T30: ::core::marker::Copy + Copy,
                    T31: ::core::marker::Copy + Copy,
                    T32: ::core::marker::Copy + Copy,
                    T38: ::core::marker::Copy + Copy,
                > ::core::marker::Copy
                for LowerPayloadErrorCode<
                    T1,
                    T14,
                    T17,
                    T21,
                    T22,
                    T23,
                    T24,
                    T26,
                    T27,
                    T28,
                    T29,
                    T30,
                    T31,
                    T32,
                    T38,
                > {}
                unsafe impl wasmtime::component::ComponentType for ErrorCode {
                    type Lower = LowerErrorCode<
                        <DnsErrorPayload as wasmtime::component::ComponentType>::Lower,
                        <TlsAlertReceivedPayload as wasmtime::component::ComponentType>::Lower,
                        <Option<u64> as wasmtime::component::ComponentType>::Lower,
                        <Option<u32> as wasmtime::component::ComponentType>::Lower,
                        <Option<
                            FieldSizePayload,
                        > as wasmtime::component::ComponentType>::Lower,
                        <Option<u32> as wasmtime::component::ComponentType>::Lower,
                        <FieldSizePayload as wasmtime::component::ComponentType>::Lower,
                        <Option<u32> as wasmtime::component::ComponentType>::Lower,
                        <FieldSizePayload as wasmtime::component::ComponentType>::Lower,
                        <Option<u64> as wasmtime::component::ComponentType>::Lower,
                        <Option<u32> as wasmtime::component::ComponentType>::Lower,
                        <FieldSizePayload as wasmtime::component::ComponentType>::Lower,
                        <Option<String> as wasmtime::component::ComponentType>::Lower,
                        <Option<String> as wasmtime::component::ComponentType>::Lower,
                        <Option<String> as wasmtime::component::ComponentType>::Lower,
                    >;
                    #[inline]
                    fn typecheck(
                        ty: &wasmtime::component::__internal::InterfaceType,
                        types: &wasmtime::component::__internal::InstanceType<'_>,
                    ) -> wasmtime::component::__internal::anyhow::Result<()> {
                        wasmtime::component::__internal::typecheck_variant(
                            ty,
                            types,
                            &[
                                ("DNS-timeout", None),
                                (
                                    "DNS-error",
                                    Some(
                                        <DnsErrorPayload as wasmtime::component::ComponentType>::typecheck,
                                    ),
                                ),
                                ("destination-not-found", None),
                                ("destination-unavailable", None),
                                ("destination-IP-prohibited", None),
                                ("destination-IP-unroutable", None),
                                ("connection-refused", None),
                                ("connection-terminated", None),
                                ("connection-timeout", None),
                                ("connection-read-timeout", None),
                                ("connection-write-timeout", None),
                                ("connection-limit-reached", None),
                                ("TLS-protocol-error", None),
                                ("TLS-certificate-error", None),
                                (
                                    "TLS-alert-received",
                                    Some(
                                        <TlsAlertReceivedPayload as wasmtime::component::ComponentType>::typecheck,
                                    ),
                                ),
                                ("HTTP-request-denied", None),
                                ("HTTP-request-length-required", None),
                                (
                                    "HTTP-request-body-size",
                                    Some(
                                        <Option<
                                            u64,
                                        > as wasmtime::component::ComponentType>::typecheck,
                                    ),
                                ),
                                ("HTTP-request-method-invalid", None),
                                ("HTTP-request-URI-invalid", None),
                                ("HTTP-request-URI-too-long", None),
                                (
                                    "HTTP-request-header-section-size",
                                    Some(
                                        <Option<
                                            u32,
                                        > as wasmtime::component::ComponentType>::typecheck,
                                    ),
                                ),
                                (
                                    "HTTP-request-header-size",
                                    Some(
                                        <Option<
                                            FieldSizePayload,
                                        > as wasmtime::component::ComponentType>::typecheck,
                                    ),
                                ),
                                (
                                    "HTTP-request-trailer-section-size",
                                    Some(
                                        <Option<
                                            u32,
                                        > as wasmtime::component::ComponentType>::typecheck,
                                    ),
                                ),
                                (
                                    "HTTP-request-trailer-size",
                                    Some(
                                        <FieldSizePayload as wasmtime::component::ComponentType>::typecheck,
                                    ),
                                ),
                                ("HTTP-response-incomplete", None),
                                (
                                    "HTTP-response-header-section-size",
                                    Some(
                                        <Option<
                                            u32,
                                        > as wasmtime::component::ComponentType>::typecheck,
                                    ),
                                ),
                                (
                                    "HTTP-response-header-size",
                                    Some(
                                        <FieldSizePayload as wasmtime::component::ComponentType>::typecheck,
                                    ),
                                ),
                                (
                                    "HTTP-response-body-size",
                                    Some(
                                        <Option<
                                            u64,
                                        > as wasmtime::component::ComponentType>::typecheck,
                                    ),
                                ),
                                (
                                    "HTTP-response-trailer-section-size",
                                    Some(
                                        <Option<
                                            u32,
                                        > as wasmtime::component::ComponentType>::typecheck,
                                    ),
                                ),
                                (
                                    "HTTP-response-trailer-size",
                                    Some(
                                        <FieldSizePayload as wasmtime::component::ComponentType>::typecheck,
                                    ),
                                ),
                                (
                                    "HTTP-response-transfer-coding",
                                    Some(
                                        <Option<
                                            String,
                                        > as wasmtime::component::ComponentType>::typecheck,
                                    ),
                                ),
                                (
                                    "HTTP-response-content-coding",
                                    Some(
                                        <Option<
                                            String,
                                        > as wasmtime::component::ComponentType>::typecheck,
                                    ),
                                ),
                                ("HTTP-response-timeout", None),
                                ("HTTP-upgrade-failed", None),
                                ("HTTP-protocol-error", None),
                                ("loop-detected", None),
                                ("configuration-error", None),
                                (
                                    "internal-error",
                                    Some(
                                        <Option<
                                            String,
                                        > as wasmtime::component::ComponentType>::typecheck,
                                    ),
                                ),
                            ],
                        )
                    }
                    const ABI: wasmtime::component::__internal::CanonicalAbiInfo = wasmtime::component::__internal::CanonicalAbiInfo::variant_static(
                        &[
                            None,
                            Some(
                                <DnsErrorPayload as wasmtime::component::ComponentType>::ABI,
                            ),
                            None,
                            None,
                            None,
                            None,
                            None,
                            None,
                            None,
                            None,
                            None,
                            None,
                            None,
                            None,
                            Some(
                                <TlsAlertReceivedPayload as wasmtime::component::ComponentType>::ABI,
                            ),
                            None,
                            None,
                            Some(
                                <Option<u64> as wasmtime::component::ComponentType>::ABI,
                            ),
                            None,
                            None,
                            None,
                            Some(
                                <Option<u32> as wasmtime::component::ComponentType>::ABI,
                            ),
                            Some(
                                <Option<
                                    FieldSizePayload,
                                > as wasmtime::component::ComponentType>::ABI,
                            ),
                            Some(
                                <Option<u32> as wasmtime::component::ComponentType>::ABI,
                            ),
                            Some(
                                <FieldSizePayload as wasmtime::component::ComponentType>::ABI,
                            ),
                            None,
                            Some(
                                <Option<u32> as wasmtime::component::ComponentType>::ABI,
                            ),
                            Some(
                                <FieldSizePayload as wasmtime::component::ComponentType>::ABI,
                            ),
                            Some(
                                <Option<u64> as wasmtime::component::ComponentType>::ABI,
                            ),
                            Some(
                                <Option<u32> as wasmtime::component::ComponentType>::ABI,
                            ),
                            Some(
                                <FieldSizePayload as wasmtime::component::ComponentType>::ABI,
                            ),
                            Some(
                                <Option<String> as wasmtime::component::ComponentType>::ABI,
                            ),
                            Some(
                                <Option<String> as wasmtime::component::ComponentType>::ABI,
                            ),
                            None,
                            None,
                            None,
                            None,
                            None,
                            Some(
                                <Option<String> as wasmtime::component::ComponentType>::ABI,
                            ),
                        ],
                    );
                }
                unsafe impl wasmtime::component::__internal::ComponentVariant
                for ErrorCode {
                    const CASES: &'static [Option<
                        wasmtime::component::__internal::CanonicalAbiInfo,
                    >] = &[
                        None,
                        Some(
                            <DnsErrorPayload as wasmtime::component::ComponentType>::ABI,
                        ),
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                        Some(
                            <TlsAlertReceivedPayload as wasmtime::component::ComponentType>::ABI,
                        ),
                        None,
                        None,
                        Some(<Option<u64> as wasmtime::component::ComponentType>::ABI),
                        None,
                        None,
                        None,
                        Some(<Option<u32> as wasmtime::component::ComponentType>::ABI),
                        Some(
                            <Option<
                                FieldSizePayload,
                            > as wasmtime::component::ComponentType>::ABI,
                        ),
                        Some(<Option<u32> as wasmtime::component::ComponentType>::ABI),
                        Some(
                            <FieldSizePayload as wasmtime::component::ComponentType>::ABI,
                        ),
                        None,
                        Some(<Option<u32> as wasmtime::component::ComponentType>::ABI),
                        Some(
                            <FieldSizePayload as wasmtime::component::ComponentType>::ABI,
                        ),
                        Some(<Option<u64> as wasmtime::component::ComponentType>::ABI),
                        Some(<Option<u32> as wasmtime::component::ComponentType>::ABI),
                        Some(
                            <FieldSizePayload as wasmtime::component::ComponentType>::ABI,
                        ),
                        Some(
                            <Option<String> as wasmtime::component::ComponentType>::ABI,
                        ),
                        Some(
                            <Option<String> as wasmtime::component::ComponentType>::ABI,
                        ),
                        None,
                        None,
                        None,
                        None,
                        None,
                        Some(<Option<String> as wasmtime::component::ComponentType>::ABI),
                    ];
                }
            };
            impl core::fmt::Debug for ErrorCode {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    match self {
                        ErrorCode::DnsTimeout => {
                            f.debug_tuple("ErrorCode::DnsTimeout").finish()
                        }
                        ErrorCode::DnsError(e) => {
                            f.debug_tuple("ErrorCode::DnsError").field(e).finish()
                        }
                        ErrorCode::DestinationNotFound => {
                            f.debug_tuple("ErrorCode::DestinationNotFound").finish()
                        }
                        ErrorCode::DestinationUnavailable => {
                            f.debug_tuple("ErrorCode::DestinationUnavailable").finish()
                        }
                        ErrorCode::DestinationIpProhibited => {
                            f.debug_tuple("ErrorCode::DestinationIpProhibited").finish()
                        }
                        ErrorCode::DestinationIpUnroutable => {
                            f.debug_tuple("ErrorCode::DestinationIpUnroutable").finish()
                        }
                        ErrorCode::ConnectionRefused => {
                            f.debug_tuple("ErrorCode::ConnectionRefused").finish()
                        }
                        ErrorCode::ConnectionTerminated => {
                            f.debug_tuple("ErrorCode::ConnectionTerminated").finish()
                        }
                        ErrorCode::ConnectionTimeout => {
                            f.debug_tuple("ErrorCode::ConnectionTimeout").finish()
                        }
                        ErrorCode::ConnectionReadTimeout => {
                            f.debug_tuple("ErrorCode::ConnectionReadTimeout").finish()
                        }
                        ErrorCode::ConnectionWriteTimeout => {
                            f.debug_tuple("ErrorCode::ConnectionWriteTimeout").finish()
                        }
                        ErrorCode::ConnectionLimitReached => {
                            f.debug_tuple("ErrorCode::ConnectionLimitReached").finish()
                        }
                        ErrorCode::TlsProtocolError => {
                            f.debug_tuple("ErrorCode::TlsProtocolError").finish()
                        }
                        ErrorCode::TlsCertificateError => {
                            f.debug_tuple("ErrorCode::TlsCertificateError").finish()
                        }
                        ErrorCode::TlsAlertReceived(e) => {
                            f.debug_tuple("ErrorCode::TlsAlertReceived")
                                .field(e)
                                .finish()
                        }
                        ErrorCode::HttpRequestDenied => {
                            f.debug_tuple("ErrorCode::HttpRequestDenied").finish()
                        }
                        ErrorCode::HttpRequestLengthRequired => {
                            f.debug_tuple("ErrorCode::HttpRequestLengthRequired")
                                .finish()
                        }
                        ErrorCode::HttpRequestBodySize(e) => {
                            f.debug_tuple("ErrorCode::HttpRequestBodySize")
                                .field(e)
                                .finish()
                        }
                        ErrorCode::HttpRequestMethodInvalid => {
                            f.debug_tuple("ErrorCode::HttpRequestMethodInvalid").finish()
                        }
                        ErrorCode::HttpRequestUriInvalid => {
                            f.debug_tuple("ErrorCode::HttpRequestUriInvalid").finish()
                        }
                        ErrorCode::HttpRequestUriTooLong => {
                            f.debug_tuple("ErrorCode::HttpRequestUriTooLong").finish()
                        }
                        ErrorCode::HttpRequestHeaderSectionSize(e) => {
                            f.debug_tuple("ErrorCode::HttpRequestHeaderSectionSize")
                                .field(e)
                                .finish()
                        }
                        ErrorCode::HttpRequestHeaderSize(e) => {
                            f.debug_tuple("ErrorCode::HttpRequestHeaderSize")
                                .field(e)
                                .finish()
                        }
                        ErrorCode::HttpRequestTrailerSectionSize(e) => {
                            f.debug_tuple("ErrorCode::HttpRequestTrailerSectionSize")
                                .field(e)
                                .finish()
                        }
                        ErrorCode::HttpRequestTrailerSize(e) => {
                            f.debug_tuple("ErrorCode::HttpRequestTrailerSize")
                                .field(e)
                                .finish()
                        }
                        ErrorCode::HttpResponseIncomplete => {
                            f.debug_tuple("ErrorCode::HttpResponseIncomplete").finish()
                        }
                        ErrorCode::HttpResponseHeaderSectionSize(e) => {
                            f.debug_tuple("ErrorCode::HttpResponseHeaderSectionSize")
                                .field(e)
                                .finish()
                        }
                        ErrorCode::HttpResponseHeaderSize(e) => {
                            f.debug_tuple("ErrorCode::HttpResponseHeaderSize")
                                .field(e)
                                .finish()
                        }
                        ErrorCode::HttpResponseBodySize(e) => {
                            f.debug_tuple("ErrorCode::HttpResponseBodySize")
                                .field(e)
                                .finish()
                        }
                        ErrorCode::HttpResponseTrailerSectionSize(e) => {
                            f.debug_tuple("ErrorCode::HttpResponseTrailerSectionSize")
                                .field(e)
                                .finish()
                        }
                        ErrorCode::HttpResponseTrailerSize(e) => {
                            f.debug_tuple("ErrorCode::HttpResponseTrailerSize")
                                .field(e)
                                .finish()
                        }
                        ErrorCode::HttpResponseTransferCoding(e) => {
                            f.debug_tuple("ErrorCode::HttpResponseTransferCoding")
                                .field(e)
                                .finish()
                        }
                        ErrorCode::HttpResponseContentCoding(e) => {
                            f.debug_tuple("ErrorCode::HttpResponseContentCoding")
                                .field(e)
                                .finish()
                        }
                        ErrorCode::HttpResponseTimeout => {
                            f.debug_tuple("ErrorCode::HttpResponseTimeout").finish()
                        }
                        ErrorCode::HttpUpgradeFailed => {
                            f.debug_tuple("ErrorCode::HttpUpgradeFailed").finish()
                        }
                        ErrorCode::HttpProtocolError => {
                            f.debug_tuple("ErrorCode::HttpProtocolError").finish()
                        }
                        ErrorCode::LoopDetected => {
                            f.debug_tuple("ErrorCode::LoopDetected").finish()
                        }
                        ErrorCode::ConfigurationError => {
                            f.debug_tuple("ErrorCode::ConfigurationError").finish()
                        }
                        ErrorCode::InternalError(e) => {
                            f.debug_tuple("ErrorCode::InternalError").field(e).finish()
                        }
                    }
                }
            }
            impl core::fmt::Display for ErrorCode {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    f.write_fmt(format_args!("{0:?}", self))
                }
            }
            impl std::error::Error for ErrorCode {}
            const _: () = {
                if !(32 == <ErrorCode as wasmtime::component::ComponentType>::SIZE32) {
                    ::core::panicking::panic(
                        "assertion failed: 32 == <ErrorCode as wasmtime::component::ComponentType>::SIZE32",
                    )
                }
                if !(8 == <ErrorCode as wasmtime::component::ComponentType>::ALIGN32) {
                    ::core::panicking::panic(
                        "assertion failed: 8 == <ErrorCode as wasmtime::component::ComponentType>::ALIGN32",
                    )
                }
            };
            /// This type enumerates the different kinds of errors that may occur when
            /// setting or appending to a `fields` resource.
            #[component(variant)]
            pub enum HeaderError {
                /// This error indicates that a `field-key` or `field-value` was
                /// syntactically invalid when used with an operation that sets headers in a
                /// `fields`.
                #[component(name = "invalid-syntax")]
                InvalidSyntax,
                /// This error indicates that a forbidden `field-key` was used when trying
                /// to set a header in a `fields`.
                #[component(name = "forbidden")]
                Forbidden,
                /// This error indicates that the operation on the `fields` was not
                /// permitted because the fields are immutable.
                #[component(name = "immutable")]
                Immutable,
            }
            #[automatically_derived]
            impl ::core::marker::Copy for HeaderError {}
            #[automatically_derived]
            impl ::core::clone::Clone for HeaderError {
                #[inline]
                fn clone(&self) -> HeaderError {
                    *self
                }
            }
            unsafe impl wasmtime::component::Lower for HeaderError {
                #[inline]
                fn lower<T>(
                    &self,
                    cx: &mut wasmtime::component::__internal::LowerContext<'_, T>,
                    ty: wasmtime::component::__internal::InterfaceType,
                    dst: &mut std::mem::MaybeUninit<Self::Lower>,
                ) -> wasmtime::component::__internal::anyhow::Result<()> {
                    let ty = match ty {
                        wasmtime::component::__internal::InterfaceType::Variant(i) => {
                            &cx.types[i]
                        }
                        _ => wasmtime::component::__internal::bad_type_info(),
                    };
                    match self {
                        Self::InvalidSyntax => {
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
                                                m.map(|p| &raw mut (*p).InvalidSyntax)
                                            }
                                        }
                                    },
                                    |dst| Ok(()),
                                )
                            }
                        }
                        Self::Forbidden => {
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
                                                m.map(|p| &raw mut (*p).Forbidden)
                                            }
                                        }
                                    },
                                    |dst| Ok(()),
                                )
                            }
                        }
                        Self::Immutable => {
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
                                .write(wasmtime::ValRaw::u32(2u32));
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
                                                m.map(|p| &raw mut (*p).Immutable)
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
                        wasmtime::component::__internal::InterfaceType::Variant(i) => {
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
                        Self::InvalidSyntax => {
                            *cx.get::<1usize>(offset) = 0u8.to_le_bytes();
                            Ok(())
                        }
                        Self::Forbidden => {
                            *cx.get::<1usize>(offset) = 1u8.to_le_bytes();
                            Ok(())
                        }
                        Self::Immutable => {
                            *cx.get::<1usize>(offset) = 2u8.to_le_bytes();
                            Ok(())
                        }
                    }
                }
            }
            unsafe impl wasmtime::component::Lift for HeaderError {
                #[inline]
                fn lift(
                    cx: &mut wasmtime::component::__internal::LiftContext<'_>,
                    ty: wasmtime::component::__internal::InterfaceType,
                    src: &Self::Lower,
                ) -> wasmtime::component::__internal::anyhow::Result<Self> {
                    let ty = match ty {
                        wasmtime::component::__internal::InterfaceType::Variant(i) => {
                            &cx.types[i]
                        }
                        _ => wasmtime::component::__internal::bad_type_info(),
                    };
                    Ok(
                        match src.tag.get_u32() {
                            0u32 => Self::InvalidSyntax,
                            1u32 => Self::Forbidden,
                            2u32 => Self::Immutable,
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
                        wasmtime::component::__internal::InterfaceType::Variant(i) => {
                            &cx.types[i]
                        }
                        _ => wasmtime::component::__internal::bad_type_info(),
                    };
                    Ok(
                        match discrim {
                            0u8 => Self::InvalidSyntax,
                            1u8 => Self::Forbidden,
                            2u8 => Self::Immutable,
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
                pub struct LowerHeaderError {
                    tag: wasmtime::ValRaw,
                    payload: LowerPayloadHeaderError,
                }
                #[automatically_derived]
                impl ::core::clone::Clone for LowerHeaderError {
                    #[inline]
                    fn clone(&self) -> LowerHeaderError {
                        let _: ::core::clone::AssertParamIsClone<wasmtime::ValRaw>;
                        let _: ::core::clone::AssertParamIsClone<
                            LowerPayloadHeaderError,
                        >;
                        *self
                    }
                }
                #[automatically_derived]
                impl ::core::marker::Copy for LowerHeaderError {}
                #[doc(hidden)]
                #[allow(non_snake_case)]
                #[repr(C)]
                union LowerPayloadHeaderError {
                    InvalidSyntax: [wasmtime::ValRaw; 0],
                    Forbidden: [wasmtime::ValRaw; 0],
                    Immutable: [wasmtime::ValRaw; 0],
                }
                #[automatically_derived]
                #[allow(non_snake_case)]
                impl ::core::clone::Clone for LowerPayloadHeaderError {
                    #[inline]
                    fn clone(&self) -> LowerPayloadHeaderError {
                        let _: ::core::clone::AssertParamIsCopy<Self>;
                        *self
                    }
                }
                #[automatically_derived]
                #[allow(non_snake_case)]
                impl ::core::marker::Copy for LowerPayloadHeaderError {}
                unsafe impl wasmtime::component::ComponentType for HeaderError {
                    type Lower = LowerHeaderError;
                    #[inline]
                    fn typecheck(
                        ty: &wasmtime::component::__internal::InterfaceType,
                        types: &wasmtime::component::__internal::InstanceType<'_>,
                    ) -> wasmtime::component::__internal::anyhow::Result<()> {
                        wasmtime::component::__internal::typecheck_variant(
                            ty,
                            types,
                            &[
                                ("invalid-syntax", None),
                                ("forbidden", None),
                                ("immutable", None),
                            ],
                        )
                    }
                    const ABI: wasmtime::component::__internal::CanonicalAbiInfo = wasmtime::component::__internal::CanonicalAbiInfo::variant_static(
                        &[None, None, None],
                    );
                }
                unsafe impl wasmtime::component::__internal::ComponentVariant
                for HeaderError {
                    const CASES: &'static [Option<
                        wasmtime::component::__internal::CanonicalAbiInfo,
                    >] = &[None, None, None];
                }
            };
            impl core::fmt::Debug for HeaderError {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    match self {
                        HeaderError::InvalidSyntax => {
                            f.debug_tuple("HeaderError::InvalidSyntax").finish()
                        }
                        HeaderError::Forbidden => {
                            f.debug_tuple("HeaderError::Forbidden").finish()
                        }
                        HeaderError::Immutable => {
                            f.debug_tuple("HeaderError::Immutable").finish()
                        }
                    }
                }
            }
            impl core::fmt::Display for HeaderError {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    f.write_fmt(format_args!("{0:?}", self))
                }
            }
            impl std::error::Error for HeaderError {}
            const _: () = {
                if !(1 == <HeaderError as wasmtime::component::ComponentType>::SIZE32) {
                    ::core::panicking::panic(
                        "assertion failed: 1 == <HeaderError as wasmtime::component::ComponentType>::SIZE32",
                    )
                }
                if !(1 == <HeaderError as wasmtime::component::ComponentType>::ALIGN32) {
                    ::core::panicking::panic(
                        "assertion failed: 1 == <HeaderError as wasmtime::component::ComponentType>::ALIGN32",
                    )
                }
            };
            /// Field keys are always strings.
            pub type FieldKey = String;
            const _: () = {
                if !(8 == <FieldKey as wasmtime::component::ComponentType>::SIZE32) {
                    ::core::panicking::panic(
                        "assertion failed: 8 == <FieldKey as wasmtime::component::ComponentType>::SIZE32",
                    )
                }
                if !(4 == <FieldKey as wasmtime::component::ComponentType>::ALIGN32) {
                    ::core::panicking::panic(
                        "assertion failed: 4 == <FieldKey as wasmtime::component::ComponentType>::ALIGN32",
                    )
                }
            };
            /// Field values should always be ASCII strings. However, in
            /// reality, HTTP implementations often have to interpret malformed values,
            /// so they are provided as a list of bytes.
            pub type FieldValue = Vec<u8>;
            const _: () = {
                if !(8 == <FieldValue as wasmtime::component::ComponentType>::SIZE32) {
                    ::core::panicking::panic(
                        "assertion failed: 8 == <FieldValue as wasmtime::component::ComponentType>::SIZE32",
                    )
                }
                if !(4 == <FieldValue as wasmtime::component::ComponentType>::ALIGN32) {
                    ::core::panicking::panic(
                        "assertion failed: 4 == <FieldValue as wasmtime::component::ComponentType>::ALIGN32",
                    )
                }
            };
            /// This following block defines the `fields` resource which corresponds to
            /// HTTP standard Fields. Fields are a common representation used for both
            /// Headers and Trailers.
            ///
            /// A `fields` may be mutable or immutable. A `fields` created using the
            /// constructor, `from-list`, or `clone` will be mutable, but a `fields`
            /// resource given by other means (including, but not limited to,
            /// `incoming-request.headers`, `outgoing-request.headers`) might be be
            /// immutable. In an immutable fields, the `set`, `append`, and `delete`
            /// operations will fail with `header-error.immutable`.
            pub enum Fields {}
            pub trait HostFields {
                /// Construct an empty HTTP Fields.
                ///
                /// The resulting `fields` is mutable.
                fn new(
                    &mut self,
                ) -> wasmtime::Result<wasmtime::component::Resource<Fields>>;
                /// Construct an HTTP Fields.
                ///
                /// The resulting `fields` is mutable.
                ///
                /// The list represents each key-value pair in the Fields. Keys
                /// which have multiple values are represented by multiple entries in this
                /// list with the same key.
                ///
                /// The tuple is a pair of the field key, represented as a string, and
                /// Value, represented as a list of bytes.
                ///
                /// An error result will be returned if any `field-key` or `field-value` is
                /// syntactically invalid, or if a field is forbidden.
                fn from_list(
                    &mut self,
                    entries: Vec<(FieldKey, FieldValue)>,
                ) -> wasmtime::Result<
                    Result<wasmtime::component::Resource<Fields>, HeaderError>,
                >;
                /// Get all of the values corresponding to a key. If the key is not present
                /// in this `fields` or is syntactically invalid, an empty list is returned.
                /// However, if the key is present but empty, this is represented by a list
                /// with one or more empty field-values present.
                fn get(
                    &mut self,
                    self_: wasmtime::component::Resource<Fields>,
                    name: FieldKey,
                ) -> wasmtime::Result<Vec<FieldValue>>;
                /// Returns `true` when the key is present in this `fields`. If the key is
                /// syntactically invalid, `false` is returned.
                fn has(
                    &mut self,
                    self_: wasmtime::component::Resource<Fields>,
                    name: FieldKey,
                ) -> wasmtime::Result<bool>;
                /// Set all of the values for a key. Clears any existing values for that
                /// key, if they have been set.
                ///
                /// Fails with `header-error.immutable` if the `fields` are immutable.
                ///
                /// Fails with `header-error.invalid-syntax` if the `field-key` or any of
                /// the `field-value`s are syntactically invalid.
                fn set(
                    &mut self,
                    self_: wasmtime::component::Resource<Fields>,
                    name: FieldKey,
                    value: Vec<FieldValue>,
                ) -> wasmtime::Result<Result<(), HeaderError>>;
                /// Delete all values for a key. Does nothing if no values for the key
                /// exist.
                ///
                /// Fails with `header-error.immutable` if the `fields` are immutable.
                ///
                /// Fails with `header-error.invalid-syntax` if the `field-key` is
                /// syntactically invalid.
                fn delete(
                    &mut self,
                    self_: wasmtime::component::Resource<Fields>,
                    name: FieldKey,
                ) -> wasmtime::Result<Result<(), HeaderError>>;
                /// Append a value for a key. Does not change or delete any existing
                /// values for that key.
                ///
                /// Fails with `header-error.immutable` if the `fields` are immutable.
                ///
                /// Fails with `header-error.invalid-syntax` if the `field-key` or
                /// `field-value` are syntactically invalid.
                fn append(
                    &mut self,
                    self_: wasmtime::component::Resource<Fields>,
                    name: FieldKey,
                    value: FieldValue,
                ) -> wasmtime::Result<Result<(), HeaderError>>;
                /// Retrieve the full set of keys and values in the Fields. Like the
                /// constructor, the list represents each key-value pair.
                ///
                /// The outer list represents each key-value pair in the Fields. Keys
                /// which have multiple values are represented by multiple entries in this
                /// list with the same key.
                fn entries(
                    &mut self,
                    self_: wasmtime::component::Resource<Fields>,
                ) -> wasmtime::Result<Vec<(FieldKey, FieldValue)>>;
                /// Make a deep copy of the Fields. Equivelant in behavior to calling the
                /// `fields` constructor on the return value of `entries`. The resulting
                /// `fields` is mutable.
                fn clone(
                    &mut self,
                    self_: wasmtime::component::Resource<Fields>,
                ) -> wasmtime::Result<wasmtime::component::Resource<Fields>>;
                fn drop(
                    &mut self,
                    rep: wasmtime::component::Resource<Fields>,
                ) -> wasmtime::Result<()>;
            }
            /// Headers is an alias for Fields.
            pub type Headers = Fields;
            /// Trailers is an alias for Fields.
            pub type Trailers = Fields;
            /// Represents an incoming HTTP Request.
            pub enum IncomingRequest {}
            pub trait HostIncomingRequest {
                /// Returns the method of the incoming request.
                fn method(
                    &mut self,
                    self_: wasmtime::component::Resource<IncomingRequest>,
                ) -> wasmtime::Result<Method>;
                /// Returns the path with query parameters from the request, as a string.
                fn path_with_query(
                    &mut self,
                    self_: wasmtime::component::Resource<IncomingRequest>,
                ) -> wasmtime::Result<Option<String>>;
                /// Returns the protocol scheme from the request.
                fn scheme(
                    &mut self,
                    self_: wasmtime::component::Resource<IncomingRequest>,
                ) -> wasmtime::Result<Option<Scheme>>;
                /// Returns the authority from the request, if it was present.
                fn authority(
                    &mut self,
                    self_: wasmtime::component::Resource<IncomingRequest>,
                ) -> wasmtime::Result<Option<String>>;
                /// Get the `headers` associated with the request.
                ///
                /// The returned `headers` resource is immutable: `set`, `append`, and
                /// `delete` operations will fail with `header-error.immutable`.
                ///
                /// The `headers` returned are a child resource: it must be dropped before
                /// the parent `incoming-request` is dropped. Dropping this
                /// `incoming-request` before all children are dropped will trap.
                fn headers(
                    &mut self,
                    self_: wasmtime::component::Resource<IncomingRequest>,
                ) -> wasmtime::Result<wasmtime::component::Resource<Headers>>;
                /// Gives the `incoming-body` associated with this request. Will only
                /// return success at most once, and subsequent calls will return error.
                fn consume(
                    &mut self,
                    self_: wasmtime::component::Resource<IncomingRequest>,
                ) -> wasmtime::Result<
                    Result<wasmtime::component::Resource<IncomingBody>, ()>,
                >;
                fn drop(
                    &mut self,
                    rep: wasmtime::component::Resource<IncomingRequest>,
                ) -> wasmtime::Result<()>;
            }
            /// Represents an outgoing HTTP Request.
            pub enum OutgoingRequest {}
            pub trait HostOutgoingRequest {
                /// Construct a new `outgoing-request` with a default `method` of `GET`, and
                /// `none` values for `path-with-query`, `scheme`, and `authority`.
                ///
                /// * `headers` is the HTTP Headers for the Request.
                ///
                /// It is possible to construct, or manipulate with the accessor functions
                /// below, an `outgoing-request` with an invalid combination of `scheme`
                /// and `authority`, or `headers` which are not permitted to be sent.
                /// It is the obligation of the `outgoing-handler.handle` implementation
                /// to reject invalid constructions of `outgoing-request`.
                fn new(
                    &mut self,
                    headers: wasmtime::component::Resource<Headers>,
                ) -> wasmtime::Result<wasmtime::component::Resource<OutgoingRequest>>;
                /// Returns the resource corresponding to the outgoing Body for this
                /// Request.
                ///
                /// Returns success on the first call: the `outgoing-body` resource for
                /// this `outgoing-request` can be retrieved at most once. Subsequent
                /// calls will return error.
                fn body(
                    &mut self,
                    self_: wasmtime::component::Resource<OutgoingRequest>,
                ) -> wasmtime::Result<
                    Result<wasmtime::component::Resource<OutgoingBody>, ()>,
                >;
                /// Get the Method for the Request.
                fn method(
                    &mut self,
                    self_: wasmtime::component::Resource<OutgoingRequest>,
                ) -> wasmtime::Result<Method>;
                /// Set the Method for the Request. Fails if the string present in a
                /// `method.other` argument is not a syntactically valid method.
                fn set_method(
                    &mut self,
                    self_: wasmtime::component::Resource<OutgoingRequest>,
                    method: Method,
                ) -> wasmtime::Result<Result<(), ()>>;
                /// Get the combination of the HTTP Path and Query for the Request.
                /// When `none`, this represents an empty Path and empty Query.
                fn path_with_query(
                    &mut self,
                    self_: wasmtime::component::Resource<OutgoingRequest>,
                ) -> wasmtime::Result<Option<String>>;
                /// Set the combination of the HTTP Path and Query for the Request.
                /// When `none`, this represents an empty Path and empty Query. Fails is the
                /// string given is not a syntactically valid path and query uri component.
                fn set_path_with_query(
                    &mut self,
                    self_: wasmtime::component::Resource<OutgoingRequest>,
                    path_with_query: Option<String>,
                ) -> wasmtime::Result<Result<(), ()>>;
                /// Get the HTTP Related Scheme for the Request. When `none`, the
                /// implementation may choose an appropriate default scheme.
                fn scheme(
                    &mut self,
                    self_: wasmtime::component::Resource<OutgoingRequest>,
                ) -> wasmtime::Result<Option<Scheme>>;
                /// Set the HTTP Related Scheme for the Request. When `none`, the
                /// implementation may choose an appropriate default scheme. Fails if the
                /// string given is not a syntactically valid uri scheme.
                fn set_scheme(
                    &mut self,
                    self_: wasmtime::component::Resource<OutgoingRequest>,
                    scheme: Option<Scheme>,
                ) -> wasmtime::Result<Result<(), ()>>;
                /// Get the HTTP Authority for the Request. A value of `none` may be used
                /// with Related Schemes which do not require an Authority. The HTTP and
                /// HTTPS schemes always require an authority.
                fn authority(
                    &mut self,
                    self_: wasmtime::component::Resource<OutgoingRequest>,
                ) -> wasmtime::Result<Option<String>>;
                /// Set the HTTP Authority for the Request. A value of `none` may be used
                /// with Related Schemes which do not require an Authority. The HTTP and
                /// HTTPS schemes always require an authority. Fails if the string given is
                /// not a syntactically valid uri authority.
                fn set_authority(
                    &mut self,
                    self_: wasmtime::component::Resource<OutgoingRequest>,
                    authority: Option<String>,
                ) -> wasmtime::Result<Result<(), ()>>;
                /// Get the headers associated with the Request.
                ///
                /// The returned `headers` resource is immutable: `set`, `append`, and
                /// `delete` operations will fail with `header-error.immutable`.
                ///
                /// This headers resource is a child: it must be dropped before the parent
                /// `outgoing-request` is dropped, or its ownership is transfered to
                /// another component by e.g. `outgoing-handler.handle`.
                fn headers(
                    &mut self,
                    self_: wasmtime::component::Resource<OutgoingRequest>,
                ) -> wasmtime::Result<wasmtime::component::Resource<Headers>>;
                fn drop(
                    &mut self,
                    rep: wasmtime::component::Resource<OutgoingRequest>,
                ) -> wasmtime::Result<()>;
            }
            /// Parameters for making an HTTP Request. Each of these parameters is
            /// currently an optional timeout applicable to the transport layer of the
            /// HTTP protocol.
            ///
            /// These timeouts are separate from any the user may use to bound a
            /// blocking call to `wasi:io/poll.poll`.
            pub enum RequestOptions {}
            pub trait HostRequestOptions {
                /// Construct a default `request-options` value.
                fn new(
                    &mut self,
                ) -> wasmtime::Result<wasmtime::component::Resource<RequestOptions>>;
                /// The timeout for the initial connect to the HTTP Server.
                fn connect_timeout(
                    &mut self,
                    self_: wasmtime::component::Resource<RequestOptions>,
                ) -> wasmtime::Result<Option<Duration>>;
                /// Set the timeout for the initial connect to the HTTP Server. An error
                /// return value indicates that this timeout is not supported.
                fn set_connect_timeout(
                    &mut self,
                    self_: wasmtime::component::Resource<RequestOptions>,
                    duration: Option<Duration>,
                ) -> wasmtime::Result<Result<(), ()>>;
                /// The timeout for receiving the first byte of the Response body.
                fn first_byte_timeout(
                    &mut self,
                    self_: wasmtime::component::Resource<RequestOptions>,
                ) -> wasmtime::Result<Option<Duration>>;
                /// Set the timeout for receiving the first byte of the Response body. An
                /// error return value indicates that this timeout is not supported.
                fn set_first_byte_timeout(
                    &mut self,
                    self_: wasmtime::component::Resource<RequestOptions>,
                    duration: Option<Duration>,
                ) -> wasmtime::Result<Result<(), ()>>;
                /// The timeout for receiving subsequent chunks of bytes in the Response
                /// body stream.
                fn between_bytes_timeout(
                    &mut self,
                    self_: wasmtime::component::Resource<RequestOptions>,
                ) -> wasmtime::Result<Option<Duration>>;
                /// Set the timeout for receiving subsequent chunks of bytes in the Response
                /// body stream. An error return value indicates that this timeout is not
                /// supported.
                fn set_between_bytes_timeout(
                    &mut self,
                    self_: wasmtime::component::Resource<RequestOptions>,
                    duration: Option<Duration>,
                ) -> wasmtime::Result<Result<(), ()>>;
                fn drop(
                    &mut self,
                    rep: wasmtime::component::Resource<RequestOptions>,
                ) -> wasmtime::Result<()>;
            }
            /// Represents the ability to send an HTTP Response.
            ///
            /// This resource is used by the `wasi:http/incoming-handler` interface to
            /// allow a Response to be sent corresponding to the Request provided as the
            /// other argument to `incoming-handler.handle`.
            pub enum ResponseOutparam {}
            pub trait HostResponseOutparam {
                /// Set the value of the `response-outparam` to either send a response,
                /// or indicate an error.
                ///
                /// This method consumes the `response-outparam` to ensure that it is
                /// called at most once. If it is never called, the implementation
                /// will respond with an error.
                ///
                /// The user may provide an `error` to `response` to allow the
                /// implementation determine how to respond with an HTTP error response.
                fn set(
                    &mut self,
                    param: wasmtime::component::Resource<ResponseOutparam>,
                    response: Result<
                        wasmtime::component::Resource<OutgoingResponse>,
                        ErrorCode,
                    >,
                ) -> wasmtime::Result<()>;
                fn drop(
                    &mut self,
                    rep: wasmtime::component::Resource<ResponseOutparam>,
                ) -> wasmtime::Result<()>;
            }
            /// This type corresponds to the HTTP standard Status Code.
            pub type StatusCode = u16;
            const _: () = {
                if !(2 == <StatusCode as wasmtime::component::ComponentType>::SIZE32) {
                    ::core::panicking::panic(
                        "assertion failed: 2 == <StatusCode as wasmtime::component::ComponentType>::SIZE32",
                    )
                }
                if !(2 == <StatusCode as wasmtime::component::ComponentType>::ALIGN32) {
                    ::core::panicking::panic(
                        "assertion failed: 2 == <StatusCode as wasmtime::component::ComponentType>::ALIGN32",
                    )
                }
            };
            /// Represents an incoming HTTP Response.
            pub enum IncomingResponse {}
            pub trait HostIncomingResponse {
                /// Returns the status code from the incoming response.
                fn status(
                    &mut self,
                    self_: wasmtime::component::Resource<IncomingResponse>,
                ) -> wasmtime::Result<StatusCode>;
                /// Returns the headers from the incoming response.
                ///
                /// The returned `headers` resource is immutable: `set`, `append`, and
                /// `delete` operations will fail with `header-error.immutable`.
                ///
                /// This headers resource is a child: it must be dropped before the parent
                /// `incoming-response` is dropped.
                fn headers(
                    &mut self,
                    self_: wasmtime::component::Resource<IncomingResponse>,
                ) -> wasmtime::Result<wasmtime::component::Resource<Headers>>;
                /// Returns the incoming body. May be called at most once. Returns error
                /// if called additional times.
                fn consume(
                    &mut self,
                    self_: wasmtime::component::Resource<IncomingResponse>,
                ) -> wasmtime::Result<
                    Result<wasmtime::component::Resource<IncomingBody>, ()>,
                >;
                fn drop(
                    &mut self,
                    rep: wasmtime::component::Resource<IncomingResponse>,
                ) -> wasmtime::Result<()>;
            }
            /// Represents an incoming HTTP Request or Response's Body.
            ///
            /// A body has both its contents - a stream of bytes - and a (possibly
            /// empty) set of trailers, indicating that the full contents of the
            /// body have been received. This resource represents the contents as
            /// an `input-stream` and the delivery of trailers as a `future-trailers`,
            /// and ensures that the user of this interface may only be consuming either
            /// the body contents or waiting on trailers at any given time.
            pub enum IncomingBody {}
            pub trait HostIncomingBody {
                /// Returns the contents of the body, as a stream of bytes.
                ///
                /// Returns success on first call: the stream representing the contents
                /// can be retrieved at most once. Subsequent calls will return error.
                ///
                /// The returned `input-stream` resource is a child: it must be dropped
                /// before the parent `incoming-body` is dropped, or consumed by
                /// `incoming-body.finish`.
                ///
                /// This invariant ensures that the implementation can determine whether
                /// the user is consuming the contents of the body, waiting on the
                /// `future-trailers` to be ready, or neither. This allows for network
                /// backpressure is to be applied when the user is consuming the body,
                /// and for that backpressure to not inhibit delivery of the trailers if
                /// the user does not read the entire body.
                fn stream(
                    &mut self,
                    self_: wasmtime::component::Resource<IncomingBody>,
                ) -> wasmtime::Result<
                    Result<wasmtime::component::Resource<InputStream>, ()>,
                >;
                /// Takes ownership of `incoming-body`, and returns a `future-trailers`.
                /// This function will trap if the `input-stream` child is still alive.
                fn finish(
                    &mut self,
                    this: wasmtime::component::Resource<IncomingBody>,
                ) -> wasmtime::Result<wasmtime::component::Resource<FutureTrailers>>;
                fn drop(
                    &mut self,
                    rep: wasmtime::component::Resource<IncomingBody>,
                ) -> wasmtime::Result<()>;
            }
            /// Represents a future which may eventaully return trailers, or an error.
            ///
            /// In the case that the incoming HTTP Request or Response did not have any
            /// trailers, this future will resolve to the empty set of trailers once the
            /// complete Request or Response body has been received.
            pub enum FutureTrailers {}
            pub trait HostFutureTrailers {
                /// Returns a pollable which becomes ready when either the trailers have
                /// been received, or an error has occured. When this pollable is ready,
                /// the `get` method will return `some`.
                fn subscribe(
                    &mut self,
                    self_: wasmtime::component::Resource<FutureTrailers>,
                ) -> wasmtime::Result<wasmtime::component::Resource<Pollable>>;
                /// Returns the contents of the trailers, or an error which occured,
                /// once the future is ready.
                ///
                /// The outer `option` represents future readiness. Users can wait on this
                /// `option` to become `some` using the `subscribe` method.
                ///
                /// The outer `result` is used to retrieve the trailers or error at most
                /// once. It will be success on the first call in which the outer option
                /// is `some`, and error on subsequent calls.
                ///
                /// The inner `result` represents that either the HTTP Request or Response
                /// body, as well as any trailers, were received successfully, or that an
                /// error occured receiving them. The optional `trailers` indicates whether
                /// or not trailers were present in the body.
                ///
                /// When some `trailers` are returned by this method, the `trailers`
                /// resource is immutable, and a child. Use of the `set`, `append`, or
                /// `delete` methods will return an error, and the resource must be
                /// dropped before the parent `future-trailers` is dropped.
                fn get(
                    &mut self,
                    self_: wasmtime::component::Resource<FutureTrailers>,
                ) -> wasmtime::Result<
                    Option<
                        Result<
                            Result<
                                Option<wasmtime::component::Resource<Trailers>>,
                                ErrorCode,
                            >,
                            (),
                        >,
                    >,
                >;
                fn drop(
                    &mut self,
                    rep: wasmtime::component::Resource<FutureTrailers>,
                ) -> wasmtime::Result<()>;
            }
            /// Represents an outgoing HTTP Response.
            pub enum OutgoingResponse {}
            pub trait HostOutgoingResponse {
                /// Construct an `outgoing-response`, with a default `status-code` of `200`.
                /// If a different `status-code` is needed, it must be set via the
                /// `set-status-code` method.
                ///
                /// * `headers` is the HTTP Headers for the Response.
                fn new(
                    &mut self,
                    headers: wasmtime::component::Resource<Headers>,
                ) -> wasmtime::Result<wasmtime::component::Resource<OutgoingResponse>>;
                /// Get the HTTP Status Code for the Response.
                fn status_code(
                    &mut self,
                    self_: wasmtime::component::Resource<OutgoingResponse>,
                ) -> wasmtime::Result<StatusCode>;
                /// Set the HTTP Status Code for the Response. Fails if the status-code
                /// given is not a valid http status code.
                fn set_status_code(
                    &mut self,
                    self_: wasmtime::component::Resource<OutgoingResponse>,
                    status_code: StatusCode,
                ) -> wasmtime::Result<Result<(), ()>>;
                /// Get the headers associated with the Request.
                ///
                /// The returned `headers` resource is immutable: `set`, `append`, and
                /// `delete` operations will fail with `header-error.immutable`.
                ///
                /// This headers resource is a child: it must be dropped before the parent
                /// `outgoing-request` is dropped, or its ownership is transfered to
                /// another component by e.g. `outgoing-handler.handle`.
                fn headers(
                    &mut self,
                    self_: wasmtime::component::Resource<OutgoingResponse>,
                ) -> wasmtime::Result<wasmtime::component::Resource<Headers>>;
                /// Returns the resource corresponding to the outgoing Body for this Response.
                ///
                /// Returns success on the first call: the `outgoing-body` resource for
                /// this `outgoing-response` can be retrieved at most once. Subsequent
                /// calls will return error.
                fn body(
                    &mut self,
                    self_: wasmtime::component::Resource<OutgoingResponse>,
                ) -> wasmtime::Result<
                    Result<wasmtime::component::Resource<OutgoingBody>, ()>,
                >;
                fn drop(
                    &mut self,
                    rep: wasmtime::component::Resource<OutgoingResponse>,
                ) -> wasmtime::Result<()>;
            }
            /// Represents an outgoing HTTP Request or Response's Body.
            ///
            /// A body has both its contents - a stream of bytes - and a (possibly
            /// empty) set of trailers, inducating the full contents of the body
            /// have been sent. This resource represents the contents as an
            /// `output-stream` child resource, and the completion of the body (with
            /// optional trailers) with a static function that consumes the
            /// `outgoing-body` resource, and ensures that the user of this interface
            /// may not write to the body contents after the body has been finished.
            ///
            /// If the user code drops this resource, as opposed to calling the static
            /// method `finish`, the implementation should treat the body as incomplete,
            /// and that an error has occured. The implementation should propogate this
            /// error to the HTTP protocol by whatever means it has available,
            /// including: corrupting the body on the wire, aborting the associated
            /// Request, or sending a late status code for the Response.
            pub enum OutgoingBody {}
            pub trait HostOutgoingBody {
                /// Returns a stream for writing the body contents.
                ///
                /// The returned `output-stream` is a child resource: it must be dropped
                /// before the parent `outgoing-body` resource is dropped (or finished),
                /// otherwise the `outgoing-body` drop or `finish` will trap.
                ///
                /// Returns success on the first call: the `output-stream` resource for
                /// this `outgoing-body` may be retrieved at most once. Subsequent calls
                /// will return error.
                fn write(
                    &mut self,
                    self_: wasmtime::component::Resource<OutgoingBody>,
                ) -> wasmtime::Result<
                    Result<wasmtime::component::Resource<OutputStream>, ()>,
                >;
                /// Finalize an outgoing body, optionally providing trailers. This must be
                /// called to signal that the response is complete. If the `outgoing-body`
                /// is dropped without calling `outgoing-body.finalize`, the implementation
                /// should treat the body as corrupted.
                ///
                /// Fails if the body's `outgoing-request` or `outgoing-response` was
                /// constructed with a Content-Length header, and the contents written
                /// to the body (via `write`) does not match the value given in the
                /// Content-Length.
                fn finish(
                    &mut self,
                    this: wasmtime::component::Resource<OutgoingBody>,
                    trailers: Option<wasmtime::component::Resource<Trailers>>,
                ) -> wasmtime::Result<Result<(), ErrorCode>>;
                fn drop(
                    &mut self,
                    rep: wasmtime::component::Resource<OutgoingBody>,
                ) -> wasmtime::Result<()>;
            }
            /// Represents a future which may eventaully return an incoming HTTP
            /// Response, or an error.
            ///
            /// This resource is returned by the `wasi:http/outgoing-handler` interface to
            /// provide the HTTP Response corresponding to the sent Request.
            pub enum FutureIncomingResponse {}
            pub trait HostFutureIncomingResponse {
                /// Returns a pollable which becomes ready when either the Response has
                /// been received, or an error has occured. When this pollable is ready,
                /// the `get` method will return `some`.
                fn subscribe(
                    &mut self,
                    self_: wasmtime::component::Resource<FutureIncomingResponse>,
                ) -> wasmtime::Result<wasmtime::component::Resource<Pollable>>;
                /// Returns the incoming HTTP Response, or an error, once one is ready.
                ///
                /// The outer `option` represents future readiness. Users can wait on this
                /// `option` to become `some` using the `subscribe` method.
                ///
                /// The outer `result` is used to retrieve the response or error at most
                /// once. It will be success on the first call in which the outer option
                /// is `some`, and error on subsequent calls.
                ///
                /// The inner `result` represents that either the incoming HTTP Response
                /// status and headers have recieved successfully, or that an error
                /// occured. Errors may also occur while consuming the response body,
                /// but those will be reported by the `incoming-body` and its
                /// `output-stream` child.
                fn get(
                    &mut self,
                    self_: wasmtime::component::Resource<FutureIncomingResponse>,
                ) -> wasmtime::Result<
                    Option<
                        Result<
                            Result<
                                wasmtime::component::Resource<IncomingResponse>,
                                ErrorCode,
                            >,
                            (),
                        >,
                    >,
                >;
                fn drop(
                    &mut self,
                    rep: wasmtime::component::Resource<FutureIncomingResponse>,
                ) -> wasmtime::Result<()>;
            }
            pub trait Host: HostFields + HostIncomingRequest + HostOutgoingRequest + HostRequestOptions + HostResponseOutparam + HostIncomingResponse + HostIncomingBody + HostFutureTrailers + HostOutgoingResponse + HostOutgoingBody + HostFutureIncomingResponse {
                /// Attempts to extract a http-related `error` from the wasi:io `error`
                /// provided.
                ///
                /// Stream operations which return
                /// `wasi:io/stream/stream-error::last-operation-failed` have a payload of
                /// type `wasi:io/error/error` with more information about the operation
                /// that failed. This payload can be passed through to this function to see
                /// if there's http-related information about the error to return.
                ///
                /// Note that this function is fallible because not all io-errors are
                /// http-related errors.
                fn http_error_code(
                    &mut self,
                    err: wasmtime::component::Resource<IoError>,
                ) -> wasmtime::Result<Option<ErrorCode>>;
            }
            pub fn add_to_linker<T, U>(
                linker: &mut wasmtime::component::Linker<T>,
                get: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
            ) -> wasmtime::Result<()>
            where
                U: Host,
            {
                let mut inst = linker.instance("wasi:http/types@0.2.0")?;
                inst.resource(
                    "fields",
                    wasmtime::component::ResourceType::host::<Fields>(),
                    move |mut store, rep| -> wasmtime::Result<()> {
                        HostFields::drop(
                            get(store.data_mut()),
                            wasmtime::component::Resource::new_own(rep),
                        )
                    },
                )?;
                inst.resource(
                    "incoming-request",
                    wasmtime::component::ResourceType::host::<IncomingRequest>(),
                    move |mut store, rep| -> wasmtime::Result<()> {
                        HostIncomingRequest::drop(
                            get(store.data_mut()),
                            wasmtime::component::Resource::new_own(rep),
                        )
                    },
                )?;
                inst.resource(
                    "outgoing-request",
                    wasmtime::component::ResourceType::host::<OutgoingRequest>(),
                    move |mut store, rep| -> wasmtime::Result<()> {
                        HostOutgoingRequest::drop(
                            get(store.data_mut()),
                            wasmtime::component::Resource::new_own(rep),
                        )
                    },
                )?;
                inst.resource(
                    "request-options",
                    wasmtime::component::ResourceType::host::<RequestOptions>(),
                    move |mut store, rep| -> wasmtime::Result<()> {
                        HostRequestOptions::drop(
                            get(store.data_mut()),
                            wasmtime::component::Resource::new_own(rep),
                        )
                    },
                )?;
                inst.resource(
                    "response-outparam",
                    wasmtime::component::ResourceType::host::<ResponseOutparam>(),
                    move |mut store, rep| -> wasmtime::Result<()> {
                        HostResponseOutparam::drop(
                            get(store.data_mut()),
                            wasmtime::component::Resource::new_own(rep),
                        )
                    },
                )?;
                inst.resource(
                    "incoming-response",
                    wasmtime::component::ResourceType::host::<IncomingResponse>(),
                    move |mut store, rep| -> wasmtime::Result<()> {
                        HostIncomingResponse::drop(
                            get(store.data_mut()),
                            wasmtime::component::Resource::new_own(rep),
                        )
                    },
                )?;
                inst.resource(
                    "incoming-body",
                    wasmtime::component::ResourceType::host::<IncomingBody>(),
                    move |mut store, rep| -> wasmtime::Result<()> {
                        HostIncomingBody::drop(
                            get(store.data_mut()),
                            wasmtime::component::Resource::new_own(rep),
                        )
                    },
                )?;
                inst.resource(
                    "future-trailers",
                    wasmtime::component::ResourceType::host::<FutureTrailers>(),
                    move |mut store, rep| -> wasmtime::Result<()> {
                        HostFutureTrailers::drop(
                            get(store.data_mut()),
                            wasmtime::component::Resource::new_own(rep),
                        )
                    },
                )?;
                inst.resource(
                    "outgoing-response",
                    wasmtime::component::ResourceType::host::<OutgoingResponse>(),
                    move |mut store, rep| -> wasmtime::Result<()> {
                        HostOutgoingResponse::drop(
                            get(store.data_mut()),
                            wasmtime::component::Resource::new_own(rep),
                        )
                    },
                )?;
                inst.resource(
                    "outgoing-body",
                    wasmtime::component::ResourceType::host::<OutgoingBody>(),
                    move |mut store, rep| -> wasmtime::Result<()> {
                        HostOutgoingBody::drop(
                            get(store.data_mut()),
                            wasmtime::component::Resource::new_own(rep),
                        )
                    },
                )?;
                inst.resource(
                    "future-incoming-response",
                    wasmtime::component::ResourceType::host::<FutureIncomingResponse>(),
                    move |mut store, rep| -> wasmtime::Result<()> {
                        HostFutureIncomingResponse::drop(
                            get(store.data_mut()),
                            wasmtime::component::Resource::new_own(rep),
                        )
                    },
                )?;
                inst.func_wrap(
                    "http-error-code",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (arg0,): (wasmtime::component::Resource<IoError>,)|
                    {
                        let host = get(caller.data_mut());
                        let r = Host::http_error_code(host, arg0);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap(
                    "[constructor]fields",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>, (): ()| {
                        let host = get(caller.data_mut());
                        let r = HostFields::new(host);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap(
                    "[static]fields.from-list",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (arg0,): (Vec<(FieldKey, FieldValue)>,)|
                    {
                        let host = get(caller.data_mut());
                        let r = HostFields::from_list(host, arg0);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap(
                    "[method]fields.get",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (arg0, arg1): (wasmtime::component::Resource<Fields>, FieldKey)|
                    {
                        let host = get(caller.data_mut());
                        let r = HostFields::get(host, arg0, arg1);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap(
                    "[method]fields.has",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (arg0, arg1): (wasmtime::component::Resource<Fields>, FieldKey)|
                    {
                        let host = get(caller.data_mut());
                        let r = HostFields::has(host, arg0, arg1);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap(
                    "[method]fields.set",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (
                            arg0,
                            arg1,
                            arg2,
                        ): (
                            wasmtime::component::Resource<Fields>,
                            FieldKey,
                            Vec<FieldValue>,
                        )|
                    {
                        let host = get(caller.data_mut());
                        let r = HostFields::set(host, arg0, arg1, arg2);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap(
                    "[method]fields.delete",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (arg0, arg1): (wasmtime::component::Resource<Fields>, FieldKey)|
                    {
                        let host = get(caller.data_mut());
                        let r = HostFields::delete(host, arg0, arg1);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap(
                    "[method]fields.append",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (
                            arg0,
                            arg1,
                            arg2,
                        ): (wasmtime::component::Resource<Fields>, FieldKey, FieldValue)|
                    {
                        let host = get(caller.data_mut());
                        let r = HostFields::append(host, arg0, arg1, arg2);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap(
                    "[method]fields.entries",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (arg0,): (wasmtime::component::Resource<Fields>,)|
                    {
                        let host = get(caller.data_mut());
                        let r = HostFields::entries(host, arg0);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap(
                    "[method]fields.clone",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (arg0,): (wasmtime::component::Resource<Fields>,)|
                    {
                        let host = get(caller.data_mut());
                        let r = HostFields::clone(host, arg0);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap(
                    "[method]incoming-request.method",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (arg0,): (wasmtime::component::Resource<IncomingRequest>,)|
                    {
                        let host = get(caller.data_mut());
                        let r = HostIncomingRequest::method(host, arg0);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap(
                    "[method]incoming-request.path-with-query",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (arg0,): (wasmtime::component::Resource<IncomingRequest>,)|
                    {
                        let host = get(caller.data_mut());
                        let r = HostIncomingRequest::path_with_query(host, arg0);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap(
                    "[method]incoming-request.scheme",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (arg0,): (wasmtime::component::Resource<IncomingRequest>,)|
                    {
                        let host = get(caller.data_mut());
                        let r = HostIncomingRequest::scheme(host, arg0);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap(
                    "[method]incoming-request.authority",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (arg0,): (wasmtime::component::Resource<IncomingRequest>,)|
                    {
                        let host = get(caller.data_mut());
                        let r = HostIncomingRequest::authority(host, arg0);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap(
                    "[method]incoming-request.headers",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (arg0,): (wasmtime::component::Resource<IncomingRequest>,)|
                    {
                        let host = get(caller.data_mut());
                        let r = HostIncomingRequest::headers(host, arg0);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap(
                    "[method]incoming-request.consume",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (arg0,): (wasmtime::component::Resource<IncomingRequest>,)|
                    {
                        let host = get(caller.data_mut());
                        let r = HostIncomingRequest::consume(host, arg0);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap(
                    "[constructor]outgoing-request",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (arg0,): (wasmtime::component::Resource<Headers>,)|
                    {
                        let host = get(caller.data_mut());
                        let r = HostOutgoingRequest::new(host, arg0);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap(
                    "[method]outgoing-request.body",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (arg0,): (wasmtime::component::Resource<OutgoingRequest>,)|
                    {
                        let host = get(caller.data_mut());
                        let r = HostOutgoingRequest::body(host, arg0);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap(
                    "[method]outgoing-request.method",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (arg0,): (wasmtime::component::Resource<OutgoingRequest>,)|
                    {
                        let host = get(caller.data_mut());
                        let r = HostOutgoingRequest::method(host, arg0);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap(
                    "[method]outgoing-request.set-method",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (
                            arg0,
                            arg1,
                        ): (wasmtime::component::Resource<OutgoingRequest>, Method)|
                    {
                        let host = get(caller.data_mut());
                        let r = HostOutgoingRequest::set_method(host, arg0, arg1);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap(
                    "[method]outgoing-request.path-with-query",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (arg0,): (wasmtime::component::Resource<OutgoingRequest>,)|
                    {
                        let host = get(caller.data_mut());
                        let r = HostOutgoingRequest::path_with_query(host, arg0);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap(
                    "[method]outgoing-request.set-path-with-query",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (
                            arg0,
                            arg1,
                        ): (
                            wasmtime::component::Resource<OutgoingRequest>,
                            Option<String>,
                        )|
                    {
                        let host = get(caller.data_mut());
                        let r = HostOutgoingRequest::set_path_with_query(
                            host,
                            arg0,
                            arg1,
                        );
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap(
                    "[method]outgoing-request.scheme",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (arg0,): (wasmtime::component::Resource<OutgoingRequest>,)|
                    {
                        let host = get(caller.data_mut());
                        let r = HostOutgoingRequest::scheme(host, arg0);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap(
                    "[method]outgoing-request.set-scheme",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (
                            arg0,
                            arg1,
                        ): (
                            wasmtime::component::Resource<OutgoingRequest>,
                            Option<Scheme>,
                        )|
                    {
                        let host = get(caller.data_mut());
                        let r = HostOutgoingRequest::set_scheme(host, arg0, arg1);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap(
                    "[method]outgoing-request.authority",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (arg0,): (wasmtime::component::Resource<OutgoingRequest>,)|
                    {
                        let host = get(caller.data_mut());
                        let r = HostOutgoingRequest::authority(host, arg0);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap(
                    "[method]outgoing-request.set-authority",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (
                            arg0,
                            arg1,
                        ): (
                            wasmtime::component::Resource<OutgoingRequest>,
                            Option<String>,
                        )|
                    {
                        let host = get(caller.data_mut());
                        let r = HostOutgoingRequest::set_authority(host, arg0, arg1);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap(
                    "[method]outgoing-request.headers",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (arg0,): (wasmtime::component::Resource<OutgoingRequest>,)|
                    {
                        let host = get(caller.data_mut());
                        let r = HostOutgoingRequest::headers(host, arg0);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap(
                    "[constructor]request-options",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>, (): ()| {
                        let host = get(caller.data_mut());
                        let r = HostRequestOptions::new(host);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap(
                    "[method]request-options.connect-timeout",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (arg0,): (wasmtime::component::Resource<RequestOptions>,)|
                    {
                        let host = get(caller.data_mut());
                        let r = HostRequestOptions::connect_timeout(host, arg0);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap(
                    "[method]request-options.set-connect-timeout",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (
                            arg0,
                            arg1,
                        ): (
                            wasmtime::component::Resource<RequestOptions>,
                            Option<Duration>,
                        )|
                    {
                        let host = get(caller.data_mut());
                        let r = HostRequestOptions::set_connect_timeout(
                            host,
                            arg0,
                            arg1,
                        );
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap(
                    "[method]request-options.first-byte-timeout",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (arg0,): (wasmtime::component::Resource<RequestOptions>,)|
                    {
                        let host = get(caller.data_mut());
                        let r = HostRequestOptions::first_byte_timeout(host, arg0);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap(
                    "[method]request-options.set-first-byte-timeout",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (
                            arg0,
                            arg1,
                        ): (
                            wasmtime::component::Resource<RequestOptions>,
                            Option<Duration>,
                        )|
                    {
                        let host = get(caller.data_mut());
                        let r = HostRequestOptions::set_first_byte_timeout(
                            host,
                            arg0,
                            arg1,
                        );
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap(
                    "[method]request-options.between-bytes-timeout",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (arg0,): (wasmtime::component::Resource<RequestOptions>,)|
                    {
                        let host = get(caller.data_mut());
                        let r = HostRequestOptions::between_bytes_timeout(host, arg0);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap(
                    "[method]request-options.set-between-bytes-timeout",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (
                            arg0,
                            arg1,
                        ): (
                            wasmtime::component::Resource<RequestOptions>,
                            Option<Duration>,
                        )|
                    {
                        let host = get(caller.data_mut());
                        let r = HostRequestOptions::set_between_bytes_timeout(
                            host,
                            arg0,
                            arg1,
                        );
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap(
                    "[static]response-outparam.set",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (
                            arg0,
                            arg1,
                        ): (
                            wasmtime::component::Resource<ResponseOutparam>,
                            Result<
                                wasmtime::component::Resource<OutgoingResponse>,
                                ErrorCode,
                            >,
                        )|
                    {
                        let host = get(caller.data_mut());
                        let r = HostResponseOutparam::set(host, arg0, arg1);
                        r
                    },
                )?;
                inst.func_wrap(
                    "[method]incoming-response.status",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (arg0,): (wasmtime::component::Resource<IncomingResponse>,)|
                    {
                        let host = get(caller.data_mut());
                        let r = HostIncomingResponse::status(host, arg0);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap(
                    "[method]incoming-response.headers",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (arg0,): (wasmtime::component::Resource<IncomingResponse>,)|
                    {
                        let host = get(caller.data_mut());
                        let r = HostIncomingResponse::headers(host, arg0);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap(
                    "[method]incoming-response.consume",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (arg0,): (wasmtime::component::Resource<IncomingResponse>,)|
                    {
                        let host = get(caller.data_mut());
                        let r = HostIncomingResponse::consume(host, arg0);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap(
                    "[method]incoming-body.stream",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (arg0,): (wasmtime::component::Resource<IncomingBody>,)|
                    {
                        let host = get(caller.data_mut());
                        let r = HostIncomingBody::stream(host, arg0);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap(
                    "[static]incoming-body.finish",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (arg0,): (wasmtime::component::Resource<IncomingBody>,)|
                    {
                        let host = get(caller.data_mut());
                        let r = HostIncomingBody::finish(host, arg0);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap(
                    "[method]future-trailers.subscribe",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (arg0,): (wasmtime::component::Resource<FutureTrailers>,)|
                    {
                        let host = get(caller.data_mut());
                        let r = HostFutureTrailers::subscribe(host, arg0);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap(
                    "[method]future-trailers.get",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (arg0,): (wasmtime::component::Resource<FutureTrailers>,)|
                    {
                        let host = get(caller.data_mut());
                        let r = HostFutureTrailers::get(host, arg0);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap(
                    "[constructor]outgoing-response",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (arg0,): (wasmtime::component::Resource<Headers>,)|
                    {
                        let host = get(caller.data_mut());
                        let r = HostOutgoingResponse::new(host, arg0);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap(
                    "[method]outgoing-response.status-code",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (arg0,): (wasmtime::component::Resource<OutgoingResponse>,)|
                    {
                        let host = get(caller.data_mut());
                        let r = HostOutgoingResponse::status_code(host, arg0);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap(
                    "[method]outgoing-response.set-status-code",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (
                            arg0,
                            arg1,
                        ): (wasmtime::component::Resource<OutgoingResponse>, StatusCode)|
                    {
                        let host = get(caller.data_mut());
                        let r = HostOutgoingResponse::set_status_code(host, arg0, arg1);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap(
                    "[method]outgoing-response.headers",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (arg0,): (wasmtime::component::Resource<OutgoingResponse>,)|
                    {
                        let host = get(caller.data_mut());
                        let r = HostOutgoingResponse::headers(host, arg0);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap(
                    "[method]outgoing-response.body",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (arg0,): (wasmtime::component::Resource<OutgoingResponse>,)|
                    {
                        let host = get(caller.data_mut());
                        let r = HostOutgoingResponse::body(host, arg0);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap(
                    "[method]outgoing-body.write",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (arg0,): (wasmtime::component::Resource<OutgoingBody>,)|
                    {
                        let host = get(caller.data_mut());
                        let r = HostOutgoingBody::write(host, arg0);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap(
                    "[static]outgoing-body.finish",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (
                            arg0,
                            arg1,
                        ): (
                            wasmtime::component::Resource<OutgoingBody>,
                            Option<wasmtime::component::Resource<Trailers>>,
                        )|
                    {
                        let host = get(caller.data_mut());
                        let r = HostOutgoingBody::finish(host, arg0, arg1);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap(
                    "[method]future-incoming-response.subscribe",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (
                            arg0,
                        ): (wasmtime::component::Resource<FutureIncomingResponse>,)|
                    {
                        let host = get(caller.data_mut());
                        let r = HostFutureIncomingResponse::subscribe(host, arg0);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap(
                    "[method]future-incoming-response.get",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (
                            arg0,
                        ): (wasmtime::component::Resource<FutureIncomingResponse>,)|
                    {
                        let host = get(caller.data_mut());
                        let r = HostFutureIncomingResponse::get(host, arg0);
                        Ok((r?,))
                    },
                )?;
                Ok(())
            }
        }
        #[allow(clippy::all)]
        pub mod outgoing_handler {
            #[allow(unused_imports)]
            use wasmtime::component::__internal::anyhow;
            pub type OutgoingRequest = super::super::super::wasi::http::types::OutgoingRequest;
            pub type RequestOptions = super::super::super::wasi::http::types::RequestOptions;
            pub type FutureIncomingResponse = super::super::super::wasi::http::types::FutureIncomingResponse;
            pub type ErrorCode = super::super::super::wasi::http::types::ErrorCode;
            const _: () = {
                if !(32 == <ErrorCode as wasmtime::component::ComponentType>::SIZE32) {
                    ::core::panicking::panic(
                        "assertion failed: 32 == <ErrorCode as wasmtime::component::ComponentType>::SIZE32",
                    )
                }
                if !(8 == <ErrorCode as wasmtime::component::ComponentType>::ALIGN32) {
                    ::core::panicking::panic(
                        "assertion failed: 8 == <ErrorCode as wasmtime::component::ComponentType>::ALIGN32",
                    )
                }
            };
            pub trait Host {
                /// This function is invoked with an outgoing HTTP Request, and it returns
                /// a resource `future-incoming-response` which represents an HTTP Response
                /// which may arrive in the future.
                ///
                /// The `options` argument accepts optional parameters for the HTTP
                /// protocol's transport layer.
                ///
                /// This function may return an error if the `outgoing-request` is invalid
                /// or not allowed to be made. Otherwise, protocol errors are reported
                /// through the `future-incoming-response`.
                fn handle(
                    &mut self,
                    request: wasmtime::component::Resource<OutgoingRequest>,
                    options: Option<wasmtime::component::Resource<RequestOptions>>,
                ) -> wasmtime::Result<
                    Result<
                        wasmtime::component::Resource<FutureIncomingResponse>,
                        ErrorCode,
                    >,
                >;
            }
            pub fn add_to_linker<T, U>(
                linker: &mut wasmtime::component::Linker<T>,
                get: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
            ) -> wasmtime::Result<()>
            where
                U: Host,
            {
                let mut inst = linker.instance("wasi:http/outgoing-handler@0.2.0")?;
                inst.func_wrap(
                    "handle",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (
                            arg0,
                            arg1,
                        ): (
                            wasmtime::component::Resource<OutgoingRequest>,
                            Option<wasmtime::component::Resource<RequestOptions>>,
                        )|
                    {
                        let host = get(caller.data_mut());
                        let r = Host::handle(host, arg0, arg1);
                        Ok((r?,))
                    },
                )?;
                Ok(())
            }
        }
    }
    pub mod io {
        #[allow(clippy::all)]
        pub mod poll {
            #[allow(unused_imports)]
            use wasmtime::component::__internal::anyhow;
            /// `pollable` represents a single I/O event which may be ready, or not.
            pub enum Pollable {}
            pub trait HostPollable {
                /// Return the readiness of a pollable. This function never blocks.
                ///
                /// Returns `true` when the pollable is ready, and `false` otherwise.
                fn ready(
                    &mut self,
                    self_: wasmtime::component::Resource<Pollable>,
                ) -> wasmtime::Result<bool>;
                /// `block` returns immediately if the pollable is ready, and otherwise
                /// blocks until ready.
                ///
                /// This function is equivalent to calling `poll.poll` on a list
                /// containing only this pollable.
                fn block(
                    &mut self,
                    self_: wasmtime::component::Resource<Pollable>,
                ) -> wasmtime::Result<()>;
                fn drop(
                    &mut self,
                    rep: wasmtime::component::Resource<Pollable>,
                ) -> wasmtime::Result<()>;
            }
            pub trait Host: HostPollable {
                /// Poll for completion on a set of pollables.
                ///
                /// This function takes a list of pollables, which identify I/O sources of
                /// interest, and waits until one or more of the events is ready for I/O.
                ///
                /// The result `list<u32>` contains one or more indices of handles in the
                /// argument list that is ready for I/O.
                ///
                /// If the list contains more elements than can be indexed with a `u32`
                /// value, this function traps.
                ///
                /// A timeout can be implemented by adding a pollable from the
                /// wasi-clocks API to the list.
                ///
                /// This function does not return a `result`; polling in itself does not
                /// do any I/O so it doesn't fail. If any of the I/O sources identified by
                /// the pollables has an error, it is indicated by marking the source as
                /// being reaedy for I/O.
                fn poll(
                    &mut self,
                    in_: Vec<wasmtime::component::Resource<Pollable>>,
                ) -> wasmtime::Result<Vec<u32>>;
            }
            pub fn add_to_linker<T, U>(
                linker: &mut wasmtime::component::Linker<T>,
                get: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
            ) -> wasmtime::Result<()>
            where
                U: Host,
            {
                let mut inst = linker.instance("wasi:io/poll@0.2.0")?;
                inst.resource(
                    "pollable",
                    wasmtime::component::ResourceType::host::<Pollable>(),
                    move |mut store, rep| -> wasmtime::Result<()> {
                        HostPollable::drop(
                            get(store.data_mut()),
                            wasmtime::component::Resource::new_own(rep),
                        )
                    },
                )?;
                inst.func_wrap(
                    "[method]pollable.ready",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (arg0,): (wasmtime::component::Resource<Pollable>,)|
                    {
                        let host = get(caller.data_mut());
                        let r = HostPollable::ready(host, arg0);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap(
                    "[method]pollable.block",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (arg0,): (wasmtime::component::Resource<Pollable>,)|
                    {
                        let host = get(caller.data_mut());
                        let r = HostPollable::block(host, arg0);
                        r
                    },
                )?;
                inst.func_wrap(
                    "poll",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (arg0,): (Vec<wasmtime::component::Resource<Pollable>>,)|
                    {
                        let host = get(caller.data_mut());
                        let r = Host::poll(host, arg0);
                        Ok((r?,))
                    },
                )?;
                Ok(())
            }
        }
        #[allow(clippy::all)]
        pub mod error {
            #[allow(unused_imports)]
            use wasmtime::component::__internal::anyhow;
            /// A resource which represents some error information.
            ///
            /// The only method provided by this resource is `to-debug-string`,
            /// which provides some human-readable information about the error.
            ///
            /// In the `wasi:io` package, this resource is returned through the
            /// `wasi:io/streams/stream-error` type.
            ///
            /// To provide more specific error information, other interfaces may
            /// provide functions to further "downcast" this error into more specific
            /// error information. For example, `error`s returned in streams derived
            /// from filesystem types to be described using the filesystem's own
            /// error-code type, using the function
            /// `wasi:filesystem/types/filesystem-error-code`, which takes a parameter
            /// `borrow<error>` and returns
            /// `option<wasi:filesystem/types/error-code>`.
            ///
            /// The set of functions which can "downcast" an `error` into a more
            /// concrete type is open.
            pub enum Error {}
            pub trait HostError {
                /// Returns a string that is suitable to assist humans in debugging
                /// this error.
                ///
                /// WARNING: The returned string should not be consumed mechanically!
                /// It may change across platforms, hosts, or other implementation
                /// details. Parsing this string is a major platform-compatibility
                /// hazard.
                fn to_debug_string(
                    &mut self,
                    self_: wasmtime::component::Resource<Error>,
                ) -> wasmtime::Result<String>;
                fn drop(
                    &mut self,
                    rep: wasmtime::component::Resource<Error>,
                ) -> wasmtime::Result<()>;
            }
            pub trait Host: HostError {}
            pub fn add_to_linker<T, U>(
                linker: &mut wasmtime::component::Linker<T>,
                get: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
            ) -> wasmtime::Result<()>
            where
                U: Host,
            {
                let mut inst = linker.instance("wasi:io/error@0.2.0")?;
                inst.resource(
                    "error",
                    wasmtime::component::ResourceType::host::<Error>(),
                    move |mut store, rep| -> wasmtime::Result<()> {
                        HostError::drop(
                            get(store.data_mut()),
                            wasmtime::component::Resource::new_own(rep),
                        )
                    },
                )?;
                inst.func_wrap(
                    "[method]error.to-debug-string",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (arg0,): (wasmtime::component::Resource<Error>,)|
                    {
                        let host = get(caller.data_mut());
                        let r = HostError::to_debug_string(host, arg0);
                        Ok((r?,))
                    },
                )?;
                Ok(())
            }
        }
        #[allow(clippy::all)]
        pub mod streams {
            #[allow(unused_imports)]
            use wasmtime::component::__internal::anyhow;
            pub type Error = super::super::super::wasi::io::error::Error;
            pub type Pollable = super::super::super::wasi::io::poll::Pollable;
            /// An error for input-stream and output-stream operations.
            #[component(variant)]
            pub enum StreamError {
                /// The last operation (a write or flush) failed before completion.
                ///
                /// More information is available in the `error` payload.
                #[component(name = "last-operation-failed")]
                LastOperationFailed(wasmtime::component::Resource<Error>),
                /// The stream is closed: no more input will be accepted by the
                /// stream. A closed output-stream will return this error on all
                /// future operations.
                #[component(name = "closed")]
                Closed,
            }
            unsafe impl wasmtime::component::Lower for StreamError {
                #[inline]
                fn lower<T>(
                    &self,
                    cx: &mut wasmtime::component::__internal::LowerContext<'_, T>,
                    ty: wasmtime::component::__internal::InterfaceType,
                    dst: &mut std::mem::MaybeUninit<Self::Lower>,
                ) -> wasmtime::component::__internal::anyhow::Result<()> {
                    let ty = match ty {
                        wasmtime::component::__internal::InterfaceType::Variant(i) => {
                            &cx.types[i]
                        }
                        _ => wasmtime::component::__internal::bad_type_info(),
                    };
                    match self {
                        Self::LastOperationFailed(value) => {
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
                                                m.map(|p| &raw mut (*p).LastOperationFailed)
                                            }
                                        }
                                    },
                                    |dst| {
                                        value
                                            .lower(
                                                cx,
                                                ty
                                                    .cases[0usize]
                                                    .ty
                                                    .unwrap_or_else(
                                                        wasmtime::component::__internal::bad_type_info,
                                                    ),
                                                dst,
                                            )
                                    },
                                )
                            }
                        }
                        Self::Closed => {
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
                                                m.map(|p| &raw mut (*p).Closed)
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
                        wasmtime::component::__internal::InterfaceType::Variant(i) => {
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
                        Self::LastOperationFailed(value) => {
                            *cx.get::<1usize>(offset) = 0u8.to_le_bytes();
                            value
                                .store(
                                    cx,
                                    ty
                                        .cases[0usize]
                                        .ty
                                        .unwrap_or_else(
                                            wasmtime::component::__internal::bad_type_info,
                                        ),
                                    offset
                                        + <Self as wasmtime::component::__internal::ComponentVariant>::PAYLOAD_OFFSET32,
                                )
                        }
                        Self::Closed => {
                            *cx.get::<1usize>(offset) = 1u8.to_le_bytes();
                            Ok(())
                        }
                    }
                }
            }
            unsafe impl wasmtime::component::Lift for StreamError {
                #[inline]
                fn lift(
                    cx: &mut wasmtime::component::__internal::LiftContext<'_>,
                    ty: wasmtime::component::__internal::InterfaceType,
                    src: &Self::Lower,
                ) -> wasmtime::component::__internal::anyhow::Result<Self> {
                    let ty = match ty {
                        wasmtime::component::__internal::InterfaceType::Variant(i) => {
                            &cx.types[i]
                        }
                        _ => wasmtime::component::__internal::bad_type_info(),
                    };
                    Ok(
                        match src.tag.get_u32() {
                            0u32 => {
                                Self::LastOperationFailed(
                                    <wasmtime::component::Resource<
                                        Error,
                                    > as wasmtime::component::Lift>::lift(
                                        cx,
                                        ty
                                            .cases[0usize]
                                            .ty
                                            .unwrap_or_else(
                                                wasmtime::component::__internal::bad_type_info,
                                            ),
                                        unsafe { &src.payload.LastOperationFailed },
                                    )?,
                                )
                            }
                            1u32 => Self::Closed,
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
                        wasmtime::component::__internal::InterfaceType::Variant(i) => {
                            &cx.types[i]
                        }
                        _ => wasmtime::component::__internal::bad_type_info(),
                    };
                    Ok(
                        match discrim {
                            0u8 => {
                                Self::LastOperationFailed(
                                    <wasmtime::component::Resource<
                                        Error,
                                    > as wasmtime::component::Lift>::load(
                                        cx,
                                        ty
                                            .cases[0usize]
                                            .ty
                                            .unwrap_or_else(
                                                wasmtime::component::__internal::bad_type_info,
                                            ),
                                        &payload[..<wasmtime::component::Resource<
                                            Error,
                                        > as wasmtime::component::ComponentType>::SIZE32],
                                    )?,
                                )
                            }
                            1u8 => Self::Closed,
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
                pub struct LowerStreamError<T0: Copy> {
                    tag: wasmtime::ValRaw,
                    payload: LowerPayloadStreamError<T0>,
                }
                #[automatically_derived]
                impl<T0: ::core::clone::Clone + Copy> ::core::clone::Clone
                for LowerStreamError<T0> {
                    #[inline]
                    fn clone(&self) -> LowerStreamError<T0> {
                        LowerStreamError {
                            tag: ::core::clone::Clone::clone(&self.tag),
                            payload: ::core::clone::Clone::clone(&self.payload),
                        }
                    }
                }
                #[automatically_derived]
                impl<T0: ::core::marker::Copy + Copy> ::core::marker::Copy
                for LowerStreamError<T0> {}
                #[doc(hidden)]
                #[allow(non_snake_case)]
                #[repr(C)]
                union LowerPayloadStreamError<T0: Copy> {
                    LastOperationFailed: T0,
                    Closed: [wasmtime::ValRaw; 0],
                }
                #[automatically_derived]
                #[allow(non_snake_case)]
                impl<
                    T0: ::core::marker::Copy + ::core::clone::Clone + Copy,
                > ::core::clone::Clone for LowerPayloadStreamError<T0> {
                    #[inline]
                    fn clone(&self) -> LowerPayloadStreamError<T0> {
                        let _: ::core::clone::AssertParamIsCopy<Self>;
                        *self
                    }
                }
                #[automatically_derived]
                #[allow(non_snake_case)]
                impl<T0: ::core::marker::Copy + Copy> ::core::marker::Copy
                for LowerPayloadStreamError<T0> {}
                unsafe impl wasmtime::component::ComponentType for StreamError {
                    type Lower = LowerStreamError<
                        <wasmtime::component::Resource<
                            Error,
                        > as wasmtime::component::ComponentType>::Lower,
                    >;
                    #[inline]
                    fn typecheck(
                        ty: &wasmtime::component::__internal::InterfaceType,
                        types: &wasmtime::component::__internal::InstanceType<'_>,
                    ) -> wasmtime::component::__internal::anyhow::Result<()> {
                        wasmtime::component::__internal::typecheck_variant(
                            ty,
                            types,
                            &[
                                (
                                    "last-operation-failed",
                                    Some(
                                        <wasmtime::component::Resource<
                                            Error,
                                        > as wasmtime::component::ComponentType>::typecheck,
                                    ),
                                ),
                                ("closed", None),
                            ],
                        )
                    }
                    const ABI: wasmtime::component::__internal::CanonicalAbiInfo = wasmtime::component::__internal::CanonicalAbiInfo::variant_static(
                        &[
                            Some(
                                <wasmtime::component::Resource<
                                    Error,
                                > as wasmtime::component::ComponentType>::ABI,
                            ),
                            None,
                        ],
                    );
                }
                unsafe impl wasmtime::component::__internal::ComponentVariant
                for StreamError {
                    const CASES: &'static [Option<
                        wasmtime::component::__internal::CanonicalAbiInfo,
                    >] = &[
                        Some(
                            <wasmtime::component::Resource<
                                Error,
                            > as wasmtime::component::ComponentType>::ABI,
                        ),
                        None,
                    ];
                }
            };
            impl core::fmt::Debug for StreamError {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    match self {
                        StreamError::LastOperationFailed(e) => {
                            f.debug_tuple("StreamError::LastOperationFailed")
                                .field(e)
                                .finish()
                        }
                        StreamError::Closed => {
                            f.debug_tuple("StreamError::Closed").finish()
                        }
                    }
                }
            }
            impl core::fmt::Display for StreamError {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    f.write_fmt(format_args!("{0:?}", self))
                }
            }
            impl std::error::Error for StreamError {}
            const _: () = {
                if !(8 == <StreamError as wasmtime::component::ComponentType>::SIZE32) {
                    ::core::panicking::panic(
                        "assertion failed: 8 == <StreamError as wasmtime::component::ComponentType>::SIZE32",
                    )
                }
                if !(4 == <StreamError as wasmtime::component::ComponentType>::ALIGN32) {
                    ::core::panicking::panic(
                        "assertion failed: 4 == <StreamError as wasmtime::component::ComponentType>::ALIGN32",
                    )
                }
            };
            /// An input bytestream.
            ///
            /// `input-stream`s are *non-blocking* to the extent practical on underlying
            /// platforms. I/O operations always return promptly; if fewer bytes are
            /// promptly available than requested, they return the number of bytes promptly
            /// available, which could even be zero. To wait for data to be available,
            /// use the `subscribe` function to obtain a `pollable` which can be polled
            /// for using `wasi:io/poll`.
            pub enum InputStream {}
            pub trait HostInputStream {
                /// Perform a non-blocking read from the stream.
                ///
                /// When the source of a `read` is binary data, the bytes from the source
                /// are returned verbatim. When the source of a `read` is known to the
                /// implementation to be text, bytes containing the UTF-8 encoding of the
                /// text are returned.
                ///
                /// This function returns a list of bytes containing the read data,
                /// when successful. The returned list will contain up to `len` bytes;
                /// it may return fewer than requested, but not more. The list is
                /// empty when no bytes are available for reading at this time. The
                /// pollable given by `subscribe` will be ready when more bytes are
                /// available.
                ///
                /// This function fails with a `stream-error` when the operation
                /// encounters an error, giving `last-operation-failed`, or when the
                /// stream is closed, giving `closed`.
                ///
                /// When the caller gives a `len` of 0, it represents a request to
                /// read 0 bytes. If the stream is still open, this call should
                /// succeed and return an empty list, or otherwise fail with `closed`.
                ///
                /// The `len` parameter is a `u64`, which could represent a list of u8 which
                /// is not possible to allocate in wasm32, or not desirable to allocate as
                /// as a return value by the callee. The callee may return a list of bytes
                /// less than `len` in size while more bytes are available for reading.
                fn read(
                    &mut self,
                    self_: wasmtime::component::Resource<InputStream>,
                    len: u64,
                ) -> wasmtime::Result<Result<Vec<u8>, StreamError>>;
                /// Read bytes from a stream, after blocking until at least one byte can
                /// be read. Except for blocking, behavior is identical to `read`.
                fn blocking_read(
                    &mut self,
                    self_: wasmtime::component::Resource<InputStream>,
                    len: u64,
                ) -> wasmtime::Result<Result<Vec<u8>, StreamError>>;
                /// Skip bytes from a stream. Returns number of bytes skipped.
                ///
                /// Behaves identical to `read`, except instead of returning a list
                /// of bytes, returns the number of bytes consumed from the stream.
                fn skip(
                    &mut self,
                    self_: wasmtime::component::Resource<InputStream>,
                    len: u64,
                ) -> wasmtime::Result<Result<u64, StreamError>>;
                /// Skip bytes from a stream, after blocking until at least one byte
                /// can be skipped. Except for blocking behavior, identical to `skip`.
                fn blocking_skip(
                    &mut self,
                    self_: wasmtime::component::Resource<InputStream>,
                    len: u64,
                ) -> wasmtime::Result<Result<u64, StreamError>>;
                /// Create a `pollable` which will resolve once either the specified stream
                /// has bytes available to read or the other end of the stream has been
                /// closed.
                /// The created `pollable` is a child resource of the `input-stream`.
                /// Implementations may trap if the `input-stream` is dropped before
                /// all derived `pollable`s created with this function are dropped.
                fn subscribe(
                    &mut self,
                    self_: wasmtime::component::Resource<InputStream>,
                ) -> wasmtime::Result<wasmtime::component::Resource<Pollable>>;
                fn drop(
                    &mut self,
                    rep: wasmtime::component::Resource<InputStream>,
                ) -> wasmtime::Result<()>;
            }
            /// An output bytestream.
            ///
            /// `output-stream`s are *non-blocking* to the extent practical on
            /// underlying platforms. Except where specified otherwise, I/O operations also
            /// always return promptly, after the number of bytes that can be written
            /// promptly, which could even be zero. To wait for the stream to be ready to
            /// accept data, the `subscribe` function to obtain a `pollable` which can be
            /// polled for using `wasi:io/poll`.
            pub enum OutputStream {}
            pub trait HostOutputStream {
                /// Check readiness for writing. This function never blocks.
                ///
                /// Returns the number of bytes permitted for the next call to `write`,
                /// or an error. Calling `write` with more bytes than this function has
                /// permitted will trap.
                ///
                /// When this function returns 0 bytes, the `subscribe` pollable will
                /// become ready when this function will report at least 1 byte, or an
                /// error.
                fn check_write(
                    &mut self,
                    self_: wasmtime::component::Resource<OutputStream>,
                ) -> wasmtime::Result<Result<u64, StreamError>>;
                /// Perform a write. This function never blocks.
                ///
                /// When the destination of a `write` is binary data, the bytes from
                /// `contents` are written verbatim. When the destination of a `write` is
                /// known to the implementation to be text, the bytes of `contents` are
                /// transcoded from UTF-8 into the encoding of the destination and then
                /// written.
                ///
                /// Precondition: check-write gave permit of Ok(n) and contents has a
                /// length of less than or equal to n. Otherwise, this function will trap.
                ///
                /// returns Err(closed) without writing if the stream has closed since
                /// the last call to check-write provided a permit.
                fn write(
                    &mut self,
                    self_: wasmtime::component::Resource<OutputStream>,
                    contents: Vec<u8>,
                ) -> wasmtime::Result<Result<(), StreamError>>;
                /// Perform a write of up to 4096 bytes, and then flush the stream. Block
                /// until all of these operations are complete, or an error occurs.
                ///
                /// This is a convenience wrapper around the use of `check-write`,
                /// `subscribe`, `write`, and `flush`, and is implemented with the
                /// following pseudo-code:
                ///
                /// ```text
                /// let pollable = this.subscribe();
                /// while !contents.is_empty() {
                /// // Wait for the stream to become writable
                /// pollable.block();
                /// let Ok(n) = this.check-write(); // eliding error handling
                /// let len = min(n, contents.len());
                /// let (chunk, rest) = contents.split_at(len);
                /// this.write(chunk  );            // eliding error handling
                /// contents = rest;
                /// }
                /// this.flush();
                /// // Wait for completion of `flush`
                /// pollable.block();
                /// // Check for any errors that arose during `flush`
                /// let _ = this.check-write();         // eliding error handling
                /// ```
                fn blocking_write_and_flush(
                    &mut self,
                    self_: wasmtime::component::Resource<OutputStream>,
                    contents: Vec<u8>,
                ) -> wasmtime::Result<Result<(), StreamError>>;
                /// Request to flush buffered output. This function never blocks.
                ///
                /// This tells the output-stream that the caller intends any buffered
                /// output to be flushed. the output which is expected to be flushed
                /// is all that has been passed to `write` prior to this call.
                ///
                /// Upon calling this function, the `output-stream` will not accept any
                /// writes (`check-write` will return `ok(0)`) until the flush has
                /// completed. The `subscribe` pollable will become ready when the
                /// flush has completed and the stream can accept more writes.
                fn flush(
                    &mut self,
                    self_: wasmtime::component::Resource<OutputStream>,
                ) -> wasmtime::Result<Result<(), StreamError>>;
                /// Request to flush buffered output, and block until flush completes
                /// and stream is ready for writing again.
                fn blocking_flush(
                    &mut self,
                    self_: wasmtime::component::Resource<OutputStream>,
                ) -> wasmtime::Result<Result<(), StreamError>>;
                /// Create a `pollable` which will resolve once the output-stream
                /// is ready for more writing, or an error has occured. When this
                /// pollable is ready, `check-write` will return `ok(n)` with n>0, or an
                /// error.
                ///
                /// If the stream is closed, this pollable is always ready immediately.
                ///
                /// The created `pollable` is a child resource of the `output-stream`.
                /// Implementations may trap if the `output-stream` is dropped before
                /// all derived `pollable`s created with this function are dropped.
                fn subscribe(
                    &mut self,
                    self_: wasmtime::component::Resource<OutputStream>,
                ) -> wasmtime::Result<wasmtime::component::Resource<Pollable>>;
                /// Write zeroes to a stream.
                ///
                /// This should be used precisely like `write` with the exact same
                /// preconditions (must use check-write first), but instead of
                /// passing a list of bytes, you simply pass the number of zero-bytes
                /// that should be written.
                fn write_zeroes(
                    &mut self,
                    self_: wasmtime::component::Resource<OutputStream>,
                    len: u64,
                ) -> wasmtime::Result<Result<(), StreamError>>;
                /// Perform a write of up to 4096 zeroes, and then flush the stream.
                /// Block until all of these operations are complete, or an error
                /// occurs.
                ///
                /// This is a convenience wrapper around the use of `check-write`,
                /// `subscribe`, `write-zeroes`, and `flush`, and is implemented with
                /// the following pseudo-code:
                ///
                /// ```text
                /// let pollable = this.subscribe();
                /// while num_zeroes != 0 {
                /// // Wait for the stream to become writable
                /// pollable.block();
                /// let Ok(n) = this.check-write(); // eliding error handling
                /// let len = min(n, num_zeroes);
                /// this.write-zeroes(len);         // eliding error handling
                /// num_zeroes -= len;
                /// }
                /// this.flush();
                /// // Wait for completion of `flush`
                /// pollable.block();
                /// // Check for any errors that arose during `flush`
                /// let _ = this.check-write();         // eliding error handling
                /// ```
                fn blocking_write_zeroes_and_flush(
                    &mut self,
                    self_: wasmtime::component::Resource<OutputStream>,
                    len: u64,
                ) -> wasmtime::Result<Result<(), StreamError>>;
                /// Read from one stream and write to another.
                ///
                /// The behavior of splice is equivelant to:
                /// 1. calling `check-write` on the `output-stream`
                /// 2. calling `read` on the `input-stream` with the smaller of the
                /// `check-write` permitted length and the `len` provided to `splice`
                /// 3. calling `write` on the `output-stream` with that read data.
                ///
                /// Any error reported by the call to `check-write`, `read`, or
                /// `write` ends the splice and reports that error.
                ///
                /// This function returns the number of bytes transferred; it may be less
                /// than `len`.
                fn splice(
                    &mut self,
                    self_: wasmtime::component::Resource<OutputStream>,
                    src: wasmtime::component::Resource<InputStream>,
                    len: u64,
                ) -> wasmtime::Result<Result<u64, StreamError>>;
                /// Read from one stream and write to another, with blocking.
                ///
                /// This is similar to `splice`, except that it blocks until the
                /// `output-stream` is ready for writing, and the `input-stream`
                /// is ready for reading, before performing the `splice`.
                fn blocking_splice(
                    &mut self,
                    self_: wasmtime::component::Resource<OutputStream>,
                    src: wasmtime::component::Resource<InputStream>,
                    len: u64,
                ) -> wasmtime::Result<Result<u64, StreamError>>;
                fn drop(
                    &mut self,
                    rep: wasmtime::component::Resource<OutputStream>,
                ) -> wasmtime::Result<()>;
            }
            pub trait Host: HostInputStream + HostOutputStream {}
            pub fn add_to_linker<T, U>(
                linker: &mut wasmtime::component::Linker<T>,
                get: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
            ) -> wasmtime::Result<()>
            where
                U: Host,
            {
                let mut inst = linker.instance("wasi:io/streams@0.2.0")?;
                inst.resource(
                    "input-stream",
                    wasmtime::component::ResourceType::host::<InputStream>(),
                    move |mut store, rep| -> wasmtime::Result<()> {
                        HostInputStream::drop(
                            get(store.data_mut()),
                            wasmtime::component::Resource::new_own(rep),
                        )
                    },
                )?;
                inst.resource(
                    "output-stream",
                    wasmtime::component::ResourceType::host::<OutputStream>(),
                    move |mut store, rep| -> wasmtime::Result<()> {
                        HostOutputStream::drop(
                            get(store.data_mut()),
                            wasmtime::component::Resource::new_own(rep),
                        )
                    },
                )?;
                inst.func_wrap(
                    "[method]input-stream.read",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (arg0, arg1): (wasmtime::component::Resource<InputStream>, u64)|
                    {
                        let host = get(caller.data_mut());
                        let r = HostInputStream::read(host, arg0, arg1);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap(
                    "[method]input-stream.blocking-read",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (arg0, arg1): (wasmtime::component::Resource<InputStream>, u64)|
                    {
                        let host = get(caller.data_mut());
                        let r = HostInputStream::blocking_read(host, arg0, arg1);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap(
                    "[method]input-stream.skip",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (arg0, arg1): (wasmtime::component::Resource<InputStream>, u64)|
                    {
                        let host = get(caller.data_mut());
                        let r = HostInputStream::skip(host, arg0, arg1);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap(
                    "[method]input-stream.blocking-skip",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (arg0, arg1): (wasmtime::component::Resource<InputStream>, u64)|
                    {
                        let host = get(caller.data_mut());
                        let r = HostInputStream::blocking_skip(host, arg0, arg1);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap(
                    "[method]input-stream.subscribe",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (arg0,): (wasmtime::component::Resource<InputStream>,)|
                    {
                        let host = get(caller.data_mut());
                        let r = HostInputStream::subscribe(host, arg0);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap(
                    "[method]output-stream.check-write",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (arg0,): (wasmtime::component::Resource<OutputStream>,)|
                    {
                        let host = get(caller.data_mut());
                        let r = HostOutputStream::check_write(host, arg0);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap(
                    "[method]output-stream.write",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (
                            arg0,
                            arg1,
                        ): (wasmtime::component::Resource<OutputStream>, Vec<u8>)|
                    {
                        let host = get(caller.data_mut());
                        let r = HostOutputStream::write(host, arg0, arg1);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap(
                    "[method]output-stream.blocking-write-and-flush",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (
                            arg0,
                            arg1,
                        ): (wasmtime::component::Resource<OutputStream>, Vec<u8>)|
                    {
                        let host = get(caller.data_mut());
                        let r = HostOutputStream::blocking_write_and_flush(
                            host,
                            arg0,
                            arg1,
                        );
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap(
                    "[method]output-stream.flush",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (arg0,): (wasmtime::component::Resource<OutputStream>,)|
                    {
                        let host = get(caller.data_mut());
                        let r = HostOutputStream::flush(host, arg0);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap(
                    "[method]output-stream.blocking-flush",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (arg0,): (wasmtime::component::Resource<OutputStream>,)|
                    {
                        let host = get(caller.data_mut());
                        let r = HostOutputStream::blocking_flush(host, arg0);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap(
                    "[method]output-stream.subscribe",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (arg0,): (wasmtime::component::Resource<OutputStream>,)|
                    {
                        let host = get(caller.data_mut());
                        let r = HostOutputStream::subscribe(host, arg0);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap(
                    "[method]output-stream.write-zeroes",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (arg0, arg1): (wasmtime::component::Resource<OutputStream>, u64)|
                    {
                        let host = get(caller.data_mut());
                        let r = HostOutputStream::write_zeroes(host, arg0, arg1);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap(
                    "[method]output-stream.blocking-write-zeroes-and-flush",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (arg0, arg1): (wasmtime::component::Resource<OutputStream>, u64)|
                    {
                        let host = get(caller.data_mut());
                        let r = HostOutputStream::blocking_write_zeroes_and_flush(
                            host,
                            arg0,
                            arg1,
                        );
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap(
                    "[method]output-stream.splice",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (
                            arg0,
                            arg1,
                            arg2,
                        ): (
                            wasmtime::component::Resource<OutputStream>,
                            wasmtime::component::Resource<InputStream>,
                            u64,
                        )|
                    {
                        let host = get(caller.data_mut());
                        let r = HostOutputStream::splice(host, arg0, arg1, arg2);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap(
                    "[method]output-stream.blocking-splice",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (
                            arg0,
                            arg1,
                            arg2,
                        ): (
                            wasmtime::component::Resource<OutputStream>,
                            wasmtime::component::Resource<InputStream>,
                            u64,
                        )|
                    {
                        let host = get(caller.data_mut());
                        let r = HostOutputStream::blocking_splice(
                            host,
                            arg0,
                            arg1,
                            arg2,
                        );
                        Ok((r?,))
                    },
                )?;
                Ok(())
            }
        }
    }
    pub mod random {
        #[allow(clippy::all)]
        pub mod random {
            #[allow(unused_imports)]
            use wasmtime::component::__internal::anyhow;
            pub trait Host {
                /// Return `len` cryptographically-secure random or pseudo-random bytes.
                ///
                /// This function must produce data at least as cryptographically secure and
                /// fast as an adequately seeded cryptographically-secure pseudo-random
                /// number generator (CSPRNG). It must not block, from the perspective of
                /// the calling program, under any circumstances, including on the first
                /// request and on requests for numbers of bytes. The returned data must
                /// always be unpredictable.
                ///
                /// This function must always return fresh data. Deterministic environments
                /// must omit this function, rather than implementing it with deterministic
                /// data.
                fn get_random_bytes(&mut self, len: u64) -> wasmtime::Result<Vec<u8>>;
                /// Return a cryptographically-secure random or pseudo-random `u64` value.
                ///
                /// This function returns the same type of data as `get-random-bytes`,
                /// represented as a `u64`.
                fn get_random_u64(&mut self) -> wasmtime::Result<u64>;
            }
            pub fn add_to_linker<T, U>(
                linker: &mut wasmtime::component::Linker<T>,
                get: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
            ) -> wasmtime::Result<()>
            where
                U: Host,
            {
                let mut inst = linker.instance("wasi:random/random@0.2.0")?;
                inst.func_wrap(
                    "get-random-bytes",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,): (u64,)| {
                        let host = get(caller.data_mut());
                        let r = Host::get_random_bytes(host, arg0);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap(
                    "get-random-u64",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>, (): ()| {
                        let host = get(caller.data_mut());
                        let r = Host::get_random_u64(host);
                        Ok((r?,))
                    },
                )?;
                Ok(())
            }
        }
    }
}
pub mod exports {
    pub mod wasi {
        pub mod http {
            #[allow(clippy::all)]
            pub mod incoming_handler {
                #[allow(unused_imports)]
                use wasmtime::component::__internal::anyhow;
                pub type IncomingRequest = super::super::super::super::wasi::http::types::IncomingRequest;
                pub type ResponseOutparam = super::super::super::super::wasi::http::types::ResponseOutparam;
                pub struct Guest {
                    handle: wasmtime::component::Func,
                }
                impl Guest {
                    pub fn new(
                        __exports: &mut wasmtime::component::ExportInstance<'_, '_>,
                    ) -> wasmtime::Result<Guest> {
                        let handle = *__exports
                            .typed_func::<
                                (
                                    wasmtime::component::Resource<IncomingRequest>,
                                    wasmtime::component::Resource<ResponseOutparam>,
                                ),
                                (),
                            >("handle")?
                            .func();
                        Ok(Guest { handle })
                    }
                    /// This function is invoked with an incoming HTTP Request, and a resource
                    /// `response-outparam` which provides the capability to reply with an HTTP
                    /// Response. The response is sent by calling the `response-outparam.set`
                    /// method, which allows execution to continue after the response has been
                    /// sent. This enables both streaming to the response body, and performing other
                    /// work.
                    ///
                    /// The implementor of this function must write a response to the
                    /// `response-outparam` before returning, or else the caller will respond
                    /// with an error on its behalf.
                    pub fn call_handle<S: wasmtime::AsContextMut>(
                        &self,
                        mut store: S,
                        arg0: wasmtime::component::Resource<IncomingRequest>,
                        arg1: wasmtime::component::Resource<ResponseOutparam>,
                    ) -> wasmtime::Result<()> {
                        let callee = unsafe {
                            wasmtime::component::TypedFunc::<
                                (
                                    wasmtime::component::Resource<IncomingRequest>,
                                    wasmtime::component::Resource<ResponseOutparam>,
                                ),
                                (),
                            >::new_unchecked(self.handle)
                        };
                        let () = callee.call(store.as_context_mut(), (arg0, arg1))?;
                        callee.post_return(store.as_context_mut())?;
                        Ok(())
                    }
                }
            }
        }
    }
}
const _: &str = "package wasi:io@0.2.0;\n\nworld imports {\n    import streams;\n    import poll;\n}\n";
const _: &str = "package wasi:io@0.2.0;\n\n/// WASI I/O is an I/O abstraction API which is currently focused on providing\n/// stream types.\n///\n/// In the future, the component model is expected to add built-in stream types;\n/// when it does, they are expected to subsume this API.\ninterface streams {\n    use error.{error};\n    use poll.{pollable};\n\n    /// An error for input-stream and output-stream operations.\n    variant stream-error {\n        /// The last operation (a write or flush) failed before completion.\n        ///\n        /// More information is available in the `error` payload.\n        last-operation-failed(error),\n        /// The stream is closed: no more input will be accepted by the\n        /// stream. A closed output-stream will return this error on all\n        /// future operations.\n        closed\n    }\n\n    /// An input bytestream.\n    ///\n    /// `input-stream`s are *non-blocking* to the extent practical on underlying\n    /// platforms. I/O operations always return promptly; if fewer bytes are\n    /// promptly available than requested, they return the number of bytes promptly\n    /// available, which could even be zero. To wait for data to be available,\n    /// use the `subscribe` function to obtain a `pollable` which can be polled\n    /// for using `wasi:io/poll`.\n    resource input-stream {\n        /// Perform a non-blocking read from the stream.\n        ///\n        /// When the source of a `read` is binary data, the bytes from the source\n        /// are returned verbatim. When the source of a `read` is known to the\n        /// implementation to be text, bytes containing the UTF-8 encoding of the\n        /// text are returned.\n        ///\n        /// This function returns a list of bytes containing the read data,\n        /// when successful. The returned list will contain up to `len` bytes;\n        /// it may return fewer than requested, but not more. The list is\n        /// empty when no bytes are available for reading at this time. The\n        /// pollable given by `subscribe` will be ready when more bytes are\n        /// available.\n        ///\n        /// This function fails with a `stream-error` when the operation\n        /// encounters an error, giving `last-operation-failed`, or when the\n        /// stream is closed, giving `closed`.\n        ///\n        /// When the caller gives a `len` of 0, it represents a request to\n        /// read 0 bytes. If the stream is still open, this call should\n        /// succeed and return an empty list, or otherwise fail with `closed`.\n        ///\n        /// The `len` parameter is a `u64`, which could represent a list of u8 which\n        /// is not possible to allocate in wasm32, or not desirable to allocate as\n        /// as a return value by the callee. The callee may return a list of bytes\n        /// less than `len` in size while more bytes are available for reading.\n        read: func(\n            /// The maximum number of bytes to read\n            len: u64\n        ) -> result<list<u8>, stream-error>;\n\n        /// Read bytes from a stream, after blocking until at least one byte can\n        /// be read. Except for blocking, behavior is identical to `read`.\n        blocking-read: func(\n            /// The maximum number of bytes to read\n            len: u64\n        ) -> result<list<u8>, stream-error>;\n\n        /// Skip bytes from a stream. Returns number of bytes skipped.\n        ///\n        /// Behaves identical to `read`, except instead of returning a list\n        /// of bytes, returns the number of bytes consumed from the stream.\n        skip: func(\n            /// The maximum number of bytes to skip.\n            len: u64,\n        ) -> result<u64, stream-error>;\n\n        /// Skip bytes from a stream, after blocking until at least one byte\n        /// can be skipped. Except for blocking behavior, identical to `skip`.\n        blocking-skip: func(\n            /// The maximum number of bytes to skip.\n            len: u64,\n        ) -> result<u64, stream-error>;\n\n        /// Create a `pollable` which will resolve once either the specified stream\n        /// has bytes available to read or the other end of the stream has been\n        /// closed.\n        /// The created `pollable` is a child resource of the `input-stream`.\n        /// Implementations may trap if the `input-stream` is dropped before\n        /// all derived `pollable`s created with this function are dropped.\n        subscribe: func() -> pollable;\n    }\n\n\n    /// An output bytestream.\n    ///\n    /// `output-stream`s are *non-blocking* to the extent practical on\n    /// underlying platforms. Except where specified otherwise, I/O operations also\n    /// always return promptly, after the number of bytes that can be written\n    /// promptly, which could even be zero. To wait for the stream to be ready to\n    /// accept data, the `subscribe` function to obtain a `pollable` which can be\n    /// polled for using `wasi:io/poll`.\n    resource output-stream {\n        /// Check readiness for writing. This function never blocks.\n        ///\n        /// Returns the number of bytes permitted for the next call to `write`,\n        /// or an error. Calling `write` with more bytes than this function has\n        /// permitted will trap.\n        ///\n        /// When this function returns 0 bytes, the `subscribe` pollable will\n        /// become ready when this function will report at least 1 byte, or an\n        /// error.\n        check-write: func() -> result<u64, stream-error>;\n\n        /// Perform a write. This function never blocks.\n        ///\n        /// When the destination of a `write` is binary data, the bytes from\n        /// `contents` are written verbatim. When the destination of a `write` is\n        /// known to the implementation to be text, the bytes of `contents` are\n        /// transcoded from UTF-8 into the encoding of the destination and then\n        /// written.\n        ///\n        /// Precondition: check-write gave permit of Ok(n) and contents has a\n        /// length of less than or equal to n. Otherwise, this function will trap.\n        ///\n        /// returns Err(closed) without writing if the stream has closed since\n        /// the last call to check-write provided a permit.\n        write: func(\n            contents: list<u8>\n        ) -> result<_, stream-error>;\n\n        /// Perform a write of up to 4096 bytes, and then flush the stream. Block\n        /// until all of these operations are complete, or an error occurs.\n        ///\n        /// This is a convenience wrapper around the use of `check-write`,\n        /// `subscribe`, `write`, and `flush`, and is implemented with the\n        /// following pseudo-code:\n        ///\n        /// ```text\n        /// let pollable = this.subscribe();\n        /// while !contents.is_empty() {\n        ///     // Wait for the stream to become writable\n        ///     pollable.block();\n        ///     let Ok(n) = this.check-write(); // eliding error handling\n        ///     let len = min(n, contents.len());\n        ///     let (chunk, rest) = contents.split_at(len);\n        ///     this.write(chunk  );            // eliding error handling\n        ///     contents = rest;\n        /// }\n        /// this.flush();\n        /// // Wait for completion of `flush`\n        /// pollable.block();\n        /// // Check for any errors that arose during `flush`\n        /// let _ = this.check-write();         // eliding error handling\n        /// ```\n        blocking-write-and-flush: func(\n            contents: list<u8>\n        ) -> result<_, stream-error>;\n\n        /// Request to flush buffered output. This function never blocks.\n        ///\n        /// This tells the output-stream that the caller intends any buffered\n        /// output to be flushed. the output which is expected to be flushed\n        /// is all that has been passed to `write` prior to this call.\n        ///\n        /// Upon calling this function, the `output-stream` will not accept any\n        /// writes (`check-write` will return `ok(0)`) until the flush has\n        /// completed. The `subscribe` pollable will become ready when the\n        /// flush has completed and the stream can accept more writes.\n        flush: func() -> result<_, stream-error>;\n\n        /// Request to flush buffered output, and block until flush completes\n        /// and stream is ready for writing again.\n        blocking-flush: func() -> result<_, stream-error>;\n\n        /// Create a `pollable` which will resolve once the output-stream\n        /// is ready for more writing, or an error has occured. When this\n        /// pollable is ready, `check-write` will return `ok(n)` with n>0, or an\n        /// error.\n        ///\n        /// If the stream is closed, this pollable is always ready immediately.\n        ///\n        /// The created `pollable` is a child resource of the `output-stream`.\n        /// Implementations may trap if the `output-stream` is dropped before\n        /// all derived `pollable`s created with this function are dropped.\n        subscribe: func() -> pollable;\n\n        /// Write zeroes to a stream.\n        ///\n        /// This should be used precisely like `write` with the exact same\n        /// preconditions (must use check-write first), but instead of\n        /// passing a list of bytes, you simply pass the number of zero-bytes\n        /// that should be written.\n        write-zeroes: func(\n            /// The number of zero-bytes to write\n            len: u64\n        ) -> result<_, stream-error>;\n\n        /// Perform a write of up to 4096 zeroes, and then flush the stream.\n        /// Block until all of these operations are complete, or an error\n        /// occurs.\n        ///\n        /// This is a convenience wrapper around the use of `check-write`,\n        /// `subscribe`, `write-zeroes`, and `flush`, and is implemented with\n        /// the following pseudo-code:\n        ///\n        /// ```text\n        /// let pollable = this.subscribe();\n        /// while num_zeroes != 0 {\n        ///     // Wait for the stream to become writable\n        ///     pollable.block();\n        ///     let Ok(n) = this.check-write(); // eliding error handling\n        ///     let len = min(n, num_zeroes);\n        ///     this.write-zeroes(len);         // eliding error handling\n        ///     num_zeroes -= len;\n        /// }\n        /// this.flush();\n        /// // Wait for completion of `flush`\n        /// pollable.block();\n        /// // Check for any errors that arose during `flush`\n        /// let _ = this.check-write();         // eliding error handling\n        /// ```\n        blocking-write-zeroes-and-flush: func(\n            /// The number of zero-bytes to write\n            len: u64\n        ) -> result<_, stream-error>;\n\n        /// Read from one stream and write to another.\n        ///\n        /// The behavior of splice is equivelant to:\n        /// 1. calling `check-write` on the `output-stream`\n        /// 2. calling `read` on the `input-stream` with the smaller of the\n        /// `check-write` permitted length and the `len` provided to `splice`\n        /// 3. calling `write` on the `output-stream` with that read data.\n        ///\n        /// Any error reported by the call to `check-write`, `read`, or\n        /// `write` ends the splice and reports that error.\n        ///\n        /// This function returns the number of bytes transferred; it may be less\n        /// than `len`.\n        splice: func(\n            /// The stream to read from\n            src: borrow<input-stream>,\n            /// The number of bytes to splice\n            len: u64,\n        ) -> result<u64, stream-error>;\n\n        /// Read from one stream and write to another, with blocking.\n        ///\n        /// This is similar to `splice`, except that it blocks until the\n        /// `output-stream` is ready for writing, and the `input-stream`\n        /// is ready for reading, before performing the `splice`.\n        blocking-splice: func(\n            /// The stream to read from\n            src: borrow<input-stream>,\n            /// The number of bytes to splice\n            len: u64,\n        ) -> result<u64, stream-error>;\n    }\n}\n";
const _: &str = "package wasi:io@0.2.0;\n\n\ninterface error {\n    /// A resource which represents some error information.\n    ///\n    /// The only method provided by this resource is `to-debug-string`,\n    /// which provides some human-readable information about the error.\n    ///\n    /// In the `wasi:io` package, this resource is returned through the\n    /// `wasi:io/streams/stream-error` type.\n    ///\n    /// To provide more specific error information, other interfaces may\n    /// provide functions to further \"downcast\" this error into more specific\n    /// error information. For example, `error`s returned in streams derived\n    /// from filesystem types to be described using the filesystem\'s own\n    /// error-code type, using the function\n    /// `wasi:filesystem/types/filesystem-error-code`, which takes a parameter\n    /// `borrow<error>` and returns\n    /// `option<wasi:filesystem/types/error-code>`.\n    ///\n    /// The set of functions which can \"downcast\" an `error` into a more\n    /// concrete type is open.\n    resource error {\n        /// Returns a string that is suitable to assist humans in debugging\n        /// this error.\n        ///\n        /// WARNING: The returned string should not be consumed mechanically!\n        /// It may change across platforms, hosts, or other implementation\n        /// details. Parsing this string is a major platform-compatibility\n        /// hazard.\n        to-debug-string: func() -> string;\n    }\n}\n";
const _: &str = "package wasi:io@0.2.0;\n\n/// A poll API intended to let users wait for I/O events on multiple handles\n/// at once.\ninterface poll {\n    /// `pollable` represents a single I/O event which may be ready, or not.\n    resource pollable {\n\n      /// Return the readiness of a pollable. This function never blocks.\n      ///\n      /// Returns `true` when the pollable is ready, and `false` otherwise.\n      ready: func() -> bool;\n\n      /// `block` returns immediately if the pollable is ready, and otherwise\n      /// blocks until ready.\n      ///\n      /// This function is equivalent to calling `poll.poll` on a list\n      /// containing only this pollable.\n      block: func();\n    }\n\n    /// Poll for completion on a set of pollables.\n    ///\n    /// This function takes a list of pollables, which identify I/O sources of\n    /// interest, and waits until one or more of the events is ready for I/O.\n    ///\n    /// The result `list<u32>` contains one or more indices of handles in the\n    /// argument list that is ready for I/O.\n    ///\n    /// If the list contains more elements than can be indexed with a `u32`\n    /// value, this function traps.\n    ///\n    /// A timeout can be implemented by adding a pollable from the\n    /// wasi-clocks API to the list.\n    ///\n    /// This function does not return a `result`; polling in itself does not\n    /// do any I/O so it doesn\'t fail. If any of the I/O sources identified by\n    /// the pollables has an error, it is indicated by marking the source as\n    /// being reaedy for I/O.\n    poll: func(in: list<borrow<pollable>>) -> list<u32>;\n}\n";
const _: &str = "package wasi:clocks@0.2.0;\n\nworld imports {\n    import monotonic-clock;\n    import wall-clock;\n}\n";
const _: &str = "package wasi:clocks@0.2.0;\n/// WASI Wall Clock is a clock API intended to let users query the current\n/// time. The name \"wall\" makes an analogy to a \"clock on the wall\", which\n/// is not necessarily monotonic as it may be reset.\n///\n/// It is intended to be portable at least between Unix-family platforms and\n/// Windows.\n///\n/// A wall clock is a clock which measures the date and time according to\n/// some external reference.\n///\n/// External references may be reset, so this clock is not necessarily\n/// monotonic, making it unsuitable for measuring elapsed time.\n///\n/// It is intended for reporting the current date and time for humans.\ninterface wall-clock {\n    /// A time and date in seconds plus nanoseconds.\n    record datetime {\n        seconds: u64,\n        nanoseconds: u32,\n    }\n\n    /// Read the current value of the clock.\n    ///\n    /// This clock is not monotonic, therefore calling this function repeatedly\n    /// will not necessarily produce a sequence of non-decreasing values.\n    ///\n    /// The returned timestamps represent the number of seconds since\n    /// 1970-01-01T00:00:00Z, also known as [POSIX\'s Seconds Since the Epoch],\n    /// also known as [Unix Time].\n    ///\n    /// The nanoseconds field of the output is always less than 1000000000.\n    ///\n    /// [POSIX\'s Seconds Since the Epoch]: https://pubs.opengroup.org/onlinepubs/9699919799/xrat/V4_xbd_chap04.html#tag_21_04_16\n    /// [Unix Time]: https://en.wikipedia.org/wiki/Unix_time\n    now: func() -> datetime;\n\n    /// Query the resolution of the clock.\n    ///\n    /// The nanoseconds field of the output is always less than 1000000000.\n    resolution: func() -> datetime;\n}\n";
const _: &str = "package wasi:clocks@0.2.0;\n/// WASI Monotonic Clock is a clock API intended to let users measure elapsed\n/// time.\n///\n/// It is intended to be portable at least between Unix-family platforms and\n/// Windows.\n///\n/// A monotonic clock is a clock which has an unspecified initial value, and\n/// successive reads of the clock will produce non-decreasing values.\n///\n/// It is intended for measuring elapsed time.\ninterface monotonic-clock {\n    use wasi:io/poll@0.2.0.{pollable};\n\n    /// An instant in time, in nanoseconds. An instant is relative to an\n    /// unspecified initial value, and can only be compared to instances from\n    /// the same monotonic-clock.\n    type instant = u64;\n\n    /// A duration of time, in nanoseconds.\n    type duration = u64;\n\n    /// Read the current value of the clock.\n    ///\n    /// The clock is monotonic, therefore calling this function repeatedly will\n    /// produce a sequence of non-decreasing values.\n    now: func() -> instant;\n\n    /// Query the resolution of the clock. Returns the duration of time\n    /// corresponding to a clock tick.\n    resolution: func() -> duration;\n\n    /// Create a `pollable` which will resolve once the specified instant\n    /// occured.\n    subscribe-instant: func(\n        when: instant,\n    ) -> pollable;\n\n    /// Create a `pollable` which will resolve once the given duration has\n    /// elapsed, starting at the time at which this function was called.\n    /// occured.\n    subscribe-duration: func(\n        when: duration,\n    ) -> pollable;\n}\n";
const _: &str = "package wasi:random@0.2.0;\n\nworld imports {\n    import random;\n    import insecure;\n    import insecure-seed;\n}\n";
const _: &str = "package wasi:random@0.2.0;\n/// The insecure interface for insecure pseudo-random numbers.\n///\n/// It is intended to be portable at least between Unix-family platforms and\n/// Windows.\ninterface insecure {\n    /// Return `len` insecure pseudo-random bytes.\n    ///\n    /// This function is not cryptographically secure. Do not use it for\n    /// anything related to security.\n    ///\n    /// There are no requirements on the values of the returned bytes, however\n    /// implementations are encouraged to return evenly distributed values with\n    /// a long period.\n    get-insecure-random-bytes: func(len: u64) -> list<u8>;\n\n    /// Return an insecure pseudo-random `u64` value.\n    ///\n    /// This function returns the same type of pseudo-random data as\n    /// `get-insecure-random-bytes`, represented as a `u64`.\n    get-insecure-random-u64: func() -> u64;\n}\n";
const _: &str = "package wasi:random@0.2.0;\n/// The insecure-seed interface for seeding hash-map DoS resistance.\n///\n/// It is intended to be portable at least between Unix-family platforms and\n/// Windows.\ninterface insecure-seed {\n    /// Return a 128-bit value that may contain a pseudo-random value.\n    ///\n    /// The returned value is not required to be computed from a CSPRNG, and may\n    /// even be entirely deterministic. Host implementations are encouraged to\n    /// provide pseudo-random values to any program exposed to\n    /// attacker-controlled content, to enable DoS protection built into many\n    /// languages\' hash-map implementations.\n    ///\n    /// This function is intended to only be called once, by a source language\n    /// to initialize Denial Of Service (DoS) protection in its hash-map\n    /// implementation.\n    ///\n    /// # Expected future evolution\n    ///\n    /// This will likely be changed to a value import, to prevent it from being\n    /// called multiple times and potentially used for purposes other than DoS\n    /// protection.\n    insecure-seed: func() -> tuple<u64, u64>;\n}\n";
const _: &str = "package wasi:random@0.2.0;\n/// WASI Random is a random data API.\n///\n/// It is intended to be portable at least between Unix-family platforms and\n/// Windows.\ninterface random {\n    /// Return `len` cryptographically-secure random or pseudo-random bytes.\n    ///\n    /// This function must produce data at least as cryptographically secure and\n    /// fast as an adequately seeded cryptographically-secure pseudo-random\n    /// number generator (CSPRNG). It must not block, from the perspective of\n    /// the calling program, under any circumstances, including on the first\n    /// request and on requests for numbers of bytes. The returned data must\n    /// always be unpredictable.\n    ///\n    /// This function must always return fresh data. Deterministic environments\n    /// must omit this function, rather than implementing it with deterministic\n    /// data.\n    get-random-bytes: func(len: u64) -> list<u8>;\n\n    /// Return a cryptographically-secure random or pseudo-random `u64` value.\n    ///\n    /// This function returns the same type of data as `get-random-bytes`,\n    /// represented as a `u64`.\n    get-random-u64: func() -> u64;\n}\n";
const _: &str = "package wasi:filesystem@0.2.0;\n\nworld imports {\n    import types;\n    import preopens;\n}\n";
const _: &str = "package wasi:filesystem@0.2.0;\n\ninterface preopens {\n    use types.{descriptor};\n\n    /// Return the set of preopened directories, and their path.\n    get-directories: func() -> list<tuple<descriptor, string>>;\n}\n";
const _: &str = "package wasi:filesystem@0.2.0;\n/// WASI filesystem is a filesystem API primarily intended to let users run WASI\n/// programs that access their files on their existing filesystems, without\n/// significant overhead.\n///\n/// It is intended to be roughly portable between Unix-family platforms and\n/// Windows, though it does not hide many of the major differences.\n///\n/// Paths are passed as interface-type `string`s, meaning they must consist of\n/// a sequence of Unicode Scalar Values (USVs). Some filesystems may contain\n/// paths which are not accessible by this API.\n///\n/// The directory separator in WASI is always the forward-slash (`/`).\n///\n/// All paths in WASI are relative paths, and are interpreted relative to a\n/// `descriptor` referring to a base directory. If a `path` argument to any WASI\n/// function starts with `/`, or if any step of resolving a `path`, including\n/// `..` and symbolic link steps, reaches a directory outside of the base\n/// directory, or reaches a symlink to an absolute or rooted path in the\n/// underlying filesystem, the function fails with `error-code::not-permitted`.\n///\n/// For more information about WASI path resolution and sandboxing, see\n/// [WASI filesystem path resolution].\n///\n/// [WASI filesystem path resolution]: https://github.com/WebAssembly/wasi-filesystem/blob/main/path-resolution.md\ninterface types {\n    use wasi:io/streams@0.2.0.{input-stream, output-stream, error};\n    use wasi:clocks/wall-clock@0.2.0.{datetime};\n\n    /// File size or length of a region within a file.\n    type filesize = u64;\n\n    /// The type of a filesystem object referenced by a descriptor.\n    ///\n    /// Note: This was called `filetype` in earlier versions of WASI.\n    enum descriptor-type {\n        /// The type of the descriptor or file is unknown or is different from\n        /// any of the other types specified.\n        unknown,\n        /// The descriptor refers to a block device inode.\n        block-device,\n        /// The descriptor refers to a character device inode.\n        character-device,\n        /// The descriptor refers to a directory inode.\n        directory,\n        /// The descriptor refers to a named pipe.\n        fifo,\n        /// The file refers to a symbolic link inode.\n        symbolic-link,\n        /// The descriptor refers to a regular file inode.\n        regular-file,\n        /// The descriptor refers to a socket.\n        socket,\n    }\n\n    /// Descriptor flags.\n    ///\n    /// Note: This was called `fdflags` in earlier versions of WASI.\n    flags descriptor-flags {\n        /// Read mode: Data can be read.\n        read,\n        /// Write mode: Data can be written to.\n        write,\n        /// Request that writes be performed according to synchronized I/O file\n        /// integrity completion. The data stored in the file and the file\'s\n        /// metadata are synchronized. This is similar to `O_SYNC` in POSIX.\n        ///\n        /// The precise semantics of this operation have not yet been defined for\n        /// WASI. At this time, it should be interpreted as a request, and not a\n        /// requirement.\n        file-integrity-sync,\n        /// Request that writes be performed according to synchronized I/O data\n        /// integrity completion. Only the data stored in the file is\n        /// synchronized. This is similar to `O_DSYNC` in POSIX.\n        ///\n        /// The precise semantics of this operation have not yet been defined for\n        /// WASI. At this time, it should be interpreted as a request, and not a\n        /// requirement.\n        data-integrity-sync,\n        /// Requests that reads be performed at the same level of integrety\n        /// requested for writes. This is similar to `O_RSYNC` in POSIX.\n        ///\n        /// The precise semantics of this operation have not yet been defined for\n        /// WASI. At this time, it should be interpreted as a request, and not a\n        /// requirement.\n        requested-write-sync,\n        /// Mutating directories mode: Directory contents may be mutated.\n        ///\n        /// When this flag is unset on a descriptor, operations using the\n        /// descriptor which would create, rename, delete, modify the data or\n        /// metadata of filesystem objects, or obtain another handle which\n        /// would permit any of those, shall fail with `error-code::read-only` if\n        /// they would otherwise succeed.\n        ///\n        /// This may only be set on directories.\n        mutate-directory,\n    }\n\n    /// File attributes.\n    ///\n    /// Note: This was called `filestat` in earlier versions of WASI.\n    record descriptor-stat {\n        /// File type.\n        %type: descriptor-type,\n        /// Number of hard links to the file.\n        link-count: link-count,\n        /// For regular files, the file size in bytes. For symbolic links, the\n        /// length in bytes of the pathname contained in the symbolic link.\n        size: filesize,\n        /// Last data access timestamp.\n        ///\n        /// If the `option` is none, the platform doesn\'t maintain an access\n        /// timestamp for this file.\n        data-access-timestamp: option<datetime>,\n        /// Last data modification timestamp.\n        ///\n        /// If the `option` is none, the platform doesn\'t maintain a\n        /// modification timestamp for this file.\n        data-modification-timestamp: option<datetime>,\n        /// Last file status-change timestamp.\n        ///\n        /// If the `option` is none, the platform doesn\'t maintain a\n        /// status-change timestamp for this file.\n        status-change-timestamp: option<datetime>,\n    }\n\n    /// Flags determining the method of how paths are resolved.\n    flags path-flags {\n        /// As long as the resolved path corresponds to a symbolic link, it is\n        /// expanded.\n        symlink-follow,\n    }\n\n    /// Open flags used by `open-at`.\n    flags open-flags {\n        /// Create file if it does not exist, similar to `O_CREAT` in POSIX.\n        create,\n        /// Fail if not a directory, similar to `O_DIRECTORY` in POSIX.\n        directory,\n        /// Fail if file already exists, similar to `O_EXCL` in POSIX.\n        exclusive,\n        /// Truncate file to size 0, similar to `O_TRUNC` in POSIX.\n        truncate,\n    }\n\n    /// Number of hard links to an inode.\n    type link-count = u64;\n\n    /// When setting a timestamp, this gives the value to set it to.\n    variant new-timestamp {\n        /// Leave the timestamp set to its previous value.\n        no-change,\n        /// Set the timestamp to the current time of the system clock associated\n        /// with the filesystem.\n        now,\n        /// Set the timestamp to the given value.\n        timestamp(datetime),\n    }\n\n    /// A directory entry.\n    record directory-entry {\n        /// The type of the file referred to by this directory entry.\n        %type: descriptor-type,\n\n        /// The name of the object.\n        name: string,\n    }\n\n    /// Error codes returned by functions, similar to `errno` in POSIX.\n    /// Not all of these error codes are returned by the functions provided by this\n    /// API; some are used in higher-level library layers, and others are provided\n    /// merely for alignment with POSIX.\n    enum error-code {\n        /// Permission denied, similar to `EACCES` in POSIX.\n        access,\n        /// Resource unavailable, or operation would block, similar to `EAGAIN` and `EWOULDBLOCK` in POSIX.\n        would-block,\n        /// Connection already in progress, similar to `EALREADY` in POSIX.\n        already,\n        /// Bad descriptor, similar to `EBADF` in POSIX.\n        bad-descriptor,\n        /// Device or resource busy, similar to `EBUSY` in POSIX.\n        busy,\n        /// Resource deadlock would occur, similar to `EDEADLK` in POSIX.\n        deadlock,\n        /// Storage quota exceeded, similar to `EDQUOT` in POSIX.\n        quota,\n        /// File exists, similar to `EEXIST` in POSIX.\n        exist,\n        /// File too large, similar to `EFBIG` in POSIX.\n        file-too-large,\n        /// Illegal byte sequence, similar to `EILSEQ` in POSIX.\n        illegal-byte-sequence,\n        /// Operation in progress, similar to `EINPROGRESS` in POSIX.\n        in-progress,\n        /// Interrupted function, similar to `EINTR` in POSIX.\n        interrupted,\n        /// Invalid argument, similar to `EINVAL` in POSIX.\n        invalid,\n        /// I/O error, similar to `EIO` in POSIX.\n        io,\n        /// Is a directory, similar to `EISDIR` in POSIX.\n        is-directory,\n        /// Too many levels of symbolic links, similar to `ELOOP` in POSIX.\n        loop,\n        /// Too many links, similar to `EMLINK` in POSIX.\n        too-many-links,\n        /// Message too large, similar to `EMSGSIZE` in POSIX.\n        message-size,\n        /// Filename too long, similar to `ENAMETOOLONG` in POSIX.\n        name-too-long,\n        /// No such device, similar to `ENODEV` in POSIX.\n        no-device,\n        /// No such file or directory, similar to `ENOENT` in POSIX.\n        no-entry,\n        /// No locks available, similar to `ENOLCK` in POSIX.\n        no-lock,\n        /// Not enough space, similar to `ENOMEM` in POSIX.\n        insufficient-memory,\n        /// No space left on device, similar to `ENOSPC` in POSIX.\n        insufficient-space,\n        /// Not a directory or a symbolic link to a directory, similar to `ENOTDIR` in POSIX.\n        not-directory,\n        /// Directory not empty, similar to `ENOTEMPTY` in POSIX.\n        not-empty,\n        /// State not recoverable, similar to `ENOTRECOVERABLE` in POSIX.\n        not-recoverable,\n        /// Not supported, similar to `ENOTSUP` and `ENOSYS` in POSIX.\n        unsupported,\n        /// Inappropriate I/O control operation, similar to `ENOTTY` in POSIX.\n        no-tty,\n        /// No such device or address, similar to `ENXIO` in POSIX.\n        no-such-device,\n        /// Value too large to be stored in data type, similar to `EOVERFLOW` in POSIX.\n        overflow,\n        /// Operation not permitted, similar to `EPERM` in POSIX.\n        not-permitted,\n        /// Broken pipe, similar to `EPIPE` in POSIX.\n        pipe,\n        /// Read-only file system, similar to `EROFS` in POSIX.\n        read-only,\n        /// Invalid seek, similar to `ESPIPE` in POSIX.\n        invalid-seek,\n        /// Text file busy, similar to `ETXTBSY` in POSIX.\n        text-file-busy,\n        /// Cross-device link, similar to `EXDEV` in POSIX.\n        cross-device,\n    }\n\n    /// File or memory access pattern advisory information.\n    enum advice {\n        /// The application has no advice to give on its behavior with respect\n        /// to the specified data.\n        normal,\n        /// The application expects to access the specified data sequentially\n        /// from lower offsets to higher offsets.\n        sequential,\n        /// The application expects to access the specified data in a random\n        /// order.\n        random,\n        /// The application expects to access the specified data in the near\n        /// future.\n        will-need,\n        /// The application expects that it will not access the specified data\n        /// in the near future.\n        dont-need,\n        /// The application expects to access the specified data once and then\n        /// not reuse it thereafter.\n        no-reuse,\n    }\n\n    /// A 128-bit hash value, split into parts because wasm doesn\'t have a\n    /// 128-bit integer type.\n    record metadata-hash-value {\n       /// 64 bits of a 128-bit hash value.\n       lower: u64,\n       /// Another 64 bits of a 128-bit hash value.\n       upper: u64,\n    }\n\n    /// A descriptor is a reference to a filesystem object, which may be a file,\n    /// directory, named pipe, special file, or other object on which filesystem\n    /// calls may be made.\n    resource descriptor {\n        /// Return a stream for reading from a file, if available.\n        ///\n        /// May fail with an error-code describing why the file cannot be read.\n        ///\n        /// Multiple read, write, and append streams may be active on the same open\n        /// file and they do not interfere with each other.\n        ///\n        /// Note: This allows using `read-stream`, which is similar to `read` in POSIX.\n        read-via-stream: func(\n            /// The offset within the file at which to start reading.\n            offset: filesize,\n        ) -> result<input-stream, error-code>;\n\n        /// Return a stream for writing to a file, if available.\n        ///\n        /// May fail with an error-code describing why the file cannot be written.\n        ///\n        /// Note: This allows using `write-stream`, which is similar to `write` in\n        /// POSIX.\n        write-via-stream: func(\n            /// The offset within the file at which to start writing.\n            offset: filesize,\n        ) -> result<output-stream, error-code>;\n\n        /// Return a stream for appending to a file, if available.\n        ///\n        /// May fail with an error-code describing why the file cannot be appended.\n        ///\n        /// Note: This allows using `write-stream`, which is similar to `write` with\n        /// `O_APPEND` in in POSIX.\n        append-via-stream: func() -> result<output-stream, error-code>;\n\n        /// Provide file advisory information on a descriptor.\n        ///\n        /// This is similar to `posix_fadvise` in POSIX.\n        advise: func(\n            /// The offset within the file to which the advisory applies.\n            offset: filesize,\n            /// The length of the region to which the advisory applies.\n            length: filesize,\n            /// The advice.\n            advice: advice\n        ) -> result<_, error-code>;\n\n        /// Synchronize the data of a file to disk.\n        ///\n        /// This function succeeds with no effect if the file descriptor is not\n        /// opened for writing.\n        ///\n        /// Note: This is similar to `fdatasync` in POSIX.\n        sync-data: func() -> result<_, error-code>;\n\n        /// Get flags associated with a descriptor.\n        ///\n        /// Note: This returns similar flags to `fcntl(fd, F_GETFL)` in POSIX.\n        ///\n        /// Note: This returns the value that was the `fs_flags` value returned\n        /// from `fdstat_get` in earlier versions of WASI.\n        get-flags: func() -> result<descriptor-flags, error-code>;\n\n        /// Get the dynamic type of a descriptor.\n        ///\n        /// Note: This returns the same value as the `type` field of the `fd-stat`\n        /// returned by `stat`, `stat-at` and similar.\n        ///\n        /// Note: This returns similar flags to the `st_mode & S_IFMT` value provided\n        /// by `fstat` in POSIX.\n        ///\n        /// Note: This returns the value that was the `fs_filetype` value returned\n        /// from `fdstat_get` in earlier versions of WASI.\n        get-type: func() -> result<descriptor-type, error-code>;\n\n        /// Adjust the size of an open file. If this increases the file\'s size, the\n        /// extra bytes are filled with zeros.\n        ///\n        /// Note: This was called `fd_filestat_set_size` in earlier versions of WASI.\n        set-size: func(size: filesize) -> result<_, error-code>;\n\n        /// Adjust the timestamps of an open file or directory.\n        ///\n        /// Note: This is similar to `futimens` in POSIX.\n        ///\n        /// Note: This was called `fd_filestat_set_times` in earlier versions of WASI.\n        set-times: func(\n            /// The desired values of the data access timestamp.\n            data-access-timestamp: new-timestamp,\n            /// The desired values of the data modification timestamp.\n            data-modification-timestamp: new-timestamp,\n        ) -> result<_, error-code>;\n\n        /// Read from a descriptor, without using and updating the descriptor\'s offset.\n        ///\n        /// This function returns a list of bytes containing the data that was\n        /// read, along with a bool which, when true, indicates that the end of the\n        /// file was reached. The returned list will contain up to `length` bytes; it\n        /// may return fewer than requested, if the end of the file is reached or\n        /// if the I/O operation is interrupted.\n        ///\n        /// In the future, this may change to return a `stream<u8, error-code>`.\n        ///\n        /// Note: This is similar to `pread` in POSIX.\n        read: func(\n            /// The maximum number of bytes to read.\n            length: filesize,\n            /// The offset within the file at which to read.\n            offset: filesize,\n        ) -> result<tuple<list<u8>, bool>, error-code>;\n\n        /// Write to a descriptor, without using and updating the descriptor\'s offset.\n        ///\n        /// It is valid to write past the end of a file; the file is extended to the\n        /// extent of the write, with bytes between the previous end and the start of\n        /// the write set to zero.\n        ///\n        /// In the future, this may change to take a `stream<u8, error-code>`.\n        ///\n        /// Note: This is similar to `pwrite` in POSIX.\n        write: func(\n            /// Data to write\n            buffer: list<u8>,\n            /// The offset within the file at which to write.\n            offset: filesize,\n        ) -> result<filesize, error-code>;\n\n        /// Read directory entries from a directory.\n        ///\n        /// On filesystems where directories contain entries referring to themselves\n        /// and their parents, often named `.` and `..` respectively, these entries\n        /// are omitted.\n        ///\n        /// This always returns a new stream which starts at the beginning of the\n        /// directory. Multiple streams may be active on the same directory, and they\n        /// do not interfere with each other.\n        read-directory: func() -> result<directory-entry-stream, error-code>;\n\n        /// Synchronize the data and metadata of a file to disk.\n        ///\n        /// This function succeeds with no effect if the file descriptor is not\n        /// opened for writing.\n        ///\n        /// Note: This is similar to `fsync` in POSIX.\n        sync: func() -> result<_, error-code>;\n\n        /// Create a directory.\n        ///\n        /// Note: This is similar to `mkdirat` in POSIX.\n        create-directory-at: func(\n            /// The relative path at which to create the directory.\n            path: string,\n        ) -> result<_, error-code>;\n\n        /// Return the attributes of an open file or directory.\n        ///\n        /// Note: This is similar to `fstat` in POSIX, except that it does not return\n        /// device and inode information. For testing whether two descriptors refer to\n        /// the same underlying filesystem object, use `is-same-object`. To obtain\n        /// additional data that can be used do determine whether a file has been\n        /// modified, use `metadata-hash`.\n        ///\n        /// Note: This was called `fd_filestat_get` in earlier versions of WASI.\n        stat: func() -> result<descriptor-stat, error-code>;\n\n        /// Return the attributes of a file or directory.\n        ///\n        /// Note: This is similar to `fstatat` in POSIX, except that it does not\n        /// return device and inode information. See the `stat` description for a\n        /// discussion of alternatives.\n        ///\n        /// Note: This was called `path_filestat_get` in earlier versions of WASI.\n        stat-at: func(\n            /// Flags determining the method of how the path is resolved.\n            path-flags: path-flags,\n            /// The relative path of the file or directory to inspect.\n            path: string,\n        ) -> result<descriptor-stat, error-code>;\n\n        /// Adjust the timestamps of a file or directory.\n        ///\n        /// Note: This is similar to `utimensat` in POSIX.\n        ///\n        /// Note: This was called `path_filestat_set_times` in earlier versions of\n        /// WASI.\n        set-times-at: func(\n            /// Flags determining the method of how the path is resolved.\n            path-flags: path-flags,\n            /// The relative path of the file or directory to operate on.\n            path: string,\n            /// The desired values of the data access timestamp.\n            data-access-timestamp: new-timestamp,\n            /// The desired values of the data modification timestamp.\n            data-modification-timestamp: new-timestamp,\n        ) -> result<_, error-code>;\n\n        /// Create a hard link.\n        ///\n        /// Note: This is similar to `linkat` in POSIX.\n        link-at: func(\n            /// Flags determining the method of how the path is resolved.\n            old-path-flags: path-flags,\n            /// The relative source path from which to link.\n            old-path: string,\n            /// The base directory for `new-path`.\n            new-descriptor: borrow<descriptor>,\n            /// The relative destination path at which to create the hard link.\n            new-path: string,\n        ) -> result<_, error-code>;\n\n        /// Open a file or directory.\n        ///\n        /// The returned descriptor is not guaranteed to be the lowest-numbered\n        /// descriptor not currently open/ it is randomized to prevent applications\n        /// from depending on making assumptions about indexes, since this is\n        /// error-prone in multi-threaded contexts. The returned descriptor is\n        /// guaranteed to be less than 2**31.\n        ///\n        /// If `flags` contains `descriptor-flags::mutate-directory`, and the base\n        /// descriptor doesn\'t have `descriptor-flags::mutate-directory` set,\n        /// `open-at` fails with `error-code::read-only`.\n        ///\n        /// If `flags` contains `write` or `mutate-directory`, or `open-flags`\n        /// contains `truncate` or `create`, and the base descriptor doesn\'t have\n        /// `descriptor-flags::mutate-directory` set, `open-at` fails with\n        /// `error-code::read-only`.\n        ///\n        /// Note: This is similar to `openat` in POSIX.\n        open-at: func(\n            /// Flags determining the method of how the path is resolved.\n            path-flags: path-flags,\n            /// The relative path of the object to open.\n            path: string,\n            /// The method by which to open the file.\n            open-flags: open-flags,\n            /// Flags to use for the resulting descriptor.\n            %flags: descriptor-flags,\n        ) -> result<descriptor, error-code>;\n\n        /// Read the contents of a symbolic link.\n        ///\n        /// If the contents contain an absolute or rooted path in the underlying\n        /// filesystem, this function fails with `error-code::not-permitted`.\n        ///\n        /// Note: This is similar to `readlinkat` in POSIX.\n        readlink-at: func(\n            /// The relative path of the symbolic link from which to read.\n            path: string,\n        ) -> result<string, error-code>;\n\n        /// Remove a directory.\n        ///\n        /// Return `error-code::not-empty` if the directory is not empty.\n        ///\n        /// Note: This is similar to `unlinkat(fd, path, AT_REMOVEDIR)` in POSIX.\n        remove-directory-at: func(\n            /// The relative path to a directory to remove.\n            path: string,\n        ) -> result<_, error-code>;\n\n        /// Rename a filesystem object.\n        ///\n        /// Note: This is similar to `renameat` in POSIX.\n        rename-at: func(\n            /// The relative source path of the file or directory to rename.\n            old-path: string,\n            /// The base directory for `new-path`.\n            new-descriptor: borrow<descriptor>,\n            /// The relative destination path to which to rename the file or directory.\n            new-path: string,\n        ) -> result<_, error-code>;\n\n        /// Create a symbolic link (also known as a \"symlink\").\n        ///\n        /// If `old-path` starts with `/`, the function fails with\n        /// `error-code::not-permitted`.\n        ///\n        /// Note: This is similar to `symlinkat` in POSIX.\n        symlink-at: func(\n            /// The contents of the symbolic link.\n            old-path: string,\n            /// The relative destination path at which to create the symbolic link.\n            new-path: string,\n        ) -> result<_, error-code>;\n\n        /// Unlink a filesystem object that is not a directory.\n        ///\n        /// Return `error-code::is-directory` if the path refers to a directory.\n        /// Note: This is similar to `unlinkat(fd, path, 0)` in POSIX.\n        unlink-file-at: func(\n            /// The relative path to a file to unlink.\n            path: string,\n        ) -> result<_, error-code>;\n\n        /// Test whether two descriptors refer to the same filesystem object.\n        ///\n        /// In POSIX, this corresponds to testing whether the two descriptors have the\n        /// same device (`st_dev`) and inode (`st_ino` or `d_ino`) numbers.\n        /// wasi-filesystem does not expose device and inode numbers, so this function\n        /// may be used instead.\n        is-same-object: func(other: borrow<descriptor>) -> bool;\n\n        /// Return a hash of the metadata associated with a filesystem object referred\n        /// to by a descriptor.\n        ///\n        /// This returns a hash of the last-modification timestamp and file size, and\n        /// may also include the inode number, device number, birth timestamp, and\n        /// other metadata fields that may change when the file is modified or\n        /// replaced. It may also include a secret value chosen by the\n        /// implementation and not otherwise exposed.\n        ///\n        /// Implementations are encourated to provide the following properties:\n        ///\n        ///  - If the file is not modified or replaced, the computed hash value should\n        ///    usually not change.\n        ///  - If the object is modified or replaced, the computed hash value should\n        ///    usually change.\n        ///  - The inputs to the hash should not be easily computable from the\n        ///    computed hash.\n        ///\n        /// However, none of these is required.\n        metadata-hash: func() -> result<metadata-hash-value, error-code>;\n\n        /// Return a hash of the metadata associated with a filesystem object referred\n        /// to by a directory descriptor and a relative path.\n        ///\n        /// This performs the same hash computation as `metadata-hash`.\n        metadata-hash-at: func(\n            /// Flags determining the method of how the path is resolved.\n            path-flags: path-flags,\n            /// The relative path of the file or directory to inspect.\n            path: string,\n        ) -> result<metadata-hash-value, error-code>;\n    }\n\n    /// A stream of directory entries.\n    resource directory-entry-stream {\n        /// Read a single directory entry from a `directory-entry-stream`.\n        read-directory-entry: func() -> result<option<directory-entry>, error-code>;\n    }\n\n    /// Attempts to extract a filesystem-related `error-code` from the stream\n    /// `error` provided.\n    ///\n    /// Stream operations which return `stream-error::last-operation-failed`\n    /// have a payload with more information about the operation that failed.\n    /// This payload can be passed through to this function to see if there\'s\n    /// filesystem-related information about the error to return.\n    ///\n    /// Note that this function is fallible because not all stream-related\n    /// errors are filesystem-related errors.\n    filesystem-error-code: func(err: borrow<error>) -> option<error-code>;\n}\n";
const _: &str = "\n/// This interface provides a value-export of the default network handle..\ninterface instance-network {\n    use network.{network};\n\n    /// Get a handle to the default network.\n    instance-network: func() -> network;\n\n}\n";
const _: &str = "package wasi:sockets@0.2.0;\n\nworld imports {\n    import instance-network;\n    import network;\n    import udp;\n    import udp-create-socket;\n    import tcp;\n    import tcp-create-socket;\n    import ip-name-lookup;\n}\n";
const _: &str = "\ninterface ip-name-lookup {\n    use wasi:io/poll@0.2.0.{pollable};\n    use network.{network, error-code, ip-address};\n\n\n    /// Resolve an internet host name to a list of IP addresses.\n    ///\n    /// Unicode domain names are automatically converted to ASCII using IDNA encoding.\n    /// If the input is an IP address string, the address is parsed and returned\n    /// as-is without making any external requests.\n    ///\n    /// See the wasi-socket proposal README.md for a comparison with getaddrinfo.\n    ///\n    /// This function never blocks. It either immediately fails or immediately\n    /// returns successfully with a `resolve-address-stream` that can be used\n    /// to (asynchronously) fetch the results.\n    ///\n    /// # Typical errors\n    /// - `invalid-argument`: `name` is a syntactically invalid domain name or IP address.\n    ///\n    /// # References:\n    /// - <https://pubs.opengroup.org/onlinepubs/9699919799/functions/getaddrinfo.html>\n    /// - <https://man7.org/linux/man-pages/man3/getaddrinfo.3.html>\n    /// - <https://learn.microsoft.com/en-us/windows/win32/api/ws2tcpip/nf-ws2tcpip-getaddrinfo>\n    /// - <https://man.freebsd.org/cgi/man.cgi?query=getaddrinfo&sektion=3>\n    resolve-addresses: func(network: borrow<network>, name: string) -> result<resolve-address-stream, error-code>;\n\n    resource resolve-address-stream {\n        /// Returns the next address from the resolver.\n        ///\n        /// This function should be called multiple times. On each call, it will\n        /// return the next address in connection order preference. If all\n        /// addresses have been exhausted, this function returns `none`.\n        ///\n        /// This function never returns IPv4-mapped IPv6 addresses.\n        ///\n        /// # Typical errors\n        /// - `name-unresolvable`:          Name does not exist or has no suitable associated IP addresses. (EAI_NONAME, EAI_NODATA, EAI_ADDRFAMILY)\n        /// - `temporary-resolver-failure`: A temporary failure in name resolution occurred. (EAI_AGAIN)\n        /// - `permanent-resolver-failure`: A permanent failure in name resolution occurred. (EAI_FAIL)\n        /// - `would-block`:                A result is not available yet. (EWOULDBLOCK, EAGAIN)\n        resolve-next-address: func() -> result<option<ip-address>, error-code>;\n\n        /// Create a `pollable` which will resolve once the stream is ready for I/O.\n        ///\n        /// Note: this function is here for WASI Preview2 only.\n        /// It\'s planned to be removed when `future` is natively supported in Preview3.\n        subscribe: func() -> pollable;\n    }\n}\n";
const _: &str = "\ninterface tcp-create-socket {\n    use network.{network, error-code, ip-address-family};\n    use tcp.{tcp-socket};\n\n    /// Create a new TCP socket.\n    ///\n    /// Similar to `socket(AF_INET or AF_INET6, SOCK_STREAM, IPPROTO_TCP)` in POSIX.\n    /// On IPv6 sockets, IPV6_V6ONLY is enabled by default and can\'t be configured otherwise.\n    ///\n    /// This function does not require a network capability handle. This is considered to be safe because\n    /// at time of creation, the socket is not bound to any `network` yet. Up to the moment `bind`/`connect`\n    /// is called, the socket is effectively an in-memory configuration object, unable to communicate with the outside world.\n    ///\n    /// All sockets are non-blocking. Use the wasi-poll interface to block on asynchronous operations.\n    ///\n    /// # Typical errors\n    /// - `not-supported`:     The specified `address-family` is not supported. (EAFNOSUPPORT)\n    /// - `new-socket-limit`:  The new socket resource could not be created because of a system limit. (EMFILE, ENFILE)\n    ///\n    /// # References\n    /// - <https://pubs.opengroup.org/onlinepubs/9699919799/functions/socket.html>\n    /// - <https://man7.org/linux/man-pages/man2/socket.2.html>\n    /// - <https://learn.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-wsasocketw>\n    /// - <https://man.freebsd.org/cgi/man.cgi?query=socket&sektion=2>\n    create-tcp-socket: func(address-family: ip-address-family) -> result<tcp-socket, error-code>;\n}\n";
const _: &str = "\ninterface udp {\n    use wasi:io/poll@0.2.0.{pollable};\n    use network.{network, error-code, ip-socket-address, ip-address-family};\n\n    /// A received datagram.\n    record incoming-datagram {\n        /// The payload.\n        /// \n        /// Theoretical max size: ~64 KiB. In practice, typically less than 1500 bytes.\n        data: list<u8>,\n\n        /// The source address.\n        ///\n        /// This field is guaranteed to match the remote address the stream was initialized with, if any.\n        ///\n        /// Equivalent to the `src_addr` out parameter of `recvfrom`.\n        remote-address: ip-socket-address,\n    }\n\n    /// A datagram to be sent out.\n    record outgoing-datagram {\n        /// The payload.\n        data: list<u8>,\n\n        /// The destination address.\n        ///\n        /// The requirements on this field depend on how the stream was initialized:\n        /// - with a remote address: this field must be None or match the stream\'s remote address exactly.\n        /// - without a remote address: this field is required.\n        ///\n        /// If this value is None, the send operation is equivalent to `send` in POSIX. Otherwise it is equivalent to `sendto`.\n        remote-address: option<ip-socket-address>,\n    }\n\n\n\n    /// A UDP socket handle.\n    resource udp-socket {\n        /// Bind the socket to a specific network on the provided IP address and port.\n        ///\n        /// If the IP address is zero (`0.0.0.0` in IPv4, `::` in IPv6), it is left to the implementation to decide which\n        /// network interface(s) to bind to.\n        /// If the port is zero, the socket will be bound to a random free port.\n        ///\n        /// # Typical errors\n        /// - `invalid-argument`:          The `local-address` has the wrong address family. (EAFNOSUPPORT, EFAULT on Windows)\n        /// - `invalid-state`:             The socket is already bound. (EINVAL)\n        /// - `address-in-use`:            No ephemeral ports available. (EADDRINUSE, ENOBUFS on Windows)\n        /// - `address-in-use`:            Address is already in use. (EADDRINUSE)\n        /// - `address-not-bindable`:      `local-address` is not an address that the `network` can bind to. (EADDRNOTAVAIL)\n        /// - `not-in-progress`:           A `bind` operation is not in progress.\n        /// - `would-block`:               Can\'t finish the operation, it is still in progress. (EWOULDBLOCK, EAGAIN)\n        ///\n        /// # Implementors note\n        /// Unlike in POSIX, in WASI the bind operation is async. This enables\n        /// interactive WASI hosts to inject permission prompts. Runtimes that\n        /// don\'t want to make use of this ability can simply call the native\n        /// `bind` as part of either `start-bind` or `finish-bind`.\n        ///\n        /// # References\n        /// - <https://pubs.opengroup.org/onlinepubs/9699919799/functions/bind.html>\n        /// - <https://man7.org/linux/man-pages/man2/bind.2.html>\n        /// - <https://learn.microsoft.com/en-us/windows/win32/api/winsock/nf-winsock-bind>\n        /// - <https://man.freebsd.org/cgi/man.cgi?query=bind&sektion=2&format=html>\n        start-bind: func(network: borrow<network>, local-address: ip-socket-address) -> result<_, error-code>;\n        finish-bind: func() -> result<_, error-code>;\n\n        /// Set up inbound & outbound communication channels, optionally to a specific peer.\n        ///\n        /// This function only changes the local socket configuration and does not generate any network traffic.\n        /// On success, the `remote-address` of the socket is updated. The `local-address` may be updated as well,\n        /// based on the best network path to `remote-address`.\n        ///\n        /// When a `remote-address` is provided, the returned streams are limited to communicating with that specific peer:\n        /// - `send` can only be used to send to this destination.\n        /// - `receive` will only return datagrams sent from the provided `remote-address`.\n        ///\n        /// This method may be called multiple times on the same socket to change its association, but\n        /// only the most recently returned pair of streams will be operational. Implementations may trap if\n        /// the streams returned by a previous invocation haven\'t been dropped yet before calling `stream` again.\n        /// \n        /// The POSIX equivalent in pseudo-code is:\n        /// ```text\n        /// if (was previously connected) {\n        /// \tconnect(s, AF_UNSPEC)\n        /// }\n        /// if (remote_address is Some) {\n        /// \tconnect(s, remote_address)\n        /// }\n        /// ```\n        ///\n        /// Unlike in POSIX, the socket must already be explicitly bound.\n        /// \n        /// # Typical errors\n        /// - `invalid-argument`:          The `remote-address` has the wrong address family. (EAFNOSUPPORT)\n        /// - `invalid-argument`:          The IP address in `remote-address` is set to INADDR_ANY (`0.0.0.0` / `::`). (EDESTADDRREQ, EADDRNOTAVAIL)\n        /// - `invalid-argument`:          The port in `remote-address` is set to 0. (EDESTADDRREQ, EADDRNOTAVAIL)\n        /// - `invalid-state`:             The socket is not bound.\n        /// - `address-in-use`:            Tried to perform an implicit bind, but there were no ephemeral ports available. (EADDRINUSE, EADDRNOTAVAIL on Linux, EAGAIN on BSD)\n        /// - `remote-unreachable`:        The remote address is not reachable. (ECONNRESET, ENETRESET, EHOSTUNREACH, EHOSTDOWN, ENETUNREACH, ENETDOWN, ENONET)\n        /// - `connection-refused`:        The connection was refused. (ECONNREFUSED)\n        ///\n        /// # References\n        /// - <https://pubs.opengroup.org/onlinepubs/9699919799/functions/connect.html>\n        /// - <https://man7.org/linux/man-pages/man2/connect.2.html>\n        /// - <https://learn.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-connect>\n        /// - <https://man.freebsd.org/cgi/man.cgi?connect>\n        %stream: func(remote-address: option<ip-socket-address>) -> result<tuple<incoming-datagram-stream, outgoing-datagram-stream>, error-code>;\n\n        /// Get the current bound address.\n        ///\n        /// POSIX mentions:\n        /// > If the socket has not been bound to a local name, the value\n        /// > stored in the object pointed to by `address` is unspecified.\n        ///\n        /// WASI is stricter and requires `local-address` to return `invalid-state` when the socket hasn\'t been bound yet.\n        /// \n        /// # Typical errors\n        /// - `invalid-state`: The socket is not bound to any local address.\n        ///\n        /// # References\n        /// - <https://pubs.opengroup.org/onlinepubs/9699919799/functions/getsockname.html>\n        /// - <https://man7.org/linux/man-pages/man2/getsockname.2.html>\n        /// - <https://learn.microsoft.com/en-us/windows/win32/api/winsock/nf-winsock-getsockname>\n        /// - <https://man.freebsd.org/cgi/man.cgi?getsockname>\n        local-address: func() -> result<ip-socket-address, error-code>;\n\n        /// Get the address the socket is currently streaming to.\n        ///\n        /// # Typical errors\n        /// - `invalid-state`: The socket is not streaming to a specific remote address. (ENOTCONN)\n        ///\n        /// # References\n        /// - <https://pubs.opengroup.org/onlinepubs/9699919799/functions/getpeername.html>\n        /// - <https://man7.org/linux/man-pages/man2/getpeername.2.html>\n        /// - <https://learn.microsoft.com/en-us/windows/win32/api/winsock/nf-winsock-getpeername>\n        /// - <https://man.freebsd.org/cgi/man.cgi?query=getpeername&sektion=2&n=1>\n        remote-address: func() -> result<ip-socket-address, error-code>;\n\n        /// Whether this is a IPv4 or IPv6 socket.\n        ///\n        /// Equivalent to the SO_DOMAIN socket option.\n        address-family: func() -> ip-address-family;\n\n        /// Equivalent to the IP_TTL & IPV6_UNICAST_HOPS socket options.\n        ///\n        /// If the provided value is 0, an `invalid-argument` error is returned.\n        ///\n        /// # Typical errors\n        /// - `invalid-argument`:     (set) The TTL value must be 1 or higher.\n        unicast-hop-limit: func() -> result<u8, error-code>;\n        set-unicast-hop-limit: func(value: u8) -> result<_, error-code>;\n\n        /// The kernel buffer space reserved for sends/receives on this socket.\n        ///\n        /// If the provided value is 0, an `invalid-argument` error is returned.\n        /// Any other value will never cause an error, but it might be silently clamped and/or rounded.\n        /// I.e. after setting a value, reading the same setting back may return a different value.\n        ///\n        /// Equivalent to the SO_RCVBUF and SO_SNDBUF socket options.\n        ///\n        /// # Typical errors\n        /// - `invalid-argument`:     (set) The provided value was 0.\n        receive-buffer-size: func() -> result<u64, error-code>;\n        set-receive-buffer-size: func(value: u64) -> result<_, error-code>;\n        send-buffer-size: func() -> result<u64, error-code>;\n        set-send-buffer-size: func(value: u64) -> result<_, error-code>;\n\n        /// Create a `pollable` which will resolve once the socket is ready for I/O.\n        ///\n        /// Note: this function is here for WASI Preview2 only.\n        /// It\'s planned to be removed when `future` is natively supported in Preview3.\n        subscribe: func() -> pollable;\n    }\n\n    resource incoming-datagram-stream {\n        /// Receive messages on the socket.\n        ///\n        /// This function attempts to receive up to `max-results` datagrams on the socket without blocking.\n        /// The returned list may contain fewer elements than requested, but never more.\n        ///\n        /// This function returns successfully with an empty list when either:\n        /// - `max-results` is 0, or:\n        /// - `max-results` is greater than 0, but no results are immediately available.\n        /// This function never returns `error(would-block)`.\n        ///\n        /// # Typical errors\n        /// - `remote-unreachable`: The remote address is not reachable. (ECONNRESET, ENETRESET on Windows, EHOSTUNREACH, EHOSTDOWN, ENETUNREACH, ENETDOWN, ENONET)\n        /// - `connection-refused`: The connection was refused. (ECONNREFUSED)\n        ///\n        /// # References\n        /// - <https://pubs.opengroup.org/onlinepubs/9699919799/functions/recvfrom.html>\n        /// - <https://pubs.opengroup.org/onlinepubs/9699919799/functions/recvmsg.html>\n        /// - <https://man7.org/linux/man-pages/man2/recv.2.html>\n        /// - <https://man7.org/linux/man-pages/man2/recvmmsg.2.html>\n        /// - <https://learn.microsoft.com/en-us/windows/win32/api/winsock/nf-winsock-recv>\n        /// - <https://learn.microsoft.com/en-us/windows/win32/api/winsock/nf-winsock-recvfrom>\n        /// - <https://learn.microsoft.com/en-us/previous-versions/windows/desktop/legacy/ms741687(v=vs.85)>\n        /// - <https://man.freebsd.org/cgi/man.cgi?query=recv&sektion=2>\n        receive: func(max-results: u64) -> result<list<incoming-datagram>, error-code>;\n\n        /// Create a `pollable` which will resolve once the stream is ready to receive again.\n        ///\n        /// Note: this function is here for WASI Preview2 only.\n        /// It\'s planned to be removed when `future` is natively supported in Preview3.\n        subscribe: func() -> pollable;\n    }\n\n    resource outgoing-datagram-stream {\n        /// Check readiness for sending. This function never blocks.\n        ///\n        /// Returns the number of datagrams permitted for the next call to `send`,\n        /// or an error. Calling `send` with more datagrams than this function has\n        /// permitted will trap.\n        ///\n        /// When this function returns ok(0), the `subscribe` pollable will\n        /// become ready when this function will report at least ok(1), or an\n        /// error.\n        /// \n        /// Never returns `would-block`.\n        check-send: func() -> result<u64, error-code>;\n\n        /// Send messages on the socket.\n        ///\n        /// This function attempts to send all provided `datagrams` on the socket without blocking and\n        /// returns how many messages were actually sent (or queued for sending). This function never\n        /// returns `error(would-block)`. If none of the datagrams were able to be sent, `ok(0)` is returned.\n        ///\n        /// This function semantically behaves the same as iterating the `datagrams` list and sequentially\n        /// sending each individual datagram until either the end of the list has been reached or the first error occurred.\n        /// If at least one datagram has been sent successfully, this function never returns an error.\n        ///\n        /// If the input list is empty, the function returns `ok(0)`.\n        ///\n        /// Each call to `send` must be permitted by a preceding `check-send`. Implementations must trap if\n        /// either `check-send` was not called or `datagrams` contains more items than `check-send` permitted.\n        ///\n        /// # Typical errors\n        /// - `invalid-argument`:        The `remote-address` has the wrong address family. (EAFNOSUPPORT)\n        /// - `invalid-argument`:        The IP address in `remote-address` is set to INADDR_ANY (`0.0.0.0` / `::`). (EDESTADDRREQ, EADDRNOTAVAIL)\n        /// - `invalid-argument`:        The port in `remote-address` is set to 0. (EDESTADDRREQ, EADDRNOTAVAIL)\n        /// - `invalid-argument`:        The socket is in \"connected\" mode and `remote-address` is `some` value that does not match the address passed to `stream`. (EISCONN)\n        /// - `invalid-argument`:        The socket is not \"connected\" and no value for `remote-address` was provided. (EDESTADDRREQ)\n        /// - `remote-unreachable`:      The remote address is not reachable. (ECONNRESET, ENETRESET on Windows, EHOSTUNREACH, EHOSTDOWN, ENETUNREACH, ENETDOWN, ENONET)\n        /// - `connection-refused`:      The connection was refused. (ECONNREFUSED)\n        /// - `datagram-too-large`:      The datagram is too large. (EMSGSIZE)\n        ///\n        /// # References\n        /// - <https://pubs.opengroup.org/onlinepubs/9699919799/functions/sendto.html>\n        /// - <https://pubs.opengroup.org/onlinepubs/9699919799/functions/sendmsg.html>\n        /// - <https://man7.org/linux/man-pages/man2/send.2.html>\n        /// - <https://man7.org/linux/man-pages/man2/sendmmsg.2.html>\n        /// - <https://learn.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-send>\n        /// - <https://learn.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-sendto>\n        /// - <https://learn.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-wsasendmsg>\n        /// - <https://man.freebsd.org/cgi/man.cgi?query=send&sektion=2>\n        send: func(datagrams: list<outgoing-datagram>) -> result<u64, error-code>;\n        \n        /// Create a `pollable` which will resolve once the stream is ready to send again.\n        ///\n        /// Note: this function is here for WASI Preview2 only.\n        /// It\'s planned to be removed when `future` is natively supported in Preview3.\n        subscribe: func() -> pollable;\n    }\n}\n";
const _: &str = "\ninterface tcp {\n    use wasi:io/streams@0.2.0.{input-stream, output-stream};\n    use wasi:io/poll@0.2.0.{pollable};\n    use wasi:clocks/monotonic-clock@0.2.0.{duration};\n    use network.{network, error-code, ip-socket-address, ip-address-family};\n\n    enum shutdown-type {\n        /// Similar to `SHUT_RD` in POSIX.\n        receive,\n\n        /// Similar to `SHUT_WR` in POSIX.\n        send,\n\n        /// Similar to `SHUT_RDWR` in POSIX.\n        both,\n    }\n    \n    /// A TCP socket resource.\n    ///\n    /// The socket can be in one of the following states:\n    /// - `unbound`\n    /// - `bind-in-progress`\n    /// - `bound` (See note below)\n    /// - `listen-in-progress`\n    /// - `listening`\n    /// - `connect-in-progress`\n    /// - `connected`\n    /// - `closed`\n    /// See <https://github.com/WebAssembly/wasi-sockets/TcpSocketOperationalSemantics.md>\n    /// for a more information.\n    ///\n    /// Note: Except where explicitly mentioned, whenever this documentation uses\n    /// the term \"bound\" without backticks it actually means: in the `bound` state *or higher*.\n    /// (i.e. `bound`, `listen-in-progress`, `listening`, `connect-in-progress` or `connected`)\n    ///\n    /// In addition to the general error codes documented on the\n    /// `network::error-code` type, TCP socket methods may always return\n    /// `error(invalid-state)` when in the `closed` state.\n    resource tcp-socket {\n        /// Bind the socket to a specific network on the provided IP address and port.\n        ///\n        /// If the IP address is zero (`0.0.0.0` in IPv4, `::` in IPv6), it is left to the implementation to decide which\n        /// network interface(s) to bind to.\n        /// If the TCP/UDP port is zero, the socket will be bound to a random free port.\n        ///\n        /// Bind can be attempted multiple times on the same socket, even with\n        /// different arguments on each iteration. But never concurrently and\n        /// only as long as the previous bind failed. Once a bind succeeds, the\n        /// binding can\'t be changed anymore.\n        ///\n        /// # Typical errors\n        /// - `invalid-argument`:          The `local-address` has the wrong address family. (EAFNOSUPPORT, EFAULT on Windows)\n        /// - `invalid-argument`:          `local-address` is not a unicast address. (EINVAL)\n        /// - `invalid-argument`:          `local-address` is an IPv4-mapped IPv6 address. (EINVAL)\n        /// - `invalid-state`:             The socket is already bound. (EINVAL)\n        /// - `address-in-use`:            No ephemeral ports available. (EADDRINUSE, ENOBUFS on Windows)\n        /// - `address-in-use`:            Address is already in use. (EADDRINUSE)\n        /// - `address-not-bindable`:      `local-address` is not an address that the `network` can bind to. (EADDRNOTAVAIL)\n        /// - `not-in-progress`:           A `bind` operation is not in progress.\n        /// - `would-block`:               Can\'t finish the operation, it is still in progress. (EWOULDBLOCK, EAGAIN)\n        /// \n        /// # Implementors note\n        /// When binding to a non-zero port, this bind operation shouldn\'t be affected by the TIME_WAIT\n        /// state of a recently closed socket on the same local address. In practice this means that the SO_REUSEADDR \n        /// socket option should be set implicitly on all platforms, except on Windows where this is the default behavior\n        /// and SO_REUSEADDR performs something different entirely.\n        ///\n        /// Unlike in POSIX, in WASI the bind operation is async. This enables\n        /// interactive WASI hosts to inject permission prompts. Runtimes that\n        /// don\'t want to make use of this ability can simply call the native\n        /// `bind` as part of either `start-bind` or `finish-bind`.\n        ///\n        /// # References\n        /// - <https://pubs.opengroup.org/onlinepubs/9699919799/functions/bind.html>\n        /// - <https://man7.org/linux/man-pages/man2/bind.2.html>\n        /// - <https://learn.microsoft.com/en-us/windows/win32/api/winsock/nf-winsock-bind>\n        /// - <https://man.freebsd.org/cgi/man.cgi?query=bind&sektion=2&format=html>\n        start-bind: func(network: borrow<network>, local-address: ip-socket-address) -> result<_, error-code>;\n        finish-bind: func() -> result<_, error-code>;\n\n        /// Connect to a remote endpoint.\n        ///\n        /// On success:\n        /// - the socket is transitioned into the `connection` state.\n        /// - a pair of streams is returned that can be used to read & write to the connection\n        ///\n        /// After a failed connection attempt, the socket will be in the `closed`\n        /// state and the only valid action left is to `drop` the socket. A single\n        /// socket can not be used to connect more than once.\n        ///\n        /// # Typical errors\n        /// - `invalid-argument`:          The `remote-address` has the wrong address family. (EAFNOSUPPORT)\n        /// - `invalid-argument`:          `remote-address` is not a unicast address. (EINVAL, ENETUNREACH on Linux, EAFNOSUPPORT on MacOS)\n        /// - `invalid-argument`:          `remote-address` is an IPv4-mapped IPv6 address. (EINVAL, EADDRNOTAVAIL on Illumos)\n        /// - `invalid-argument`:          The IP address in `remote-address` is set to INADDR_ANY (`0.0.0.0` / `::`). (EADDRNOTAVAIL on Windows)\n        /// - `invalid-argument`:          The port in `remote-address` is set to 0. (EADDRNOTAVAIL on Windows)\n        /// - `invalid-argument`:          The socket is already attached to a different network. The `network` passed to `connect` must be identical to the one passed to `bind`.\n        /// - `invalid-state`:             The socket is already in the `connected` state. (EISCONN)\n        /// - `invalid-state`:             The socket is already in the `listening` state. (EOPNOTSUPP, EINVAL on Windows)\n        /// - `timeout`:                   Connection timed out. (ETIMEDOUT)\n        /// - `connection-refused`:        The connection was forcefully rejected. (ECONNREFUSED)\n        /// - `connection-reset`:          The connection was reset. (ECONNRESET)\n        /// - `connection-aborted`:        The connection was aborted. (ECONNABORTED)\n        /// - `remote-unreachable`:        The remote address is not reachable. (EHOSTUNREACH, EHOSTDOWN, ENETUNREACH, ENETDOWN, ENONET)\n        /// - `address-in-use`:            Tried to perform an implicit bind, but there were no ephemeral ports available. (EADDRINUSE, EADDRNOTAVAIL on Linux, EAGAIN on BSD)\n        /// - `not-in-progress`:           A connect operation is not in progress.\n        /// - `would-block`:               Can\'t finish the operation, it is still in progress. (EWOULDBLOCK, EAGAIN)\n        ///\n        /// # Implementors note\n        /// The POSIX equivalent of `start-connect` is the regular `connect` syscall.\n        /// Because all WASI sockets are non-blocking this is expected to return\n        /// EINPROGRESS, which should be translated to `ok()` in WASI.\n        ///\n        /// The POSIX equivalent of `finish-connect` is a `poll` for event `POLLOUT`\n        /// with a timeout of 0 on the socket descriptor. Followed by a check for\n        /// the `SO_ERROR` socket option, in case the poll signaled readiness.\n        ///\n        /// # References\n        /// - <https://pubs.opengroup.org/onlinepubs/9699919799/functions/connect.html>\n        /// - <https://man7.org/linux/man-pages/man2/connect.2.html>\n        /// - <https://learn.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-connect>\n        /// - <https://man.freebsd.org/cgi/man.cgi?connect>\n        start-connect: func(network: borrow<network>, remote-address: ip-socket-address) -> result<_, error-code>;\n        finish-connect: func() -> result<tuple<input-stream, output-stream>, error-code>;\n\n        /// Start listening for new connections.\n        ///\n        /// Transitions the socket into the `listening` state.\n        ///\n        /// Unlike POSIX, the socket must already be explicitly bound.\n        ///\n        /// # Typical errors\n        /// - `invalid-state`:             The socket is not bound to any local address. (EDESTADDRREQ)\n        /// - `invalid-state`:             The socket is already in the `connected` state. (EISCONN, EINVAL on BSD)\n        /// - `invalid-state`:             The socket is already in the `listening` state.\n        /// - `address-in-use`:            Tried to perform an implicit bind, but there were no ephemeral ports available. (EADDRINUSE)\n        /// - `not-in-progress`:           A listen operation is not in progress.\n        /// - `would-block`:               Can\'t finish the operation, it is still in progress. (EWOULDBLOCK, EAGAIN)\n        ///\n        /// # Implementors note\n        /// Unlike in POSIX, in WASI the listen operation is async. This enables\n        /// interactive WASI hosts to inject permission prompts. Runtimes that\n        /// don\'t want to make use of this ability can simply call the native\n        /// `listen` as part of either `start-listen` or `finish-listen`.\n        ///\n        /// # References\n        /// - <https://pubs.opengroup.org/onlinepubs/9699919799/functions/listen.html>\n        /// - <https://man7.org/linux/man-pages/man2/listen.2.html>\n        /// - <https://learn.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-listen>\n        /// - <https://man.freebsd.org/cgi/man.cgi?query=listen&sektion=2>\n        start-listen: func() -> result<_, error-code>;\n        finish-listen: func() -> result<_, error-code>;\n\n        /// Accept a new client socket.\n        ///\n        /// The returned socket is bound and in the `connected` state. The following properties are inherited from the listener socket:\n        /// - `address-family`\n        /// - `keep-alive-enabled`\n        /// - `keep-alive-idle-time`\n        /// - `keep-alive-interval`\n        /// - `keep-alive-count`\n        /// - `hop-limit`\n        /// - `receive-buffer-size`\n        /// - `send-buffer-size`\n        ///\n        /// On success, this function returns the newly accepted client socket along with\n        /// a pair of streams that can be used to read & write to the connection.\n        ///\n        /// # Typical errors\n        /// - `invalid-state`:      Socket is not in the `listening` state. (EINVAL)\n        /// - `would-block`:        No pending connections at the moment. (EWOULDBLOCK, EAGAIN)\n        /// - `connection-aborted`: An incoming connection was pending, but was terminated by the client before this listener could accept it. (ECONNABORTED)\n        /// - `new-socket-limit`:   The new socket resource could not be created because of a system limit. (EMFILE, ENFILE)\n        ///\n        /// # References\n        /// - <https://pubs.opengroup.org/onlinepubs/9699919799/functions/accept.html>\n        /// - <https://man7.org/linux/man-pages/man2/accept.2.html>\n        /// - <https://learn.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-accept>\n        /// - <https://man.freebsd.org/cgi/man.cgi?query=accept&sektion=2>\n        accept: func() -> result<tuple<tcp-socket, input-stream, output-stream>, error-code>;\n\n        /// Get the bound local address.\n        ///\n        /// POSIX mentions:\n        /// > If the socket has not been bound to a local name, the value\n        /// > stored in the object pointed to by `address` is unspecified.\n        ///\n        /// WASI is stricter and requires `local-address` to return `invalid-state` when the socket hasn\'t been bound yet.\n        ///\n        /// # Typical errors\n        /// - `invalid-state`: The socket is not bound to any local address.\n        ///\n        /// # References\n        /// - <https://pubs.opengroup.org/onlinepubs/9699919799/functions/getsockname.html>\n        /// - <https://man7.org/linux/man-pages/man2/getsockname.2.html>\n        /// - <https://learn.microsoft.com/en-us/windows/win32/api/winsock/nf-winsock-getsockname>\n        /// - <https://man.freebsd.org/cgi/man.cgi?getsockname>\n        local-address: func() -> result<ip-socket-address, error-code>;\n\n        /// Get the remote address.\n        ///\n        /// # Typical errors\n        /// - `invalid-state`: The socket is not connected to a remote address. (ENOTCONN)\n        ///\n        /// # References\n        /// - <https://pubs.opengroup.org/onlinepubs/9699919799/functions/getpeername.html>\n        /// - <https://man7.org/linux/man-pages/man2/getpeername.2.html>\n        /// - <https://learn.microsoft.com/en-us/windows/win32/api/winsock/nf-winsock-getpeername>\n        /// - <https://man.freebsd.org/cgi/man.cgi?query=getpeername&sektion=2&n=1>\n        remote-address: func() -> result<ip-socket-address, error-code>;\n\n        /// Whether the socket is in the `listening` state.\n        ///\n        /// Equivalent to the SO_ACCEPTCONN socket option.\n        is-listening: func() -> bool;\n\n        /// Whether this is a IPv4 or IPv6 socket.\n        ///\n        /// Equivalent to the SO_DOMAIN socket option.\n        address-family: func() -> ip-address-family;\n\n        /// Hints the desired listen queue size. Implementations are free to ignore this.\n        ///\n        /// If the provided value is 0, an `invalid-argument` error is returned.\n        /// Any other value will never cause an error, but it might be silently clamped and/or rounded.\n        ///\n        /// # Typical errors\n        /// - `not-supported`:        (set) The platform does not support changing the backlog size after the initial listen.\n        /// - `invalid-argument`:     (set) The provided value was 0.\n        /// - `invalid-state`:        (set) The socket is in the `connect-in-progress` or `connected` state.\n        set-listen-backlog-size: func(value: u64) -> result<_, error-code>;\n\n        /// Enables or disables keepalive.\n        ///\n        /// The keepalive behavior can be adjusted using:\n        /// - `keep-alive-idle-time`\n        /// - `keep-alive-interval`\n        /// - `keep-alive-count`\n        /// These properties can be configured while `keep-alive-enabled` is false, but only come into effect when `keep-alive-enabled` is true.\n        ///\n        /// Equivalent to the SO_KEEPALIVE socket option.\n        keep-alive-enabled: func() -> result<bool, error-code>;\n        set-keep-alive-enabled: func(value: bool) -> result<_, error-code>;\n\n        /// Amount of time the connection has to be idle before TCP starts sending keepalive packets.\n        ///\n        /// If the provided value is 0, an `invalid-argument` error is returned.\n        /// Any other value will never cause an error, but it might be silently clamped and/or rounded.\n        /// I.e. after setting a value, reading the same setting back may return a different value.\n        ///\n        /// Equivalent to the TCP_KEEPIDLE socket option. (TCP_KEEPALIVE on MacOS)\n        ///\n        /// # Typical errors\n        /// - `invalid-argument`:     (set) The provided value was 0.\n        keep-alive-idle-time: func() -> result<duration, error-code>;\n        set-keep-alive-idle-time: func(value: duration) -> result<_, error-code>;\n\n        /// The time between keepalive packets.\n        ///\n        /// If the provided value is 0, an `invalid-argument` error is returned.\n        /// Any other value will never cause an error, but it might be silently clamped and/or rounded.\n        /// I.e. after setting a value, reading the same setting back may return a different value.\n        ///\n        /// Equivalent to the TCP_KEEPINTVL socket option.\n        ///\n        /// # Typical errors\n        /// - `invalid-argument`:     (set) The provided value was 0.\n        keep-alive-interval: func() -> result<duration, error-code>;\n        set-keep-alive-interval: func(value: duration) -> result<_, error-code>;\n\n        /// The maximum amount of keepalive packets TCP should send before aborting the connection.\n        ///\n        /// If the provided value is 0, an `invalid-argument` error is returned.\n        /// Any other value will never cause an error, but it might be silently clamped and/or rounded.\n        /// I.e. after setting a value, reading the same setting back may return a different value.\n        ///\n        /// Equivalent to the TCP_KEEPCNT socket option.\n        ///\n        /// # Typical errors\n        /// - `invalid-argument`:     (set) The provided value was 0.\n        keep-alive-count: func() -> result<u32, error-code>;\n        set-keep-alive-count: func(value: u32) -> result<_, error-code>;\n\n        /// Equivalent to the IP_TTL & IPV6_UNICAST_HOPS socket options.\n        ///\n        /// If the provided value is 0, an `invalid-argument` error is returned.\n        ///\n        /// # Typical errors\n        /// - `invalid-argument`:     (set) The TTL value must be 1 or higher.\n        hop-limit: func() -> result<u8, error-code>;\n        set-hop-limit: func(value: u8) -> result<_, error-code>;\n\n        /// The kernel buffer space reserved for sends/receives on this socket.\n        ///\n        /// If the provided value is 0, an `invalid-argument` error is returned.\n        /// Any other value will never cause an error, but it might be silently clamped and/or rounded.\n        /// I.e. after setting a value, reading the same setting back may return a different value.\n        ///\n        /// Equivalent to the SO_RCVBUF and SO_SNDBUF socket options.\n        ///\n        /// # Typical errors\n        /// - `invalid-argument`:     (set) The provided value was 0.\n        receive-buffer-size: func() -> result<u64, error-code>;\n        set-receive-buffer-size: func(value: u64) -> result<_, error-code>;\n        send-buffer-size: func() -> result<u64, error-code>;\n        set-send-buffer-size: func(value: u64) -> result<_, error-code>;\n\n        /// Create a `pollable` which can be used to poll for, or block on,\n        /// completion of any of the asynchronous operations of this socket.\n        ///\n        /// When `finish-bind`, `finish-listen`, `finish-connect` or `accept`\n        /// return `error(would-block)`, this pollable can be used to wait for\n        /// their success or failure, after which the method can be retried.\n        ///\n        /// The pollable is not limited to the async operation that happens to be\n        /// in progress at the time of calling `subscribe` (if any). Theoretically,\n        /// `subscribe` only has to be called once per socket and can then be\n        /// (re)used for the remainder of the socket\'s lifetime.\n        ///\n        /// See <https://github.com/WebAssembly/wasi-sockets/TcpSocketOperationalSemantics.md#Pollable-readiness>\n        /// for a more information.\n        ///\n        /// Note: this function is here for WASI Preview2 only.\n        /// It\'s planned to be removed when `future` is natively supported in Preview3.\n        subscribe: func() -> pollable;\n\n        /// Initiate a graceful shutdown.\n        ///\n        /// - `receive`: The socket is not expecting to receive any data from\n        ///   the peer. The `input-stream` associated with this socket will be\n        ///   closed. Any data still in the receive queue at time of calling\n        ///   this method will be discarded.\n        /// - `send`: The socket has no more data to send to the peer. The `output-stream`\n        ///   associated with this socket will be closed and a FIN packet will be sent.\n        /// - `both`: Same effect as `receive` & `send` combined.\n        ///\n        /// This function is idempotent. Shutting a down a direction more than once\n        /// has no effect and returns `ok`.\n        ///\n        /// The shutdown function does not close (drop) the socket.\n        ///\n        /// # Typical errors\n        /// - `invalid-state`: The socket is not in the `connected` state. (ENOTCONN)\n        ///\n        /// # References\n        /// - <https://pubs.opengroup.org/onlinepubs/9699919799/functions/shutdown.html>\n        /// - <https://man7.org/linux/man-pages/man2/shutdown.2.html>\n        /// - <https://learn.microsoft.com/en-us/windows/win32/api/winsock/nf-winsock-shutdown>\n        /// - <https://man.freebsd.org/cgi/man.cgi?query=shutdown&sektion=2>\n        shutdown: func(shutdown-type: shutdown-type) -> result<_, error-code>;\n    }\n}\n";
const _: &str = "\ninterface udp-create-socket {\n    use network.{network, error-code, ip-address-family};\n    use udp.{udp-socket};\n\n    /// Create a new UDP socket.\n    ///\n    /// Similar to `socket(AF_INET or AF_INET6, SOCK_DGRAM, IPPROTO_UDP)` in POSIX.\n    /// On IPv6 sockets, IPV6_V6ONLY is enabled by default and can\'t be configured otherwise.\n    ///\n    /// This function does not require a network capability handle. This is considered to be safe because\n    /// at time of creation, the socket is not bound to any `network` yet. Up to the moment `bind` is called,\n    /// the socket is effectively an in-memory configuration object, unable to communicate with the outside world.\n    ///\n    /// All sockets are non-blocking. Use the wasi-poll interface to block on asynchronous operations.\n    ///\n    /// # Typical errors\n    /// - `not-supported`:     The specified `address-family` is not supported. (EAFNOSUPPORT)\n    /// - `new-socket-limit`:  The new socket resource could not be created because of a system limit. (EMFILE, ENFILE)\n    ///\n    /// # References:\n    /// - <https://pubs.opengroup.org/onlinepubs/9699919799/functions/socket.html>\n    /// - <https://man7.org/linux/man-pages/man2/socket.2.html>\n    /// - <https://learn.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-wsasocketw>\n    /// - <https://man.freebsd.org/cgi/man.cgi?query=socket&sektion=2>\n    create-udp-socket: func(address-family: ip-address-family) -> result<udp-socket, error-code>;\n}\n";
const _: &str = "\ninterface network {\n    /// An opaque resource that represents access to (a subset of) the network.\n    /// This enables context-based security for networking.\n    /// There is no need for this to map 1:1 to a physical network interface.\n    resource network;\n\n    /// Error codes.\n    ///\n    /// In theory, every API can return any error code.\n    /// In practice, API\'s typically only return the errors documented per API\n    /// combined with a couple of errors that are always possible:\n    /// - `unknown`\n    /// - `access-denied`\n    /// - `not-supported`\n    /// - `out-of-memory`\n    /// - `concurrency-conflict`\n    ///\n    /// See each individual API for what the POSIX equivalents are. They sometimes differ per API.\n    enum error-code {\n        /// Unknown error\n        unknown,\n\n        /// Access denied.\n        ///\n        /// POSIX equivalent: EACCES, EPERM\n        access-denied,\n\n        /// The operation is not supported.\n        ///\n        /// POSIX equivalent: EOPNOTSUPP\n        not-supported,\n\n        /// One of the arguments is invalid.\n        ///\n        /// POSIX equivalent: EINVAL\n        invalid-argument,\n\n        /// Not enough memory to complete the operation.\n        ///\n        /// POSIX equivalent: ENOMEM, ENOBUFS, EAI_MEMORY\n        out-of-memory,\n\n        /// The operation timed out before it could finish completely.\n        timeout,\n\n        /// This operation is incompatible with another asynchronous operation that is already in progress.\n        ///\n        /// POSIX equivalent: EALREADY\n        concurrency-conflict,\n\n        /// Trying to finish an asynchronous operation that:\n        /// - has not been started yet, or:\n        /// - was already finished by a previous `finish-*` call.\n        ///\n        /// Note: this is scheduled to be removed when `future`s are natively supported.\n        not-in-progress,\n\n        /// The operation has been aborted because it could not be completed immediately.\n        ///\n        /// Note: this is scheduled to be removed when `future`s are natively supported.\n        would-block,\n\n\n        /// The operation is not valid in the socket\'s current state.\n        invalid-state,\n\n        /// A new socket resource could not be created because of a system limit.\n        new-socket-limit,\n\n        /// A bind operation failed because the provided address is not an address that the `network` can bind to.\n        address-not-bindable,\n\n        /// A bind operation failed because the provided address is already in use or because there are no ephemeral ports available.\n        address-in-use,\n\n        /// The remote address is not reachable\n        remote-unreachable,\n\n\n        /// The TCP connection was forcefully rejected\n        connection-refused,\n\n        /// The TCP connection was reset.\n        connection-reset,\n\n        /// A TCP connection was aborted.\n        connection-aborted,\n\n\n        /// The size of a datagram sent to a UDP socket exceeded the maximum\n        /// supported size.\n        datagram-too-large,\n\n\n        /// Name does not exist or has no suitable associated IP addresses.\n        name-unresolvable,\n\n        /// A temporary failure in name resolution occurred.\n        temporary-resolver-failure,\n\n        /// A permanent failure in name resolution occurred.\n        permanent-resolver-failure,\n    }\n\n    enum ip-address-family {\n        /// Similar to `AF_INET` in POSIX.\n        ipv4,\n\n        /// Similar to `AF_INET6` in POSIX.\n        ipv6,\n    }\n\n    type ipv4-address = tuple<u8, u8, u8, u8>;\n    type ipv6-address = tuple<u16, u16, u16, u16, u16, u16, u16, u16>;\n\n    variant ip-address {\n        ipv4(ipv4-address),\n        ipv6(ipv6-address),\n    }\n\n    record ipv4-socket-address {\n        /// sin_port\n        port: u16,\n        /// sin_addr\n        address: ipv4-address,\n    }\n\n    record ipv6-socket-address {\n        /// sin6_port\n        port: u16,\n        /// sin6_flowinfo\n        flow-info: u32,\n        /// sin6_addr\n        address: ipv6-address,\n        /// sin6_scope_id\n        scope-id: u32,\n    }\n\n    variant ip-socket-address {\n        ipv4(ipv4-socket-address),\n        ipv6(ipv6-socket-address),\n    }\n\n}\n";
const _: &str = "package wasi:cli@0.2.0;\n\nworld imports {\n  include wasi:clocks/imports@0.2.0;\n  include wasi:filesystem/imports@0.2.0;\n  include wasi:sockets/imports@0.2.0;\n  include wasi:random/imports@0.2.0;\n  include wasi:io/imports@0.2.0;\n\n  import environment;\n  import exit;\n  import stdin;\n  import stdout;\n  import stderr;\n  import terminal-input;\n  import terminal-output;\n  import terminal-stdin;\n  import terminal-stdout;\n  import terminal-stderr;\n}\n";
const _: &str = "/// Terminal input.\n///\n/// In the future, this may include functions for disabling echoing,\n/// disabling input buffering so that keyboard events are sent through\n/// immediately, querying supported features, and so on.\ninterface terminal-input {\n    /// The input side of a terminal.\n    resource terminal-input;\n}\n\n/// Terminal output.\n///\n/// In the future, this may include functions for querying the terminal\n/// size, being notified of terminal size changes, querying supported\n/// features, and so on.\ninterface terminal-output {\n    /// The output side of a terminal.\n    resource terminal-output;\n}\n\n/// An interface providing an optional `terminal-input` for stdin as a\n/// link-time authority.\ninterface terminal-stdin {\n    use terminal-input.{terminal-input};\n\n    /// If stdin is connected to a terminal, return a `terminal-input` handle\n    /// allowing further interaction with it.\n    get-terminal-stdin: func() -> option<terminal-input>;\n}\n\n/// An interface providing an optional `terminal-output` for stdout as a\n/// link-time authority.\ninterface terminal-stdout {\n    use terminal-output.{terminal-output};\n\n    /// If stdout is connected to a terminal, return a `terminal-output` handle\n    /// allowing further interaction with it.\n    get-terminal-stdout: func() -> option<terminal-output>;\n}\n\n/// An interface providing an optional `terminal-output` for stderr as a\n/// link-time authority.\ninterface terminal-stderr {\n    use terminal-output.{terminal-output};\n\n    /// If stderr is connected to a terminal, return a `terminal-output` handle\n    /// allowing further interaction with it.\n    get-terminal-stderr: func() -> option<terminal-output>;\n}\n";
const _: &str = "interface stdin {\n  use wasi:io/streams@0.2.0.{input-stream};\n\n  get-stdin: func() -> input-stream;\n}\n\ninterface stdout {\n  use wasi:io/streams@0.2.0.{output-stream};\n\n  get-stdout: func() -> output-stream;\n}\n\ninterface stderr {\n  use wasi:io/streams@0.2.0.{output-stream};\n\n  get-stderr: func() -> output-stream;\n}\n";
const _: &str = "package wasi:cli@0.2.0;\n\nworld command {\n  include imports;\n\n  export run;\n}\n";
const _: &str = "interface exit {\n  /// Exit the current instance and any linked instances.\n  exit: func(status: result);\n}\n";
const _: &str = "interface run {\n  /// Run the program.\n  run: func() -> result;\n}\n";
const _: &str = "interface environment {\n  /// Get the POSIX-style environment variables.\n  ///\n  /// Each environment variable is provided as a pair of string variable names\n  /// and string value.\n  ///\n  /// Morally, these are a value import, but until value imports are available\n  /// in the component model, this import function should return the same\n  /// values each time it is called.\n  get-environment: func() -> list<tuple<string, string>>;\n\n  /// Get the POSIX-style arguments to the program.\n  get-arguments: func() -> list<string>;\n\n  /// Return a path that programs should use as their initial current working\n  /// directory, interpreting `.` as shorthand for this.\n  initial-cwd: func() -> option<string>;\n}\n";
const _: &str = "/// This interface defines a handler of incoming HTTP Requests. It should\n/// be exported by components which can respond to HTTP Requests.\ninterface incoming-handler {\n  use types.{incoming-request, response-outparam};\n\n  /// This function is invoked with an incoming HTTP Request, and a resource\n  /// `response-outparam` which provides the capability to reply with an HTTP\n  /// Response. The response is sent by calling the `response-outparam.set`\n  /// method, which allows execution to continue after the response has been\n  /// sent. This enables both streaming to the response body, and performing other\n  /// work.\n  ///\n  /// The implementor of this function must write a response to the\n  /// `response-outparam` before returning, or else the caller will respond\n  /// with an error on its behalf.\n  handle: func(\n    request: incoming-request,\n    response-out: response-outparam\n  );\n}\n\n/// This interface defines a handler of outgoing HTTP Requests. It should be\n/// imported by components which wish to make HTTP Requests.\ninterface outgoing-handler {\n  use types.{\n    outgoing-request, request-options, future-incoming-response, error-code\n  };\n\n  /// This function is invoked with an outgoing HTTP Request, and it returns\n  /// a resource `future-incoming-response` which represents an HTTP Response\n  /// which may arrive in the future.\n  ///\n  /// The `options` argument accepts optional parameters for the HTTP\n  /// protocol\'s transport layer.\n  ///\n  /// This function may return an error if the `outgoing-request` is invalid\n  /// or not allowed to be made. Otherwise, protocol errors are reported\n  /// through the `future-incoming-response`.\n  handle: func(\n    request: outgoing-request,\n    options: option<request-options>\n  ) -> result<future-incoming-response, error-code>;\n}\n";
const _: &str = "package wasi:http@0.2.0;\n\n/// The `wasi:http/imports` world imports all the APIs for HTTP proxies.\n/// It is intended to be `include`d in other worlds.\nworld imports {\n  /// HTTP proxies have access to time and randomness.\n  include wasi:clocks/imports@0.2.0;\n  import wasi:random/random@0.2.0;\n\n  /// Proxies have standard output and error streams which are expected to\n  /// terminate in a developer-facing console provided by the host.\n  import wasi:cli/stdout@0.2.0;\n  import wasi:cli/stderr@0.2.0;\n\n  /// TODO: this is a temporary workaround until component tooling is able to\n  /// gracefully handle the absence of stdin. Hosts must return an eof stream\n  /// for this import, which is what wasi-libc + tooling will do automatically\n  /// when this import is properly removed.\n  import wasi:cli/stdin@0.2.0;\n\n  /// This is the default handler to use when user code simply wants to make an\n  /// HTTP request (e.g., via `fetch()`).\n  import outgoing-handler;\n}\n\n/// The `wasi:http/proxy` world captures a widely-implementable intersection of\n/// hosts that includes HTTP forward and reverse proxies. Components targeting\n/// this world may concurrently stream in and out any number of incoming and\n/// outgoing HTTP requests.\nworld proxy {\n  include imports;\n\n  /// The host delivers incoming HTTP requests to a component by calling the\n  /// `handle` function of this exported interface. A host may arbitrarily reuse\n  /// or not reuse component instance when delivering incoming HTTP requests and\n  /// thus a component must be able to handle 0..N calls to `handle`.\n  export incoming-handler;\n}\n";
const _: &str = "/// This interface defines all of the types and methods for implementing\n/// HTTP Requests and Responses, both incoming and outgoing, as well as\n/// their headers, trailers, and bodies.\ninterface types {\n  use wasi:clocks/monotonic-clock@0.2.0.{duration};\n  use wasi:io/streams@0.2.0.{input-stream, output-stream};\n  use wasi:io/error@0.2.0.{error as io-error};\n  use wasi:io/poll@0.2.0.{pollable};\n\n  /// This type corresponds to HTTP standard Methods.\n  variant method {\n    get,\n    head,\n    post,\n    put,\n    delete,\n    connect,\n    options,\n    trace,\n    patch,\n    other(string)\n  }\n\n  /// This type corresponds to HTTP standard Related Schemes.\n  variant scheme {\n    HTTP,\n    HTTPS,\n    other(string)\n  }\n\n  /// These cases are inspired by the IANA HTTP Proxy Error Types:\n  ///   https://www.iana.org/assignments/http-proxy-status/http-proxy-status.xhtml#table-http-proxy-error-types\n  variant error-code {\n    DNS-timeout,\n    DNS-error(DNS-error-payload),\n    destination-not-found,\n    destination-unavailable,\n    destination-IP-prohibited,\n    destination-IP-unroutable,\n    connection-refused,\n    connection-terminated,\n    connection-timeout,\n    connection-read-timeout,\n    connection-write-timeout,\n    connection-limit-reached,\n    TLS-protocol-error,\n    TLS-certificate-error,\n    TLS-alert-received(TLS-alert-received-payload),\n    HTTP-request-denied,\n    HTTP-request-length-required,\n    HTTP-request-body-size(option<u64>),\n    HTTP-request-method-invalid,\n    HTTP-request-URI-invalid,\n    HTTP-request-URI-too-long,\n    HTTP-request-header-section-size(option<u32>),\n    HTTP-request-header-size(option<field-size-payload>),\n    HTTP-request-trailer-section-size(option<u32>),\n    HTTP-request-trailer-size(field-size-payload),\n    HTTP-response-incomplete,\n    HTTP-response-header-section-size(option<u32>),\n    HTTP-response-header-size(field-size-payload),\n    HTTP-response-body-size(option<u64>),\n    HTTP-response-trailer-section-size(option<u32>),\n    HTTP-response-trailer-size(field-size-payload),\n    HTTP-response-transfer-coding(option<string>),\n    HTTP-response-content-coding(option<string>),\n    HTTP-response-timeout,\n    HTTP-upgrade-failed,\n    HTTP-protocol-error,\n    loop-detected,\n    configuration-error,\n    /// This is a catch-all error for anything that doesn\'t fit cleanly into a\n    /// more specific case. It also includes an optional string for an\n    /// unstructured description of the error. Users should not depend on the\n    /// string for diagnosing errors, as it\'s not required to be consistent\n    /// between implementations.\n    internal-error(option<string>)\n  }\n\n  /// Defines the case payload type for `DNS-error` above:\n  record DNS-error-payload {\n    rcode: option<string>,\n    info-code: option<u16>\n  }\n\n  /// Defines the case payload type for `TLS-alert-received` above:\n  record TLS-alert-received-payload {\n    alert-id: option<u8>,\n    alert-message: option<string>\n  }\n\n  /// Defines the case payload type for `HTTP-response-{header,trailer}-size` above:\n  record field-size-payload {\n    field-name: option<string>,\n    field-size: option<u32>\n  }\n\n  /// Attempts to extract a http-related `error` from the wasi:io `error`\n  /// provided.\n  ///\n  /// Stream operations which return\n  /// `wasi:io/stream/stream-error::last-operation-failed` have a payload of\n  /// type `wasi:io/error/error` with more information about the operation\n  /// that failed. This payload can be passed through to this function to see\n  /// if there\'s http-related information about the error to return.\n  ///\n  /// Note that this function is fallible because not all io-errors are\n  /// http-related errors.\n  http-error-code: func(err: borrow<io-error>) -> option<error-code>;\n\n  /// This type enumerates the different kinds of errors that may occur when\n  /// setting or appending to a `fields` resource.\n  variant header-error {\n    /// This error indicates that a `field-key` or `field-value` was\n    /// syntactically invalid when used with an operation that sets headers in a\n    /// `fields`.\n    invalid-syntax,\n\n    /// This error indicates that a forbidden `field-key` was used when trying\n    /// to set a header in a `fields`.\n    forbidden,\n\n    /// This error indicates that the operation on the `fields` was not\n    /// permitted because the fields are immutable.\n    immutable,\n  }\n\n  /// Field keys are always strings.\n  type field-key = string;\n\n  /// Field values should always be ASCII strings. However, in\n  /// reality, HTTP implementations often have to interpret malformed values,\n  /// so they are provided as a list of bytes.\n  type field-value = list<u8>;\n\n  /// This following block defines the `fields` resource which corresponds to\n  /// HTTP standard Fields. Fields are a common representation used for both\n  /// Headers and Trailers.\n  ///\n  /// A `fields` may be mutable or immutable. A `fields` created using the\n  /// constructor, `from-list`, or `clone` will be mutable, but a `fields`\n  /// resource given by other means (including, but not limited to,\n  /// `incoming-request.headers`, `outgoing-request.headers`) might be be\n  /// immutable. In an immutable fields, the `set`, `append`, and `delete`\n  /// operations will fail with `header-error.immutable`.\n  resource fields {\n\n    /// Construct an empty HTTP Fields.\n    ///\n    /// The resulting `fields` is mutable.\n    constructor();\n\n    /// Construct an HTTP Fields.\n    ///\n    /// The resulting `fields` is mutable.\n    ///\n    /// The list represents each key-value pair in the Fields. Keys\n    /// which have multiple values are represented by multiple entries in this\n    /// list with the same key.\n    ///\n    /// The tuple is a pair of the field key, represented as a string, and\n    /// Value, represented as a list of bytes.\n    ///\n    /// An error result will be returned if any `field-key` or `field-value` is\n    /// syntactically invalid, or if a field is forbidden.\n    from-list: static func(\n      entries: list<tuple<field-key,field-value>>\n    ) -> result<fields, header-error>;\n\n    /// Get all of the values corresponding to a key. If the key is not present\n    /// in this `fields` or is syntactically invalid, an empty list is returned.\n    /// However, if the key is present but empty, this is represented by a list\n    /// with one or more empty field-values present.\n    get: func(name: field-key) -> list<field-value>;\n\n    /// Returns `true` when the key is present in this `fields`. If the key is\n    /// syntactically invalid, `false` is returned.\n    has: func(name: field-key) -> bool;\n\n    /// Set all of the values for a key. Clears any existing values for that\n    /// key, if they have been set.\n    ///\n    /// Fails with `header-error.immutable` if the `fields` are immutable.\n    ///\n    /// Fails with `header-error.invalid-syntax` if the `field-key` or any of\n    /// the `field-value`s are syntactically invalid.\n    set: func(name: field-key, value: list<field-value>) -> result<_, header-error>;\n\n    /// Delete all values for a key. Does nothing if no values for the key\n    /// exist.\n    ///\n    /// Fails with `header-error.immutable` if the `fields` are immutable.\n    ///\n    /// Fails with `header-error.invalid-syntax` if the `field-key` is\n    /// syntactically invalid.\n    delete: func(name: field-key) -> result<_, header-error>;\n\n    /// Append a value for a key. Does not change or delete any existing\n    /// values for that key.\n    ///\n    /// Fails with `header-error.immutable` if the `fields` are immutable.\n    ///\n    /// Fails with `header-error.invalid-syntax` if the `field-key` or\n    /// `field-value` are syntactically invalid.\n    append: func(name: field-key, value: field-value) -> result<_, header-error>;\n\n    /// Retrieve the full set of keys and values in the Fields. Like the\n    /// constructor, the list represents each key-value pair.\n    ///\n    /// The outer list represents each key-value pair in the Fields. Keys\n    /// which have multiple values are represented by multiple entries in this\n    /// list with the same key.\n    entries: func() -> list<tuple<field-key,field-value>>;\n\n    /// Make a deep copy of the Fields. Equivelant in behavior to calling the\n    /// `fields` constructor on the return value of `entries`. The resulting\n    /// `fields` is mutable.\n    clone: func() -> fields;\n  }\n\n  /// Headers is an alias for Fields.\n  type headers = fields;\n\n  /// Trailers is an alias for Fields.\n  type trailers = fields;\n\n  /// Represents an incoming HTTP Request.\n  resource incoming-request {\n\n    /// Returns the method of the incoming request.\n    method: func() -> method;\n\n    /// Returns the path with query parameters from the request, as a string.\n    path-with-query: func() -> option<string>;\n\n    /// Returns the protocol scheme from the request.\n    scheme: func() -> option<scheme>;\n\n    /// Returns the authority from the request, if it was present.\n    authority: func() -> option<string>;\n\n    /// Get the `headers` associated with the request.\n    ///\n    /// The returned `headers` resource is immutable: `set`, `append`, and\n    /// `delete` operations will fail with `header-error.immutable`.\n    ///\n    /// The `headers` returned are a child resource: it must be dropped before\n    /// the parent `incoming-request` is dropped. Dropping this\n    /// `incoming-request` before all children are dropped will trap.\n    headers: func() -> headers;\n\n    /// Gives the `incoming-body` associated with this request. Will only\n    /// return success at most once, and subsequent calls will return error.\n    consume: func() -> result<incoming-body>;\n  }\n\n  /// Represents an outgoing HTTP Request.\n  resource outgoing-request {\n\n    /// Construct a new `outgoing-request` with a default `method` of `GET`, and\n    /// `none` values for `path-with-query`, `scheme`, and `authority`.\n    ///\n    /// * `headers` is the HTTP Headers for the Request.\n    ///\n    /// It is possible to construct, or manipulate with the accessor functions\n    /// below, an `outgoing-request` with an invalid combination of `scheme`\n    /// and `authority`, or `headers` which are not permitted to be sent.\n    /// It is the obligation of the `outgoing-handler.handle` implementation\n    /// to reject invalid constructions of `outgoing-request`.\n    constructor(\n      headers: headers\n    );\n\n    /// Returns the resource corresponding to the outgoing Body for this\n    /// Request.\n    ///\n    /// Returns success on the first call: the `outgoing-body` resource for\n    /// this `outgoing-request` can be retrieved at most once. Subsequent\n    /// calls will return error.\n    body: func() -> result<outgoing-body>;\n\n    /// Get the Method for the Request.\n    method: func() -> method;\n    /// Set the Method for the Request. Fails if the string present in a\n    /// `method.other` argument is not a syntactically valid method.\n    set-method: func(method: method) -> result;\n\n    /// Get the combination of the HTTP Path and Query for the Request.\n    /// When `none`, this represents an empty Path and empty Query.\n    path-with-query: func() -> option<string>;\n    /// Set the combination of the HTTP Path and Query for the Request.\n    /// When `none`, this represents an empty Path and empty Query. Fails is the\n    /// string given is not a syntactically valid path and query uri component.\n    set-path-with-query: func(path-with-query: option<string>) -> result;\n\n    /// Get the HTTP Related Scheme for the Request. When `none`, the\n    /// implementation may choose an appropriate default scheme.\n    scheme: func() -> option<scheme>;\n    /// Set the HTTP Related Scheme for the Request. When `none`, the\n    /// implementation may choose an appropriate default scheme. Fails if the\n    /// string given is not a syntactically valid uri scheme.\n    set-scheme: func(scheme: option<scheme>) -> result;\n\n    /// Get the HTTP Authority for the Request. A value of `none` may be used\n    /// with Related Schemes which do not require an Authority. The HTTP and\n    /// HTTPS schemes always require an authority.\n    authority: func() -> option<string>;\n    /// Set the HTTP Authority for the Request. A value of `none` may be used\n    /// with Related Schemes which do not require an Authority. The HTTP and\n    /// HTTPS schemes always require an authority. Fails if the string given is\n    /// not a syntactically valid uri authority.\n    set-authority: func(authority: option<string>) -> result;\n\n    /// Get the headers associated with the Request.\n    ///\n    /// The returned `headers` resource is immutable: `set`, `append`, and\n    /// `delete` operations will fail with `header-error.immutable`.\n    ///\n    /// This headers resource is a child: it must be dropped before the parent\n    /// `outgoing-request` is dropped, or its ownership is transfered to\n    /// another component by e.g. `outgoing-handler.handle`.\n    headers: func() -> headers;\n  }\n\n  /// Parameters for making an HTTP Request. Each of these parameters is\n  /// currently an optional timeout applicable to the transport layer of the\n  /// HTTP protocol.\n  ///\n  /// These timeouts are separate from any the user may use to bound a\n  /// blocking call to `wasi:io/poll.poll`.\n  resource request-options {\n    /// Construct a default `request-options` value.\n    constructor();\n\n    /// The timeout for the initial connect to the HTTP Server.\n    connect-timeout: func() -> option<duration>;\n\n    /// Set the timeout for the initial connect to the HTTP Server. An error\n    /// return value indicates that this timeout is not supported.\n    set-connect-timeout: func(duration: option<duration>) -> result;\n\n    /// The timeout for receiving the first byte of the Response body.\n    first-byte-timeout: func() -> option<duration>;\n\n    /// Set the timeout for receiving the first byte of the Response body. An\n    /// error return value indicates that this timeout is not supported.\n    set-first-byte-timeout: func(duration: option<duration>) -> result;\n\n    /// The timeout for receiving subsequent chunks of bytes in the Response\n    /// body stream.\n    between-bytes-timeout: func() -> option<duration>;\n\n    /// Set the timeout for receiving subsequent chunks of bytes in the Response\n    /// body stream. An error return value indicates that this timeout is not\n    /// supported.\n    set-between-bytes-timeout: func(duration: option<duration>) -> result;\n  }\n\n  /// Represents the ability to send an HTTP Response.\n  ///\n  /// This resource is used by the `wasi:http/incoming-handler` interface to\n  /// allow a Response to be sent corresponding to the Request provided as the\n  /// other argument to `incoming-handler.handle`.\n  resource response-outparam {\n\n    /// Set the value of the `response-outparam` to either send a response,\n    /// or indicate an error.\n    ///\n    /// This method consumes the `response-outparam` to ensure that it is\n    /// called at most once. If it is never called, the implementation\n    /// will respond with an error.\n    ///\n    /// The user may provide an `error` to `response` to allow the\n    /// implementation determine how to respond with an HTTP error response.\n    set: static func(\n      param: response-outparam,\n      response: result<outgoing-response, error-code>,\n    );\n  }\n\n  /// This type corresponds to the HTTP standard Status Code.\n  type status-code = u16;\n\n  /// Represents an incoming HTTP Response.\n  resource incoming-response {\n\n    /// Returns the status code from the incoming response.\n    status: func() -> status-code;\n\n    /// Returns the headers from the incoming response.\n    ///\n    /// The returned `headers` resource is immutable: `set`, `append`, and\n    /// `delete` operations will fail with `header-error.immutable`.\n    ///\n    /// This headers resource is a child: it must be dropped before the parent\n    /// `incoming-response` is dropped.\n    headers: func() -> headers;\n\n    /// Returns the incoming body. May be called at most once. Returns error\n    /// if called additional times.\n    consume: func() -> result<incoming-body>;\n  }\n\n  /// Represents an incoming HTTP Request or Response\'s Body.\n  ///\n  /// A body has both its contents - a stream of bytes - and a (possibly\n  /// empty) set of trailers, indicating that the full contents of the\n  /// body have been received. This resource represents the contents as\n  /// an `input-stream` and the delivery of trailers as a `future-trailers`,\n  /// and ensures that the user of this interface may only be consuming either\n  /// the body contents or waiting on trailers at any given time.\n  resource incoming-body {\n\n    /// Returns the contents of the body, as a stream of bytes.\n    ///\n    /// Returns success on first call: the stream representing the contents\n    /// can be retrieved at most once. Subsequent calls will return error.\n    ///\n    /// The returned `input-stream` resource is a child: it must be dropped\n    /// before the parent `incoming-body` is dropped, or consumed by\n    /// `incoming-body.finish`.\n    ///\n    /// This invariant ensures that the implementation can determine whether\n    /// the user is consuming the contents of the body, waiting on the\n    /// `future-trailers` to be ready, or neither. This allows for network\n    /// backpressure is to be applied when the user is consuming the body,\n    /// and for that backpressure to not inhibit delivery of the trailers if\n    /// the user does not read the entire body.\n    %stream: func() -> result<input-stream>;\n\n    /// Takes ownership of `incoming-body`, and returns a `future-trailers`.\n    /// This function will trap if the `input-stream` child is still alive.\n    finish: static func(this: incoming-body) -> future-trailers;\n  }\n\n  /// Represents a future which may eventaully return trailers, or an error.\n  ///\n  /// In the case that the incoming HTTP Request or Response did not have any\n  /// trailers, this future will resolve to the empty set of trailers once the\n  /// complete Request or Response body has been received.\n  resource future-trailers {\n\n    /// Returns a pollable which becomes ready when either the trailers have\n    /// been received, or an error has occured. When this pollable is ready,\n    /// the `get` method will return `some`.\n    subscribe: func() -> pollable;\n\n    /// Returns the contents of the trailers, or an error which occured,\n    /// once the future is ready.\n    ///\n    /// The outer `option` represents future readiness. Users can wait on this\n    /// `option` to become `some` using the `subscribe` method.\n    ///\n    /// The outer `result` is used to retrieve the trailers or error at most\n    /// once. It will be success on the first call in which the outer option\n    /// is `some`, and error on subsequent calls.\n    ///\n    /// The inner `result` represents that either the HTTP Request or Response\n    /// body, as well as any trailers, were received successfully, or that an\n    /// error occured receiving them. The optional `trailers` indicates whether\n    /// or not trailers were present in the body.\n    ///\n    /// When some `trailers` are returned by this method, the `trailers`\n    /// resource is immutable, and a child. Use of the `set`, `append`, or\n    /// `delete` methods will return an error, and the resource must be\n    /// dropped before the parent `future-trailers` is dropped.\n    get: func() -> option<result<result<option<trailers>, error-code>>>;\n  }\n\n  /// Represents an outgoing HTTP Response.\n  resource outgoing-response {\n\n    /// Construct an `outgoing-response`, with a default `status-code` of `200`.\n    /// If a different `status-code` is needed, it must be set via the\n    /// `set-status-code` method.\n    ///\n    /// * `headers` is the HTTP Headers for the Response.\n    constructor(headers: headers);\n\n    /// Get the HTTP Status Code for the Response.\n    status-code: func() -> status-code;\n\n    /// Set the HTTP Status Code for the Response. Fails if the status-code\n    /// given is not a valid http status code.\n    set-status-code: func(status-code: status-code) -> result;\n\n    /// Get the headers associated with the Request.\n    ///\n    /// The returned `headers` resource is immutable: `set`, `append`, and\n    /// `delete` operations will fail with `header-error.immutable`.\n    ///\n    /// This headers resource is a child: it must be dropped before the parent\n    /// `outgoing-request` is dropped, or its ownership is transfered to\n    /// another component by e.g. `outgoing-handler.handle`.\n    headers: func() -> headers;\n\n    /// Returns the resource corresponding to the outgoing Body for this Response.\n    ///\n    /// Returns success on the first call: the `outgoing-body` resource for\n    /// this `outgoing-response` can be retrieved at most once. Subsequent\n    /// calls will return error.\n    body: func() -> result<outgoing-body>;\n  }\n\n  /// Represents an outgoing HTTP Request or Response\'s Body.\n  ///\n  /// A body has both its contents - a stream of bytes - and a (possibly\n  /// empty) set of trailers, inducating the full contents of the body\n  /// have been sent. This resource represents the contents as an\n  /// `output-stream` child resource, and the completion of the body (with\n  /// optional trailers) with a static function that consumes the\n  /// `outgoing-body` resource, and ensures that the user of this interface\n  /// may not write to the body contents after the body has been finished.\n  ///\n  /// If the user code drops this resource, as opposed to calling the static\n  /// method `finish`, the implementation should treat the body as incomplete,\n  /// and that an error has occured. The implementation should propogate this\n  /// error to the HTTP protocol by whatever means it has available,\n  /// including: corrupting the body on the wire, aborting the associated\n  /// Request, or sending a late status code for the Response.\n  resource outgoing-body {\n\n    /// Returns a stream for writing the body contents.\n    ///\n    /// The returned `output-stream` is a child resource: it must be dropped\n    /// before the parent `outgoing-body` resource is dropped (or finished),\n    /// otherwise the `outgoing-body` drop or `finish` will trap.\n    ///\n    /// Returns success on the first call: the `output-stream` resource for\n    /// this `outgoing-body` may be retrieved at most once. Subsequent calls\n    /// will return error.\n    write: func() -> result<output-stream>;\n\n    /// Finalize an outgoing body, optionally providing trailers. This must be\n    /// called to signal that the response is complete. If the `outgoing-body`\n    /// is dropped without calling `outgoing-body.finalize`, the implementation\n    /// should treat the body as corrupted.\n    ///\n    /// Fails if the body\'s `outgoing-request` or `outgoing-response` was\n    /// constructed with a Content-Length header, and the contents written\n    /// to the body (via `write`) does not match the value given in the\n    /// Content-Length.\n    finish: static func(\n      this: outgoing-body,\n      trailers: option<trailers>\n    ) -> result<_, error-code>;\n  }\n\n  /// Represents a future which may eventaully return an incoming HTTP\n  /// Response, or an error.\n  ///\n  /// This resource is returned by the `wasi:http/outgoing-handler` interface to\n  /// provide the HTTP Response corresponding to the sent Request.\n  resource future-incoming-response {\n    /// Returns a pollable which becomes ready when either the Response has\n    /// been received, or an error has occured. When this pollable is ready,\n    /// the `get` method will return `some`.\n    subscribe: func() -> pollable;\n\n    /// Returns the incoming HTTP Response, or an error, once one is ready.\n    ///\n    /// The outer `option` represents future readiness. Users can wait on this\n    /// `option` to become `some` using the `subscribe` method.\n    ///\n    /// The outer `result` is used to retrieve the response or error at most\n    /// once. It will be success on the first call in which the outer option\n    /// is `some`, and error on subsequent calls.\n    ///\n    /// The inner `result` represents that either the incoming HTTP Response\n    /// status and headers have recieved successfully, or that an error\n    /// occured. Errors may also occur while consuming the response body,\n    /// but those will be reported by the `incoming-body` and its\n    /// `output-stream` child.\n    get: func() -> option<result<result<incoming-response, error-code>>>;\n\n  }\n}\n";
const _: &str = "package faaas:runjs;\n\nworld runjs {\n   include wasi:http/proxy@0.2.0;\n}\n";
const _: &str = "package faaas:faaastime;\n\nworld faaastime {\n   include faaas:runjs/runjs;\n}\n";
struct MyState {
    name: String,
}
fn main() -> Result<()> {
    let mut store: Store<()> = Store::default();
    let module = Module::from_file(
        store.engine(),
        "../runjs/target/wasm32-wasi/release/runjs.wasm",
    )?;
    let instance = Instance::new(&mut store, &module, &[])?;
    Ok(())
}
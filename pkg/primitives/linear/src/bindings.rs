// Generated by `wit-bindgen` 0.24.0. DO NOT EDIT!
// Options used:
pub type TaskContext = faaas::task::types::TaskContext;
pub type TaskOutput = faaas::task::types::TaskOutput;
pub type TaskError = faaas::task::types::TaskError;
#[allow(unused_unsafe, clippy::all)]
pub fn task_fst(ctx: &TaskContext) -> Result<TaskOutput, TaskError> {
    unsafe {
        #[repr(align(4))]
        struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 8]);
        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
        #[cfg(target_arch = "wasm32")]
        #[link(wasm_import_module = "$root")]
        extern "C" {
            #[link_name = "task-fst"]
            fn wit_import(_: i32, _: *mut u8);
        }

        #[cfg(not(target_arch = "wasm32"))]
        fn wit_import(_: i32, _: *mut u8) {
            unreachable!()
        }
        wit_import((ctx).handle() as i32, ptr0);
        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
        match l1 {
            0 => {
                let e = {
                    let l2 = *ptr0.add(4).cast::<i32>();

                    faaas::task::types::TaskOutput::from_handle(l2 as u32)
                };
                Ok(e)
            }
            1 => {
                let e = {
                    let l3 = *ptr0.add(4).cast::<i32>();

                    faaas::task::types::TaskError::from_handle(l3 as u32)
                };
                Err(e)
            }
            _ => _rt::invalid_enum_discriminant(),
        }
    }
}
#[allow(unused_unsafe, clippy::all)]
pub fn task_snd(ctx: &TaskContext) -> Result<TaskOutput, TaskError> {
    unsafe {
        #[repr(align(4))]
        struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 8]);
        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
        #[cfg(target_arch = "wasm32")]
        #[link(wasm_import_module = "$root")]
        extern "C" {
            #[link_name = "task-snd"]
            fn wit_import(_: i32, _: *mut u8);
        }

        #[cfg(not(target_arch = "wasm32"))]
        fn wit_import(_: i32, _: *mut u8) {
            unreachable!()
        }
        wit_import((ctx).handle() as i32, ptr0);
        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
        match l1 {
            0 => {
                let e = {
                    let l2 = *ptr0.add(4).cast::<i32>();

                    faaas::task::types::TaskOutput::from_handle(l2 as u32)
                };
                Ok(e)
            }
            1 => {
                let e = {
                    let l3 = *ptr0.add(4).cast::<i32>();

                    faaas::task::types::TaskError::from_handle(l3 as u32)
                };
                Err(e)
            }
            _ => _rt::invalid_enum_discriminant(),
        }
    }
}
#[allow(dead_code)]
pub mod faaas {
    #[allow(dead_code)]
    pub mod task {
        #[allow(dead_code, clippy::all)]
        pub mod types {
            #[used]
            #[doc(hidden)]
            #[cfg(target_arch = "wasm32")]
            static __FORCE_SECTION_REF: fn() =
                super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;

            #[derive(Debug)]
            #[repr(transparent)]
            pub struct TaskContext {
                handle: _rt::Resource<TaskContext>,
            }

            impl TaskContext {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self {
                        handle: _rt::Resource::from_handle(handle),
                    }
                }

                #[doc(hidden)]
                pub fn take_handle(&self) -> u32 {
                    _rt::Resource::take_handle(&self.handle)
                }

                #[doc(hidden)]
                pub fn handle(&self) -> u32 {
                    _rt::Resource::handle(&self.handle)
                }
            }

            unsafe impl _rt::WasmResource for TaskContext {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();

                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(wasm_import_module = "faaas:task/types")]
                        extern "C" {
                            #[link_name = "[resource-drop]task-context"]
                            fn drop(_: u32);
                        }

                        drop(_handle);
                    }
                }
            }

            #[repr(u8)]
            #[derive(Clone, Copy, Eq, PartialEq)]
            pub enum TaskStatus {
                Success,
                Error,
            }
            impl ::core::fmt::Debug for TaskStatus {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    match self {
                        TaskStatus::Success => f.debug_tuple("TaskStatus::Success").finish(),
                        TaskStatus::Error => f.debug_tuple("TaskStatus::Error").finish(),
                    }
                }
            }

            impl TaskStatus {
                pub(crate) unsafe fn _lift(val: u8) -> TaskStatus {
                    if !cfg!(debug_assertions) {
                        return ::core::mem::transmute(val);
                    }

                    match val {
                        0 => TaskStatus::Success,
                        1 => TaskStatus::Error,

                        _ => panic!("invalid enum discriminant"),
                    }
                }
            }

            #[derive(Debug)]
            #[repr(transparent)]
            pub struct TaskOutput {
                handle: _rt::Resource<TaskOutput>,
            }

            impl TaskOutput {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self {
                        handle: _rt::Resource::from_handle(handle),
                    }
                }

                #[doc(hidden)]
                pub fn take_handle(&self) -> u32 {
                    _rt::Resource::take_handle(&self.handle)
                }

                #[doc(hidden)]
                pub fn handle(&self) -> u32 {
                    _rt::Resource::handle(&self.handle)
                }
            }

            unsafe impl _rt::WasmResource for TaskOutput {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();

                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(wasm_import_module = "faaas:task/types")]
                        extern "C" {
                            #[link_name = "[resource-drop]task-output"]
                            fn drop(_: u32);
                        }

                        drop(_handle);
                    }
                }
            }

            #[derive(Debug)]
            #[repr(transparent)]
            pub struct TaskError {
                handle: _rt::Resource<TaskError>,
            }

            impl TaskError {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self {
                        handle: _rt::Resource::from_handle(handle),
                    }
                }

                #[doc(hidden)]
                pub fn take_handle(&self) -> u32 {
                    _rt::Resource::take_handle(&self.handle)
                }

                #[doc(hidden)]
                pub fn handle(&self) -> u32 {
                    _rt::Resource::handle(&self.handle)
                }
            }

            unsafe impl _rt::WasmResource for TaskError {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();

                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(wasm_import_module = "faaas:task/types")]
                        extern "C" {
                            #[link_name = "[resource-drop]task-error"]
                            fn drop(_: u32);
                        }

                        drop(_handle);
                    }
                }
            }

            impl TaskContext {
                #[allow(unused_unsafe, clippy::all)]
                pub fn set(&self, key: &str, value: &str) {
                    unsafe {
                        let vec0 = key;
                        let ptr0 = vec0.as_ptr().cast::<u8>();
                        let len0 = vec0.len();
                        let vec1 = value;
                        let ptr1 = vec1.as_ptr().cast::<u8>();
                        let len1 = vec1.len();

                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "faaas:task/types")]
                        extern "C" {
                            #[link_name = "[method]task-context.set"]
                            fn wit_import(_: i32, _: *mut u8, _: usize, _: *mut u8, _: usize);
                        }

                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8, _: usize, _: *mut u8, _: usize) {
                            unreachable!()
                        }
                        wit_import(
                            (self).handle() as i32,
                            ptr0.cast_mut(),
                            len0,
                            ptr1.cast_mut(),
                            len1,
                        );
                    }
                }
            }
            impl TaskContext {
                #[allow(unused_unsafe, clippy::all)]
                pub fn get(&self, key: &str) -> _rt::String {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 8]);
                        let vec0 = key;
                        let ptr0 = vec0.as_ptr().cast::<u8>();
                        let len0 = vec0.len();
                        let ptr1 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "faaas:task/types")]
                        extern "C" {
                            #[link_name = "[method]task-context.get"]
                            fn wit_import(_: i32, _: *mut u8, _: usize, _: *mut u8);
                        }

                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8, _: usize, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0.cast_mut(), len0, ptr1);
                        let l2 = *ptr1.add(0).cast::<*mut u8>();
                        let l3 = *ptr1.add(4).cast::<usize>();
                        let len4 = l3;
                        let bytes4 = _rt::Vec::from_raw_parts(l2.cast(), len4, len4);
                        _rt::string_lift(bytes4)
                    }
                }
            }
            impl TaskContext {
                #[allow(unused_unsafe, clippy::all)]
                pub fn clone(&self) -> TaskContext {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "faaas:task/types")]
                        extern "C" {
                            #[link_name = "[method]task-context.clone"]
                            fn wit_import(_: i32) -> i32;
                        }

                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import((self).handle() as i32);
                        TaskContext::from_handle(ret as u32)
                    }
                }
            }
            impl TaskContext {
                #[allow(unused_unsafe, clippy::all)]
                pub fn marge(fst: &TaskContext, snd: &TaskContext) -> TaskContext {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "faaas:task/types")]
                        extern "C" {
                            #[link_name = "[static]task-context.marge"]
                            fn wit_import(_: i32, _: i32) -> i32;
                        }

                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import((fst).handle() as i32, (snd).handle() as i32);
                        TaskContext::from_handle(ret as u32)
                    }
                }
            }
            impl TaskOutput {
                #[allow(unused_unsafe, clippy::all)]
                pub fn new() -> Self {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "faaas:task/types")]
                        extern "C" {
                            #[link_name = "[constructor]task-output"]
                            fn wit_import() -> i32;
                        }

                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import() -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import();
                        TaskOutput::from_handle(ret as u32)
                    }
                }
            }
            impl TaskOutput {
                #[allow(unused_unsafe, clippy::all)]
                pub fn set_status(&self, status: TaskStatus) {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "faaas:task/types")]
                        extern "C" {
                            #[link_name = "[method]task-output.set-status"]
                            fn wit_import(_: i32, _: i32);
                        }

                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, status.clone() as i32);
                    }
                }
            }
            impl TaskOutput {
                #[allow(unused_unsafe, clippy::all)]
                pub fn get_status(&self) -> TaskStatus {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "faaas:task/types")]
                        extern "C" {
                            #[link_name = "[method]task-output.get-status"]
                            fn wit_import(_: i32) -> i32;
                        }

                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import((self).handle() as i32);
                        TaskStatus::_lift(ret as u8)
                    }
                }
            }
        }
    }
}
#[allow(dead_code)]
pub mod exports {
    #[allow(dead_code)]
    pub mod faaas {
        #[allow(dead_code)]
        pub mod task {
            #[allow(dead_code, clippy::all)]
            pub mod callable {
                #[used]
                #[doc(hidden)]
                #[cfg(target_arch = "wasm32")]
                static __FORCE_SECTION_REF: fn() =
                    super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                pub type TaskContext = super::super::super::super::faaas::task::types::TaskContext;
                pub type TaskError = super::super::super::super::faaas::task::types::TaskError;
                pub type TaskOutput = super::super::super::super::faaas::task::types::TaskOutput;
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_call_cabi<T: Guest>(arg0: i32) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")]
                    _rt::run_ctors_once();
                    let handle0;
                    let result1 = T::call({
                        handle0 = super::super::super::super::faaas::task::types::TaskContext::from_handle(arg0 as u32);
                        &handle0
                    });
                    let ptr2 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result1 {
                        Ok(e) => {
                            *ptr2.add(0).cast::<u8>() = (0i32) as u8;
                            *ptr2.add(4).cast::<i32>() = (e).take_handle() as i32;
                        }
                        Err(e) => {
                            *ptr2.add(0).cast::<u8>() = (1i32) as u8;
                            *ptr2.add(4).cast::<i32>() = (e).take_handle() as i32;
                        }
                    };
                    ptr2
                }
                pub trait Guest {
                    fn call(ctx: &TaskContext) -> Result<TaskOutput, TaskError>;
                }
                #[doc(hidden)]

                macro_rules! __export_faaas_task_callable_cabi{
        ($ty:ident with_types_in $($path_to_types:tt)*) => (const _: () = {

          #[export_name = "faaas:task/callable#call"]
          unsafe extern "C" fn export_call(arg0: i32,) -> *mut u8 {
            $($path_to_types)*::_export_call_cabi::<$ty>(arg0)
          }
        };);
      }
                #[doc(hidden)]
                pub(crate) use __export_faaas_task_callable_cabi;
                #[repr(align(4))]
                struct _RetArea([::core::mem::MaybeUninit<u8>; 8]);
                static mut _RET_AREA: _RetArea = _RetArea([::core::mem::MaybeUninit::uninit(); 8]);
            }
        }
    }
}
mod _rt {

    use core::fmt;
    use core::marker;
    use core::sync::atomic::{AtomicU32, Ordering::Relaxed};

    /// A type which represents a component model resource, either imported or
    /// exported into this component.
    ///
    /// This is a low-level wrapper which handles the lifetime of the resource
    /// (namely this has a destructor). The `T` provided defines the component model
    /// intrinsics that this wrapper uses.
    ///
    /// One of the chief purposes of this type is to provide `Deref` implementations
    /// to access the underlying data when it is owned.
    ///
    /// This type is primarily used in generated code for exported and imported
    /// resources.
    #[repr(transparent)]
    pub struct Resource<T: WasmResource> {
        // NB: This would ideally be `u32` but it is not. The fact that this has
        // interior mutability is not exposed in the API of this type except for the
        // `take_handle` method which is supposed to in theory be private.
        //
        // This represents, almost all the time, a valid handle value. When it's
        // invalid it's stored as `u32::MAX`.
        handle: AtomicU32,
        _marker: marker::PhantomData<T>,
    }

    /// A trait which all wasm resources implement, namely providing the ability to
    /// drop a resource.
    ///
    /// This generally is implemented by generated code, not user-facing code.
    pub unsafe trait WasmResource {
        /// Invokes the `[resource-drop]...` intrinsic.
        unsafe fn drop(handle: u32);
    }

    impl<T: WasmResource> Resource<T> {
        #[doc(hidden)]
        pub unsafe fn from_handle(handle: u32) -> Self {
            debug_assert!(handle != u32::MAX);
            Self {
                handle: AtomicU32::new(handle),
                _marker: marker::PhantomData,
            }
        }

        /// Takes ownership of the handle owned by `resource`.
        ///
        /// Note that this ideally would be `into_handle` taking `Resource<T>` by
        /// ownership. The code generator does not enable that in all situations,
        /// unfortunately, so this is provided instead.
        ///
        /// Also note that `take_handle` is in theory only ever called on values
        /// owned by a generated function. For example a generated function might
        /// take `Resource<T>` as an argument but then call `take_handle` on a
        /// reference to that argument. In that sense the dynamic nature of
        /// `take_handle` should only be exposed internally to generated code, not
        /// to user code.
        #[doc(hidden)]
        pub fn take_handle(resource: &Resource<T>) -> u32 {
            resource.handle.swap(u32::MAX, Relaxed)
        }

        #[doc(hidden)]
        pub fn handle(resource: &Resource<T>) -> u32 {
            resource.handle.load(Relaxed)
        }
    }

    impl<T: WasmResource> fmt::Debug for Resource<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("Resource")
                .field("handle", &self.handle)
                .finish()
        }
    }

    impl<T: WasmResource> Drop for Resource<T> {
        fn drop(&mut self) {
            unsafe {
                match self.handle.load(Relaxed) {
                    // If this handle was "taken" then don't do anything in the
                    // destructor.
                    u32::MAX => {}

                    // ... but otherwise do actually destroy it with the imported
                    // component model intrinsic as defined through `T`.
                    other => T::drop(other),
                }
            }
        }
    }
    pub use alloc_crate::string::String;
    pub use alloc_crate::vec::Vec;
    pub unsafe fn string_lift(bytes: Vec<u8>) -> String {
        if cfg!(debug_assertions) {
            String::from_utf8(bytes).unwrap()
        } else {
            String::from_utf8_unchecked(bytes)
        }
    }
    pub unsafe fn invalid_enum_discriminant<T>() -> T {
        if cfg!(debug_assertions) {
            panic!("invalid enum discriminant")
        } else {
            core::hint::unreachable_unchecked()
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub fn run_ctors_once() {
        wit_bindgen_rt::run_ctors_once();
    }
    extern crate alloc as alloc_crate;
}

/// Generates `#[no_mangle]` functions to export the specified type as the
/// root implementation of all generated traits.
///
/// For more information see the documentation of `wit_bindgen::generate!`.
///
/// ```rust
/// # macro_rules! export{ ($($t:tt)*) => (); }
/// # trait Guest {}
/// struct MyType;
///
/// impl Guest for MyType {
///     // ...
/// }
///
/// export!(MyType);
/// ```
#[allow(unused_macros)]
#[doc(hidden)]

macro_rules! __export_linear_impl {
  ($ty:ident) => (self::export!($ty with_types_in self););
  ($ty:ident with_types_in $($path_to_types_root:tt)*) => (
  $($path_to_types_root)*::exports::faaas::task::callable::__export_faaas_task_callable_cabi!($ty with_types_in $($path_to_types_root)*::exports::faaas::task::callable);
  )
}
#[doc(inline)]
pub(crate) use __export_linear_impl as export;

#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.24.0:linear:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 869] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\xe8\x05\x01A\x02\x01\
A\x11\x01B\x17\x04\0\x0ctask-context\x03\x01\x01m\x02\x07success\x05error\x04\0\x0b\
task-status\x03\0\x01\x04\0\x0btask-output\x03\x01\x04\0\x0atask-error\x03\x01\x01\
h\0\x01@\x03\x04self\x05\x03keys\x05values\x01\0\x04\0\x18[method]task-context.s\
et\x01\x06\x01@\x02\x04self\x05\x03keys\0s\x04\0\x18[method]task-context.get\x01\
\x07\x01i\0\x01@\x01\x04self\x05\0\x08\x04\0\x1a[method]task-context.clone\x01\x09\
\x01@\x02\x03fst\x05\x03snd\x05\0\x08\x04\0\x1a[static]task-context.marge\x01\x0a\
\x01i\x03\x01@\0\0\x0b\x04\0\x18[constructor]task-output\x01\x0c\x01h\x03\x01@\x02\
\x04self\x0d\x06status\x02\x01\0\x04\0\x1e[method]task-output.set-status\x01\x0e\
\x01@\x01\x04self\x0d\0\x02\x04\0\x1e[method]task-output.get-status\x01\x0f\x03\x01\
\x10faaas:task/types\x05\0\x02\x03\0\0\x0ctask-context\x03\0\x0ctask-context\x03\
\0\x01\x02\x03\0\0\x0btask-output\x03\0\x0btask-output\x03\0\x03\x02\x03\0\0\x0a\
task-error\x03\0\x0atask-error\x03\0\x05\x01h\x02\x01i\x04\x01i\x06\x01j\x01\x08\
\x01\x09\x01@\x01\x03ctx\x07\0\x0a\x03\0\x08task-fst\x01\x0b\x03\0\x08task-snd\x01\
\x0b\x01B\x0c\x02\x03\x02\x01\x01\x04\0\x0ctask-context\x03\0\0\x02\x03\x02\x01\x05\
\x04\0\x0atask-error\x03\0\x02\x02\x03\x02\x01\x03\x04\0\x0btask-output\x03\0\x04\
\x01h\x01\x01i\x05\x01i\x03\x01j\x01\x07\x01\x08\x01@\x01\x03ctx\x06\0\x09\x04\0\
\x04call\x01\x0a\x04\x01\x13faaas:task/callable\x05\x0c\x04\x01\x13faaas:linear/\
linear\x04\0\x0b\x0c\x01\0\x06linear\x03\0\0\0G\x09producers\x01\x0cprocessed-by\
\x02\x0dwit-component\x070.202.0\x10wit-bindgen-rust\x060.24.0";

#[inline(never)]
#[doc(hidden)]
#[cfg(target_arch = "wasm32")]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}

use rquickjs::class::Trace;
use rquickjs::{class, methods, module};

use crate::bindings::wasi::clocks::monotonic_clock::now;

#[derive(Clone, Trace)]
#[class(rename_all = "camelCase", rename = "performance")]
pub struct Performance {}

#[methods]
impl Performance {
    #[qjs(constructor)]
    pub fn new() -> Self {
        Self {}
    }

    #[qjs(static)]
    pub fn now() -> f64 {
        let nano_sec = now();
        let micro_sec = (nano_sec / 1000) as f64;

        micro_sec / 1e3
    }
}

#[module(rename_types = "lowercase")]
pub mod perf_hooks_mod {
    pub use super::Performance;
}

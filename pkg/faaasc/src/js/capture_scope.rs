use std::collections::HashSet;

use swc_ecma_ast::Str;

struct CapturedScope {
    ids: HashSet<Str>,
}

impl CapturedScope {
    pub fn new() -> Self {
        Self {
            ids: Default::default(),
        }
    }
}

pub trait CaptureScope {
    fn capture_scope(&self) -> CapturedScope;
}

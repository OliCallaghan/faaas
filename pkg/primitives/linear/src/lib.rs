#[allow(warnings)]
mod bindings;

use std::sync::RwLock;

use bindings::exports::faaas::task::callable::Guest as CallableGuest;
use bindings::exports::faaas::task::identifiable::Guest as IdentifiableGuest;
use bindings::{identify_fst, identify_snd, task_fst, task_snd, TaskContext, TaskError};

struct Linear {}

static WHOAMI: RwLock<Option<String>> = RwLock::new(None);

impl CallableGuest for Linear {
    fn call(ctx: TaskContext) -> Result<TaskContext, TaskError> {
        let ctx_fst = ctx.lens(&identify_fst());
        let ctx_fst = task_fst(ctx_fst)?;

        let ctx_snd = ctx_fst.lens(&identify_snd());
        let ctx_snd = task_snd(ctx_snd)?;

        Ok(ctx_snd)
    }
}

impl IdentifiableGuest for Linear {
    fn identify() -> String {
        WHOAMI
            .write()
            .unwrap()
            .get_or_insert_with(compute_identity)
            .clone()
    }
}

fn compute_identity() -> String {
    format!("({}) ; ({})", identify_fst(), identify_snd())
}

bindings::export!(Linear with_types_in bindings);

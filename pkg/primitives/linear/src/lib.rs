#[allow(warnings)]
mod bindings;

use bindings::exports::faaas::task::callable::Guest;
use bindings::{task_fst, task_snd, TaskContext, TaskError};

struct Linear {}

impl Guest for Linear {
    fn call(ctx: TaskContext) -> Result<TaskContext, TaskError> {
        task_snd(task_fst(ctx)?)
    }
}

bindings::export!(Linear with_types_in bindings);

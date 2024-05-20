#[allow(warnings)]
mod bindings;

// use bindings::{task_fst, task_snd, Guest, TaskError, TaskInput, TaskOutput};
// use bindings::exports::faaas::task::callable::Guest;
use bindings::exports::faaas::task::callable::Guest;
use bindings::{task_fst, task_snd, TaskContext, TaskError, TaskOutput};

struct Linear {}

impl Guest for Linear {
    fn call(input: &TaskContext) -> Result<TaskOutput, TaskError> {
        task_fst(input)?;
        task_snd(input)
    }
}

bindings::export!(Linear with_types_in bindings);

#[allow(warnings)]
mod bindings;

use bindings::exports::faaas::task::callable::Guest;
use bindings::faaas::task::types::{TaskContext, TaskError, TaskOutput};

struct Task {}

impl Guest for Task {
    fn call(ctx: &TaskContext) -> Result<TaskOutput, TaskError> {
        ctx.set("a", ctx.get("a") + 1);

        Ok(TaskOutput::new())
    }
}

bindings::export!(Task with_types_in bindings);

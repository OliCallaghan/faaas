#[allow(warnings)]
mod bindings;

use bindings::exports::faaas::task::callable::Guest;
use bindings::faaas::task::types::{TaskContext, TaskError};

struct Task {}

impl Guest for Task {
    fn call(ctx: TaskContext) -> Result<TaskContext, TaskError> {
        ctx.set("a", ctx.get("a") + 1);

        Ok(ctx)
    }
}

bindings::export!(Task with_types_in bindings);

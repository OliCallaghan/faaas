#[allow(warnings)]
mod bindings;

use bindings::exports::faaas::task::callable::Guest as CallableGuest;
use bindings::exports::faaas::task::identifiable::Guest as IdentifiableGuest;
use bindings::faaas::task::types::{TaskContext, TaskError};

struct Task {}

impl CallableGuest for Task {
    fn call(ctx: TaskContext) -> Result<TaskContext, TaskError> {
        Ok(ctx)
    }
}

impl IdentifiableGuest for Task {
    fn identify() -> String {
        "task:1".to_string()
    }
}

bindings::export!(Task with_types_in bindings);

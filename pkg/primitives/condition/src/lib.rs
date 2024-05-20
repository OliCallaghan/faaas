#[allow(warnings)]
mod bindings;

use bindings::exports::faaas::task::callable::Guest;
use bindings::faaas::task::types::TaskStatus;
use bindings::{task_branch_false, task_branch_true, task_condition, TaskContext, TaskError};

struct Condition {}

impl Guest for Condition {
    fn call(ctx: TaskContext) -> Result<TaskContext, TaskError> {
        let ctx = task_condition(ctx)?;

        let ctx = match ctx.get_status() {
            TaskStatus::Success => task_branch_true(ctx)?,
            TaskStatus::Error => task_branch_false(ctx)?,
            _ => panic!("No status!"),
        };

        Ok(ctx)
    }
}

bindings::export!(Condition with_types_in bindings);

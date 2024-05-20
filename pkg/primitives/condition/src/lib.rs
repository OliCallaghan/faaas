#[allow(warnings)]
mod bindings;

use bindings::exports::faaas::task::callable::Guest;
use bindings::{
    task_branch_false, task_branch_true, task_condition, TaskContext, TaskError, TaskOutput,
};

struct Condition {}

impl Guest for Condition {
    fn call(ctx: &TaskContext) -> Result<TaskOutput, TaskError> {
        let cond_output = task_condition(ctx)?;

        let _branch_true_output = task_branch_true(ctx)?;
        let branch_false_output = task_branch_false(ctx)?;

        Ok(branch_false_output)
    }
}

bindings::export!(Condition with_types_in bindings);

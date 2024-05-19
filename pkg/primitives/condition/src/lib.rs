#[allow(warnings)]
mod bindings;

use bindings::Guest;

struct Condition {}

impl Guest for Condition {
    fn call(input: bindings::TaskInput) -> Result<bindings::TaskOutput, bindings::TaskError> {
        let cond_output = bindings::faaas::condition::condition_impl::task_condition(input)?;

        let _branch_true_output =
            bindings::faaas::condition::condition_impl::task_branch_true(cond_output.as_input())?;
        let branch_false_output =
            bindings::faaas::condition::condition_impl::task_branch_false(cond_output.as_input())?;

        Ok(branch_false_output)
    }
}

bindings::export!(Condition with_types_in bindings);

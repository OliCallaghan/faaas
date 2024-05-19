#[allow(warnings)]
mod bindings;

use bindings::Guest;

struct Linear {}

impl Guest for Linear {
    fn call(input: bindings::TaskInput) -> Result<bindings::TaskOutput, bindings::TaskError> {
        let output = bindings::faaas::linear::linear_impl::task_fst(input)?;
        let input = output.as_input();

        bindings::faaas::linear::linear_impl::task_snd(input)
    }
}

bindings::export!(Linear with_types_in bindings);

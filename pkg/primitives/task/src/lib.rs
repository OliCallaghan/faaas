#[allow(warnings)]
mod bindings;

use bindings::Guest;

struct Task {}

impl Guest for Task {
    fn call(_input: bindings::TaskInput) -> Result<bindings::TaskOutput, bindings::TaskError> {
        Ok(bindings::TaskOutput::new())
    }
}

bindings::export!(Task with_types_in bindings);

#[allow(warnings)]
mod bindings;

use bindings::exports::faaas::task::callable::Guest as CallableGuest;
use bindings::exports::faaas::task::identifiable::Guest as IdentifiableGuest;
use bindings::faaas::task::types::{TaskContext, TaskError, Value};

struct Task {}

impl CallableGuest for Task {
    fn call(ctx: TaskContext) -> Result<TaskContext, TaskError> {
        let a_old = match ctx.get("a") {
            Some(Value::U32Val(a)) => a,
            _ => 0,
        };

        ctx.set("a", &Value::U32Val(a_old + 1));

        // let next_task = "io/sql/pg";
        // let next_args = [
        //     Value::StrVal("SELECT * FROM pets".into()),
        //     Value::StrVal("tasks/dec_pet".into()),
        // ];
        // ctx.set_continuation_id(next_task);
        // ctx.set_continuation_args(&next_args);

        Ok(ctx)
    }
}

impl IdentifiableGuest for Task {
    fn identify() -> String {
        "get_pet".to_string()
    }
}

bindings::export!(Task with_types_in bindings);

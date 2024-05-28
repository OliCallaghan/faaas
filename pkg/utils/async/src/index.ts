type Value = string | number | boolean;
type TaskContext = Record<string, Value>;

type Continuation = {
  status: "continuation";
  taskId: string;
  args: Value[];
  ctx: TaskContext;
};

type Complete = {
  status: "complete";
  ctx: TaskContext;
};

type Result = Continuation | Complete;

export function continuation(
  taskId: string,
  args: Value[],
  ctx: TaskContext,
): Result {
  return {
    status: "continuation",
    taskId,
    args,
    ctx,
  };
}

export function result(ctx: TaskContext) {
  return {
    status: "complete",
    ctx,
  };
}

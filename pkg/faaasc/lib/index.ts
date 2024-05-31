export type Value = string | number | boolean;
export type TaskContext = Record<string, any>;
export type TaskContextState = TaskContext;

export type Continuation = {
  what: "continuation";
  taskId: string;
  args: Value[];
  ctx: TaskContext;
};

export type Result = {
  what: "complete";
  ctx: TaskContext;
};

export function continuation(
  taskId: string,
  args: Value[],
  ctx: TaskContext,
): Continuation {
  return {
    what: "continuation",
    taskId,
    args,
    ctx,
  };
}

export function result(ctx: TaskContext): Result {
  return {
    what: "complete",
    ctx,
  };
}

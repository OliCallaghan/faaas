// This is the type which any "use async" function must implement.
export type Task = {
  proxy: string;
};

// Denotes a continuation to a proxy
export function continuation(
  task: Task,
  taskArgs: string[],
  taskScope: Record<string, any>,
) {
  return {
    status: "continuation",
    taskId: task.proxy,
    taskArgs,
    taskScope,
  } as const;
}

// Denotes a result to the function invocation
export function result(data: any) {
  return {
    status: "done",
    data: JSON.stringify(data),
  } as const;
}

/**
    This represents the function invocation context.
    When functions are split, the result of the
    async proxy is stored in data.
*/
export type FaaascInternalContext = {
  id: string;
  data: string;
};

// Represents the state of the function invocation. Stores free variables
export type FaaascInternalState = Record<string, any>;

// This is the type of a split function handler. This is **not** what a developer implements.
export type Handler = (
  ctx: FaaascInternalContext,
  state: FaaascInternalState,
) => Promise<ReturnType<typeof continuation> | ReturnType<typeof result>>;

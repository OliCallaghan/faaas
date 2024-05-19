import { Universe } from "@faaas/universe";

export interface TaskContext {}
export interface WorkflowContext extends TaskContext {}

export interface TaskResult {
  ctx(): TaskContext;
}

export interface TaskInvocation {
  result(): TaskResult;
}

export function registerCall<TaskIds extends string>(
  universe: Universe<TaskIds>,
) {
  return function call(taskId: TaskIds, ctx: TaskContext) {
    console.log(`Calling ${taskId.toString()} with ${ctx}`);

    return {
      result: () => ({
        ctx: () => ({}),
      }),
    };
  };
}

export function caller<TaskIds>(
  fn: keyof Universe<TaskIds>,
  ctx: TaskContext,
): TaskInvocation {
  console.log(`Calling ${fn.toString()} with ${ctx}`);

  return {
    result: () => ({
      ctx: () => ({}),
    }),
  };
}

interface ConditionBranch {
  which: "true" | "false";
  branch(): TaskInvocation;
}

export function cond(
  res: TaskResult,
  ...branches: ConditionBranch[]
): TaskInvocation {
  return {
    result: () => ({
      ctx: () => ({}),
    }),
  };
}

export function whenTrue(branch: ConditionBranch["branch"]): ConditionBranch {
  return {
    which: "true",
    branch,
  };
}

export function whenFalse(branch: ConditionBranch["branch"]): ConditionBranch {
  return {
    which: "false",
    branch,
  };
}

type TI = TaskInvocation;
type TR = TaskResult;
export function wait(i1: TI): [TR];
export function wait(i1: TI, i2: TI): [TR, TR];
export function wait(i1: TI, i2: TI, i3: TI): [TR, TR, TR];
export function wait(i1: TI, i2: TI, i3: TI, i4: TI): [TR, TR, TR, TR];
export function wait(...invocations: TaskInvocation[]): TaskResult[] {
  return invocations.map((i) => i.result());
}

export type Workflow = (ctx: WorkflowContext) => TaskResult;

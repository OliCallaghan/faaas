import { Universe } from "@faaas/universe";

class Task {
  constructor(private readonly taskId: string) {}
}

export class Context {
  private constructor(
    private readonly parent: Context | undefined,
    private readonly taskId: Task | undefined,
  ) {}

  public static new() {
    return new Context(undefined, undefined);
  }

  private sequential(task: Task): Context {
    return new Context(this, task);
  }

  public call(taskId: string): Context {
    const task = new Task(taskId);

    return this.sequential(task);
  }
}

export function registerCall<TaskIds extends string>(
  _universe: Universe<TaskIds>,
) {
  return function call(taskId: TaskIds, ctx: Context): Context {
    console.log(`Calling ${taskId.toString()} with ${ctx}`);

    return ctx.call(taskId);
  };
}

interface ConditionBranch {
  which: "true" | "false";
  branch(ctx: Context): Context;
}

export function cond(ctx: Context, ...branches: ConditionBranch[]): Context {
  return ctx;
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

export type Workflow = (ctx: Context) => Context;

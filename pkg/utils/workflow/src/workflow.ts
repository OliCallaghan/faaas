import { Universe } from "@faaas/universe";

abstract class Node {
  public abstract serialize(): object;
}

class Linear {
  constructor(
    private readonly t1: Node,
    private readonly t2: Node,
  ) {}

  public serialize(): object {
    return { linear: [this.t1.serialize(), this.t2.serialize()] };
  }
}

class Task {
  constructor(private readonly taskId: string) {}

  public serialize(): object {
    return { task: this.taskId };
  }
}

export class Context {
  private readonly nodes: Node[] = [];

  public call(taskId: string): Context {
    this.nodes.push(new Task(taskId));

    return this;
  }

  public build(): object {
    console.log(this.nodes);
    let graph = this.nodes.shift();

    if (!graph) return {};

    for (const node of this.nodes) {
      graph = new Linear(graph, node);
    }

    return graph.serialize();
  }
}

export function registerCall<TaskIds extends string>(
  _universe: Universe<TaskIds>,
) {
  return function call(taskId: TaskIds, ctx: Context): Context {
    console.log(`Calling ${taskId.toString()} with ${ctx}`);

    return ctx.call(`/tasks/${taskId}`);
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

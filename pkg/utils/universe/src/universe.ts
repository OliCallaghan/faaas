export class Universe<Ids> {
  constructor(readonly tasks: TaskDefinition<Ids>[]) {}
}

export type TaskDefinition<Id> = {
  readonly taskId: Id;
  readonly wasmPath: () => string;
};

export function defineTask<Id>(taskId: Id, path: string): TaskDefinition<Id> {
  return {
    taskId,
    wasmPath: () =>
      require.resolve(path, {
        paths: [process.cwd()],
      }),
  };
}

export function defineUniverse<Ids>(
  tasks: TaskDefinition<Ids>[],
): Universe<Ids> {
  return new Universe(tasks);
}

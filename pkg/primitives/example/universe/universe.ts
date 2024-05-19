import { defineTask, defineUniverse } from "@faaas/universe";

export const universe = defineUniverse([
  defineTask("task:1" as const, "@faaas-example/task1"),
  defineTask("task:2" as const, "@faaas-example/task1"),
  defineTask("task:3" as const, "@faaas-example/task1"),
  defineTask("task:4" as const, "@faaas-example/task1"),
  defineTask("task:5" as const, "@faaas-example/task1"),
]);

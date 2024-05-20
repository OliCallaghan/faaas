import { defineTask, defineUniverse } from "@faaas/faaasman";

export const universe = defineUniverse([
  defineTask("task:parse_auth" as const, "@faaas-example/task1"),
  defineTask("task:has_permission" as const, "@faaas-example/task1"),
  defineTask("task:get_pet" as const, "@faaas-example/task1"),
  defineTask("task:inc_pet" as const, "@faaas-example/task1"),
  defineTask("task:dec_pet" as const, "@faaas-example/task1"),
]);

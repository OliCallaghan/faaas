import { defineTask, defineUniverse } from "@faaas/faaasman";

export const universe = defineUniverse([
  defineTask("parse_auth" as const, "@faaas-example/task1"),
  defineTask("has_permission" as const, "@faaas-example/task1"),
  defineTask("get_pet" as const, "@faaas-example/task1"),
  defineTask("inc_pet" as const, "@faaas-example/task1"),
  defineTask("dec_pet" as const, "@faaas-example/task1"),
]);

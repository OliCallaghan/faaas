import { defineTask, defineUniverse } from "@faaas/faaasman";

export const universe = defineUniverse([
  defineTask("parse_auth" as const, "@faaas-example/parse_auth"),
  defineTask("has_permission" as const, "@faaas-example/has_permission"),
  defineTask("get_pet" as const, "@faaas-example/get_pet"),
  defineTask("inc_pet" as const, "@faaas-example/inc_pet"),
  defineTask("dec_pet" as const, "@faaas-example/dec_pet"),
]);

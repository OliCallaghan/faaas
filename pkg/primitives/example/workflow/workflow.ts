import { universe } from "@faaas-example/universe";
import {
  registerCall,
  // cond,
  // whenTrue,
  // whenFalse,
  type Context,
} from "@faaas/faaasman";

const call = registerCall(universe);

// My First Example Workflow
export const workflowId = "composition";

export function workflow(ctx: Context): Context {
  ctx = call("parse_auth", ctx);
  ctx = call("has_permission", ctx);
  ctx = call("get_pet", ctx);

  // Later when conditions are supported.
  // ctx = cond(
  //   call("has_permission", ctx),
  //   whenTrue((ctx) => call("get_pet", ctx)),
  //   whenFalse((ctx) => call("dec_pet", ctx)),
  // );

  ctx = call("inc_pet", ctx);
  ctx = call("dec_pet", ctx);

  return ctx;
}

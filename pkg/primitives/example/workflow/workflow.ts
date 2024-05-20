import { universe } from "@faaas-example/universe";
import {
  registerCall,
  cond,
  whenTrue,
  whenFalse,
  type Context,
} from "@faaas/faaasman";

const call = registerCall(universe);

export function workflow(ctx: Context): Context {
  ctx = call("task:parse_auth", ctx);
  ctx = call("task:get_pet", ctx);

  ctx = cond(
    call("task:has_permission", ctx),
    whenTrue((ctx) => call("task:get_pet", ctx)),
    whenFalse((ctx) => call("task:dec_pet", ctx)),
  );

  return ctx;
}

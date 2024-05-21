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
  ctx = call("parse_auth", ctx);
  ctx = call("get_pet", ctx);

  ctx = cond(
    call("has_permission", ctx),
    whenTrue((ctx) => call("get_pet", ctx)),
    whenFalse((ctx) => call("dec_pet", ctx)),
  );

  return ctx;
}

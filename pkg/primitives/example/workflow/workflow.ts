import { universe } from "@faaas-example/universe";
import {
  registerCall,
  cond,
  wait,
  whenTrue,
  whenFalse,
  type WorkflowContext,
  type TaskResult,
} from "@faaas/workflow";

const call = registerCall(universe);

export function workflow(ctx: WorkflowContext): TaskResult {
  const invokeAuth = call("task:auth", ctx);

  const [auth] = wait(invokeAuth);

  const [foo] = wait(
    cond(
      auth.ctx().get("authorised"),
      whenTrue(() => call("task:get_pet", ctx)),
      whenFalse(() => call("task:reject", auth.ctx().get("reason"))),
    ),
  );

  const invokeGet = call("task:get_pet", ctx);

  // const invokeOne = call("task:auth", ctx);
  // const invokeTwo = call("task:2", ctx);

  // const [one, two] = wait(invokeOne, invokeTwo);

  // const [buz] = wait(call("task:3", one.ctx));

  // const invokeQuz = cond(
  //   buz,
  //   whenTrue(() => call("task:4", two.ctx)),
  //   whenFalse(() => call("task:5", two.ctx)),
  // );

  // const [quz] = wait(invokeQuz);

  // return quz;
}

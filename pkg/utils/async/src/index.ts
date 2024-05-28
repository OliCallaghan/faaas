type Value = boolean | number | string;
type InvocationContext = Record<string, Value>;

type Handler = (ctx: InvocationContext) => InvocationContext;
type ContinuationHandler = (
  ctx: InvocationContext,
  cont: Continuation,
) => Continuation;

type Continuation = {
  fn: string;
  state: Record<string, Value>;
};

export function execute(handler: ContinuationHandler): Handler {
  return function handle(ctx: InvocationContext) {
    const prev = {};
    const { fn, state: next } = handler(ctx, prev);

    if (fn) {
    }

    return { fn, state: next };
  };
}

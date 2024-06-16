import { result, type Context } from "@faaas/handler";
import { echo } from "@faaas/echo";

// Original function
export async function handler(ctx: Context) {
  const [data = "{}"] = ctx.data;
  const { ms } = JSON.parse(data);

  ("use async");
  const location = await echo(ms);

  return result({ location });
}

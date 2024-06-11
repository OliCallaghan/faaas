import { Context, APIGatewayEvent } from "aws-lambda";
import { type Handler } from "@faaas/faaasc";

export function buildEntrypoint(handlers: Record<string, Handler>) {
  const handler = handlers["handler"];

  async function entrypoint(event: APIGatewayEvent, _ctx: Context) {
    try {
      const res = await handler(
        {
          id: event.requestContext.requestId,
          data: [event.body ?? ""],
        },
        {},
      );

      if (res.status == "done") return res.data;

      console.error("Function returned:", res);
      throw new Error("Function returned something other than a result");
    } catch (err) {
      console.error("Invalid event", err);

      throw new Error("Invalid event schema");
    }
  }

  return entrypoint;
}

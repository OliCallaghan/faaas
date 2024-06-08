type MqValue = string | number | boolean | number[];

interface MqTaskContext {
  id: string;
  task_id: string;
  args: MqValue[];
  data: Record<string, MqValue>;
  continuation: string | undefined;
  continuation_args: MqValue[];
}

const rabbit = new Connection("amqp://guest:guest@localhost:5672");

export async function entrypoint(event, ctx) {
  if ("rmqMessagesByQueue" in event) {
    const { rmqMessagesByQueue } = event;

    if ("task_invocations::/" in rmqMessagesByQueue) {
      const msgs = rmqMessagesByQueue["task_invocations::/"];

      for (const msg of msgs) {
        const mqTaskCtxJson = atob(msg.data);
        const mqTaskCtx = JSON.parse(mqTaskCtxJson);

        handler(mqTaskCtx);
      }
    }
  }
  return "Hello world";
}

function handler(mqTaskCtx: MqTaskContext) {
  console.log("Responding to invocation", mqTaskCtx.id);
}

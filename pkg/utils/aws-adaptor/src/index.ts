import { z } from "zod";
import { Connection } from "rabbitmq-client";

const MQInvocationEvent = z.object({
  rmqMessagesByQueue: z.object({
    "task_invocations::/": z.array(
      z.object({
        data: z.string(),
      }),
    ),
  }),
});
type MQInvocationEvent = z.infer<typeof MQInvocationEvent>;

const MQTaskContext = z.object({
  id: z.string(),
  task_id: z.string(),
  args: z.any(),
  data: z.record(z.string(), z.any()),
  continuation: z.nullable(z.string()),
  continuation_args: z.array(z.any()),
});
type MQTaskContext = z.infer<typeof MQTaskContext>;

const MQ_HOST =
  process.env.MQ_HOST ||
  "b-33245d09-61e4-4119-84ae-13dcdd945031.mq.eu-west-2.amazonaws.com";
const MQ_PORT = process.env.MQ_PORT || "5671";
const MQ_USER = process.env.MQ_USER || "admin";
const MQ_PASS = process.env.MQ_PASS || "ishouldmakethissecure";
const rabbit = new Connection(
  `amqps://${MQ_USER}:${MQ_PASS}@${MQ_HOST}:${MQ_PORT}`,
);

function onRabbitConnError(err: unknown) {
  console.log("RabbitMQ connection error", err);
}

function onRabbitConnOpen() {
  console.log("Connection successfully (re)established");
}

rabbit.on("error", onRabbitConnError);
rabbit.on("connection", onRabbitConnOpen);

const pub = rabbit.createPublisher({
  confirm: true,
  maxAttempts: 2,
});

export async function entrypoint(event: unknown, ctx) {
  try {
    const mqInvocEvent = MQInvocationEvent.parse(event);

    const { rmqMessagesByQueue } = mqInvocEvent;
    const { "task_invocations::/": msgs } = rmqMessagesByQueue;

    const mqTaskCtxs = msgs
      .map((msg) => msg.data)
      .map(atob)
      .map((str) => JSON.parse(str))
      .map((json) => MQTaskContext.parse(json))
      .map(handle);

    await Promise.all(mqTaskCtxs);

    return "processed";
  } catch (err) {
    console.error("Invalid event", err);

    throw new Error("Invalid event schema");
  }
}

async function handle(mqTaskCtx: MQTaskContext): Promise<void> {
  console.log("Responding to invocation", mqTaskCtx.id);

  const exchange = "amq.direct";
  const routingKey = `mq.gateway.invocations.${mqTaskCtx.id}`;

  const publishStart = performance.now();
  await pub.send({ exchange, routingKey }, "Successful response!");
  const publishEnd = performance.now();

  console.log("Publish response took", publishEnd - publishStart, "ms");
}

async function onShutdown() {
  await pub.close();
  await rabbit.close();
}

process.on("SIGINT", onShutdown);
process.on("SIGTERM", onShutdown);

import { z } from "zod";
import { Connection } from "rabbitmq-client";

import { continuation, result } from "@faaas/async";

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

function continuation(
  taskId: string,
  taskArgs: string[],
  taskScope: Record<string, any>,
) {
  return {
    status: "continuation",
    taskId,
    taskArgs,
    taskScope,
  } as const;
}

function result(data: string) {
  return {
    status: "done",
    data,
  } as const;
}

type Handler = (
  mqTaskCtx: MQTaskContext,
) => Promise<ReturnType<typeof continuation> | ReturnType<typeof result>>;

const handlers: Record<string, Handler> = {
  handler_0: handler_0,
  handler_1: handler_1,
};

async function handle(mqTaskCtx: MQTaskContext): Promise<void> {
  console.log("Responding to invocation", mqTaskCtx.id);

  const taskId = mqTaskCtx.task_id;
  const handler = handlers[taskId];

  try {
    if (handler) {
      const res = await handler(mqTaskCtx);

      if (res.status == "done") {
        await invokeResponse(mqTaskCtx, res.data);
      } else {
        await invokeContinuation(
          mqTaskCtx,
          res.taskId,
          res.taskArgs,
          res.taskScope,
        );
      }
    } else {
      throw new Error("Unknown handler");
    }
  } catch (err) {
    if (err instanceof Error) {
      await invokeError(mqTaskCtx, err);
    } else {
      await invokeError(mqTaskCtx, new Error("Unknown error"));
    }
  }
}

async function invokeResponse(mqTaskCtx: MQTaskContext, data: string) {
  const exchange = "amq.direct";
  const routingKey = `mq.gateway.invocations.${mqTaskCtx.id}`;

  // Log MQ Publish Performance
  const publishStart = performance.now();
  await pub.send({ exchange, routingKey }, data);
  const publishEnd = performance.now();

  console.log("Publish response took", publishEnd - publishStart, "ms");
}

async function invokeContinuation(
  mqTaskCtx: MQTaskContext,
  taskId: string,
  taskArgs: string[],
  taskScope: Record<string, any>,
) {
  const exchange = "amq.direct";
  const routingKey = `mq.invocations.${taskId}`;

  const mqContinuationTaskCtx: MQTaskContext = {
    id: mqTaskCtx.id,
    task_id: taskId,
    args: taskArgs,
    continuation: null,
    continuation_args: [],
    data: taskScope,
  };

  const publishStart = performance.now();
  await pub.send({ exchange, routingKey }, mqContinuationTaskCtx);
  const publishEnd = performance.now();

  console.log("Publish continuation took", publishEnd - publishStart, "ms");
}

async function invokeError(mqTaskCtx: MQTaskContext, err: Error) {
  const exchange = "amq.direct";
  const routingKey = `mq.gateway.invocations.${mqTaskCtx.id}`;

  // Log MQ Publish Performance
  const publishStart = performance.now();
  await pub.send({ exchange, routingKey }, err.toString());
  const publishEnd = performance.now();

  console.log("Publish err response took", publishEnd - publishStart, "ms");
}

async function onShutdown() {
  await pub.close();
  await rabbit.close();
}

process.on("SIGINT", onShutdown);
process.on("SIGTERM", onShutdown);

/**
    Insert user code from here
*/
import postgres from "postgres";

const sql = postgres({
  host: "postgres-db.cno4eviwxzxv.eu-west-2.rds.amazonaws.com",
  port: 5432,
  username: "faaasuser",
  password: "securepassword",
  database: "postgres",
  ssl: { rejectUnauthorized: false },
});

/**
    Handlers start from here
*/

// handler_0: initial handler
async function handler_0(mqTaskCtx: MQTaskContext) {
  // Log SQL Query Performance
  const queryStart = performance.now();
  const data = await sql`SELECT * FROM pets`;
  const dataStr = data.toString();
  const queryEnd = performance.now();

  console.log("Query took", queryEnd - queryStart, "ms");
  console.log("Query returned", dataStr);

  return continuation(
    "proxy.sql.pg",
    ["handler", "handler_1", "SELECT * FROM pets"],
    {},
  );
}

async function handler_1(mqTaskCtx: MQTaskContext) {
  return result("Hello, World!");
}

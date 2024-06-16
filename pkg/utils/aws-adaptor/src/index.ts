import { Context } from "aws-lambda";
import { z } from "zod";
import { Connection } from "rabbitmq-client";

import { type Task, type Handler } from "@faaas/faaasc";
import { computeProxyProfitability, publishDuration } from "./billing";

const MQTaskQueueName = process.env.MQ_QUEUE ?? "task_invocations";
const MQTaskQueueBinding = `${MQTaskQueueName}::/`;

const MQInvocationEvent = z.object({
  rmqMessagesByQueue: z.object({
    [MQTaskQueueBinding]: z.array(
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
  continuation_id: z.string(),
  args: z.array(z.string()),
  state: z.record(z.string(), z.any()),
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

export function buildEntrypoint(handlers: Record<string, Handler>) {
  async function entrypoint(event: unknown, ctx: Context) {
    try {
      const mqInvocEvent = MQInvocationEvent.parse(event);

      const { rmqMessagesByQueue } = mqInvocEvent;
      const { [MQTaskQueueBinding]: msgs } = rmqMessagesByQueue;

      const mqTaskCtxs = msgs
        .map((msg) => msg.data)
        .map(atob)
        .map((str) => JSON.parse(str))
        .map((json) => MQTaskContext.parse(json))
        .map((taskCtx) => handle(taskCtx, ctx));

      await Promise.all(mqTaskCtxs);

      return "processed";
    } catch (err) {
      console.error("Invalid event", err);

      throw new Error("Invalid event schema");
    }
  }

  async function handle(mqTaskCtx: MQTaskContext, ctx: Context): Promise<void> {
    console.log("Responding to invocation", mqTaskCtx.id);

    // The continuation id to execute from the task id
    const continuationId = mqTaskCtx.continuation_id;
    const handler = handlers[continuationId];

    try {
      if (handler) {
        const res = await handler(
          { id: mqTaskCtx.id, data: mqTaskCtx.args ?? [""] },
          mqTaskCtx.state,
        );

        if (res.status == "done") {
          await invokeResponse(mqTaskCtx, res.data);
        } else {
          if (await shouldProxy(res.task, res.taskArgs, ctx)) {
            await invokeProxyContinuation(
              mqTaskCtx,
              res.task.proxy,
              [mqTaskCtx.task_id, ...res.taskArgs],
              res.taskScope,
            );
          } else {
            await invokeLocalContinuation(
              mqTaskCtx,
              res.task,
              [mqTaskCtx.task_id, ...res.taskArgs],
              res.taskScope,
              ctx,
            );
          }
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

  async function invokeLocalContinuation(
    mqTaskCtx: MQTaskContext,
    task: Task,
    taskArgs: string[],
    taskScope: Record<string, any>,
    ctx: Context,
  ) {
    const taskLocalStart = performance.now();

    const [nextTaskId, nextContinuationId, ...taskCallArgs] = taskArgs;
    const res = await task(...taskCallArgs);
    const resSerialised = JSON.stringify(res);

    const taskLocalEnd = performance.now();
    const taskDuration = taskLocalEnd - taskLocalStart;

    // Log the local continuation execution time to CloudWatch for analysis
    console.log(
      "Local continuation of:",
      task.proxy,
      "took:",
      taskDuration,
      "ms",
    );

    const mqNextTaskCtx = {
      id: mqTaskCtx.id,
      task_id: nextTaskId,
      continuation_id: nextContinuationId,
      state: taskScope,
      args: [resSerialised],
    } as MQTaskContext;

    await Promise.all([
      handle(mqNextTaskCtx, ctx),
      publishDuration(task.proxy, taskLocalEnd - taskLocalStart),
    ]);
  }

  return entrypoint;
}

const { FAAAS_STRATEGY = "adaptive" } = process.env;

// To implement with cost estimation
async function shouldProxy(
  task: Task,
  _taskArgs: string[],
  ctx: Context,
): Promise<boolean> {
  if (FAAAS_STRATEGY == "proxy") return true;
  if (FAAAS_STRATEGY == "local") return false;
  if (FAAAS_STRATEGY != "adaptive")
    console.error(
      "Unknown continuation execution strategy, defaulting to adaptive",
    );

  const start = performance.now();
  const { prob, saving } = await computeProxyProfitability(task.proxy, ctx);
  const elapsed = performance.now() - start;

  console.log("Proxy profitability likelihood:", prob);
  console.log("Proxy saving estimated to save:", saving, "USD for", task.proxy);
  console.log("Proxy saving computation took:", elapsed, "ms for:", task.proxy);

  if (Math.random() < 0.1) {
    return false;
  }

  return prob > 0.75;
}

async function invokeResponse(mqTaskCtx: MQTaskContext, data: string) {
  const exchange = "amq.direct";
  const routingKey = `mq.gateway.invocations.${mqTaskCtx.id}`;

  // Log MQ Publish Performance
  const publishStart = performance.now();
  await pub.send({ exchange, routingKey }, data);
  const publishEnd = performance.now();

  console.log("Publish to MQ time:", publishEnd - publishStart, "ms");
}

async function invokeProxyContinuation(
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
    continuation_id: taskId,
    args: taskArgs,
    state: taskScope,
  };

  const publishStart = performance.now();
  await pub.send({ exchange, routingKey }, mqContinuationTaskCtx);
  const publishEnd = performance.now();

  console.log("Publish to MQ time:", publishEnd - publishStart, "ms");
}

async function invokeError(mqTaskCtx: MQTaskContext, err: Error) {
  const exchange = "amq.direct";
  const routingKey = `mq.gateway.invocations.${mqTaskCtx.id}`;

  // Log MQ Publish Performance
  const publishStart = performance.now();
  await pub.send({ exchange, routingKey }, err.toString());
  const publishEnd = performance.now();

  console.log("Publish to MQ time:", publishEnd - publishStart, "ms");
}

async function onShutdown() {
  await pub.close();
  await rabbit.close();
}

process.on("SIGINT", onShutdown);
process.on("SIGTERM", onShutdown);

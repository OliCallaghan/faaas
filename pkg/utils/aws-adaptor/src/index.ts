import { z } from "zod";
import { Connection } from "rabbitmq-client";

import { type Handler } from "@faaas/faaasc";

// type Task = {
//   proxy: string;
// };

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
  state: z.record(z.string(), z.any()),
  continuation: z.nullable(z.string()), // deprecated
  continuation_args: z.array(z.any()), // deprecated
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
  async function entrypoint(event: unknown, _: unknown) {
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

    const taskId = mqTaskCtx.task_id;
    const handler = handlers[taskId];

    try {
      if (handler) {
        const res = await handler(
          { id: mqTaskCtx.id, data: mqTaskCtx.args ?? "" },
          mqTaskCtx.state,
        );

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

  return entrypoint;
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
    state: taskScope,
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

// /**
//     Insert user code from here
// */
// import postgres from "postgres";

// const sql = postgres({
//   host: "postgres-db.cno4eviwxzxv.eu-west-2.rds.amazonaws.com",
//   port: 5432,
//   username: "faaasuser",
//   password: "securepassword",
//   database: "postgres",
//   ssl: { rejectUnauthorized: false },
// });

// // Is vulnerable to SQL injection
// function execSql<T>(query: string): Promise<T[]> {
//   return sql.unsafe(query);
// }

// execSql.proxy = "proxy.sql.pg";

// // Original function
// async function handler_no_async(ctx: Context, _: State) {
//   const { name } = JSON.parse(ctx.data ?? "{}");
//   const foo = 11;

//   ("use async");
//   const rows = await execSql<{ name: string; age: number }>(
//     `SELECT * FROM pets WHERE name = '${name}'`,
//   );

//   let sumAge = 0;
//   for (const row of rows) {
//     if (row.age) {
//       sumAge += row.age;
//     }
//   }

//   const age = sumAge + foo;
//   console.log("age", age);

//   return result(JSON.stringify({ age }));
// }

// /**
//     Handlers start from here
// */

// // handler_0: initial handler
// async function handler_0(ctx: Context, _: State) {
//   const { name } = JSON.parse(ctx.data ?? "{}");
//   const foo = 11;

//   return continuation(
//     execSql,
//     ["handler", "handler_1", `SELECT * FROM pets WHERE name = '${name}'`],
//     { foo },
//   );
// }

// async function handler_1(ctx: Context, state: State) {
//   console.log("state", state);
//   console.log("state.foo", state.foo);

//   const { foo } = state;

//   console.log("foo", foo);

//   console.log("typeof ctx", typeof ctx);
//   console.log("ctx", ctx);

//   console.log("typeof ctx.data", typeof ctx.data);
//   console.log("ctx.data", ctx.data);

//   const rows = JSON.parse(ctx.data);
//   console.log("data", rows);

//   let sumAge = 0;
//   for (const row of rows) {
//     if (row.age) {
//       sumAge += Number(row.age);
//     }
//   }

//   const age = sumAge + foo;
//   console.log("age", age);

//   return result(JSON.stringify({ age }));
// }

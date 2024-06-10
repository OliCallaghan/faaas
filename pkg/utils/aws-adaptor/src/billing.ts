import { Context } from "aws-lambda";
import { createClient } from "redis";

const AWS_ARM_COST_PER_GB_SEC = 0.0000133334;
const AWS_X86_COST_PER_GB_SEC = 0.0000166667;

const AWS_INVOCATION_COST = 0.0000002;

function isArm() {
  return process.arch === "arm64";
}

/**
  Compute the cost to keep an ARM function running for the given amount of time.
*/
function computeArmExecutionCost(memAllocMB: number, ms: number): number {
  return AWS_ARM_COST_PER_GB_SEC * (memAllocMB / 1024) * (ms / 1000);
}

/**
  Compute the cost to keep an x86 function running for the given amount of time.
*/
function computeX86ExecutionCost(memAllocMB: number, ms: number): number {
  return AWS_X86_COST_PER_GB_SEC * (memAllocMB / 1024) * (ms / 1000);
}

/**
    Compute the cost to execute a function.
*/
function computeExecutionCost(memAllocMB: number, ms: number): number {
  return isArm()
    ? computeArmExecutionCost(memAllocMB, ms)
    : computeX86ExecutionCost(memAllocMB, ms);
}

/**
  Compute the cost to invoke an ARM function.
*/
function computeArmInvocationCost(warmupTime: number): number {
  return AWS_INVOCATION_COST + computeArmExecutionCost(128, warmupTime);
}

/**
  Compute the cost to invoke an x86 function.
*/
function computeX86InvocationCost(warmupTime: number): number {
  return AWS_INVOCATION_COST + computeX86ExecutionCost(128, warmupTime);
}

/**
  Compute the cost to invoke a function.
*/
function computeInvocationCost(warmupTime: number): number {
  return isArm()
    ? computeArmInvocationCost(warmupTime)
    : computeX86InvocationCost(warmupTime);
}

const clientConn = createClient({
  url: process.env.REDIS_URL,
  disableClientInfo: true,
}).connect();

export async function publishDuration(
  task: string,
  durationMs: number,
): Promise<void> {
  const ms = Math.floor(Math.min(durationMs, 9999));
  const msStr = ms.toString().padStart(4, "0");

  const client = await clientConn;
  await client.append(`tasks:${task}:durations`, msStr);
}

async function fetchDurations(task: string): Promise<number[]> {
  try {
    const client = await clientConn;
    const durationsKey = `tasks:${task}:durations`;

    const sampleCnt = 4;
    const sampleLst = await client.strLen(durationsKey);
    const sampleFst = Math.max(sampleLst - sampleCnt * 4, 0);

    const durationsRaw = await client.getRange(
      durationsKey,
      sampleFst,
      sampleLst,
    );
    const durations = durationsRaw.match(/.{4}/g)?.map((x) => parseInt(x)) ?? [
      0,
    ];

    return durations;
  } catch (err) {
    console.error("Error accessing redis", err);

    return [0];
  }
}

/**
    Compute the saving of proxying a function.
*/
export async function computeProxySaving(
  task: string,
  ctx: Context,
): Promise<number> {
  const memAllocMB = parseInt(ctx.memoryLimitInMB);
  const taskEstimatedTimesMs = await fetchDurations(task);

  const taskAvgTimeMs = avg(taskEstimatedTimesMs);
  const taskStdTimeMs = std(taskEstimatedTimesMs);
  const taskSampledTimeMs = randomGaussian(taskAvgTimeMs, taskStdTimeMs);

  console.log("Task times", taskEstimatedTimesMs);

  const warmupTime = randomGaussian(20, 5); // TODO: Tune this number
  console.log(
    "Sampled task time",
    taskSampledTimeMs,
    `from N(${taskAvgTimeMs}, ${taskStdTimeMs})`,
  );
  console.log("Sampled warmup time", warmupTime);

  // Compute the cost savings of proxying the function
  const invocationCost = computeInvocationCost(warmupTime);
  const executionCost = computeExecutionCost(memAllocMB, taskSampledTimeMs);

  return executionCost - invocationCost;
}

function avg(vals: number[]): number {
  if (vals.length == 0) throw new Error("Cannot compute average of empty list");

  return vals.reduce((a, b) => a + b, 0) / vals.length;
}

function std(vals: number[]): number {
  if (vals.length == 0) throw new Error("Cannot compute std of empty list");

  const squareOfMeans = avg(vals) ** 2;
  const meanofSquares = avg(vals.map((val) => val * val));

  return Math.sqrt(meanofSquares - squareOfMeans);
}

function randomGaussian(mean: number, std: number): number {
  const u = 1 - Math.random();
  const v = Math.random();
  const z = Math.sqrt(-2 * Math.log(u)) * Math.cos(2 * Math.PI * v);

  return z * std + mean;
}

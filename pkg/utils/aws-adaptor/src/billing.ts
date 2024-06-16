import { Context } from "aws-lambda";
import { createClient } from "redis";

import { weibull3, weibull3cdf, weibull3median } from "@faaas/resp-estimator";

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

interface WeibullParameters {
  loc: number;
  scale: number;
  shape: number;
}

async function fetchParameters(
  fn: string,
  task: string,
): Promise<WeibullParameters> {
  try {
    const client = await clientConn;

    const paramsStr = await client.get(`tasks:${fn}:${task}:params`);
    const params = JSON.parse(paramsStr ?? "{}");

    return {
      loc: parseFloat(params.loc),
      scale: parseFloat(params.scale),
      shape: parseFloat(params.shape),
    };
  } catch (err) {
    console.error("Error accessing redis", err);

    return {
      loc: 0,
      scale: 0,
      shape: 0,
    };
  }
}

function computeProfitThreshold(
  memAllocMB: number,
  warmupTime: number,
): number {
  const baseCt = isArm() ? AWS_ARM_COST_PER_GB_SEC : AWS_X86_COST_PER_GB_SEC;
  const Ct = baseCt * (memAllocMB / 1024);

  return (Ct * warmupTime + AWS_INVOCATION_COST) / Ct;
}

/**
    Compute the saving of proxying a function.
*/
export async function computeProxyProfitability(
  task: string,
  ctx: Context,
): Promise<{ prob: number; saving: number }> {
  const memAllocMB = parseInt(ctx.memoryLimitInMB);
  const warmupTime = randomGaussian(20, 5) / 1000; // TODO: Tune this number

  const profitThreshold = computeProfitThreshold(memAllocMB, warmupTime);
  console.log("Profitable to split if t >", profitThreshold * 1e3);

  const { loc, scale, shape } = await fetchParameters(ctx.functionName, task);
  console.log("Proxy(", task, ") ~ W(", shape, loc, scale, ")");

  const dist = weibull3(loc, scale, shape);
  const mean = weibull3median(dist); // approximation that mean ~ median for continuous functions
  const prob = 1 - weibull3cdf(dist, profitThreshold * 1e3);

  const saving =
    computeExecutionCost(memAllocMB, mean) - computeInvocationCost(warmupTime);

  return {
    prob,
    saving,
  };
}

function randomGaussian(mean: number, std: number): number {
  const u = 1 - Math.random();
  const v = Math.random();
  const z = Math.sqrt(-2 * Math.log(u)) * Math.cos(2 * Math.PI * v);

  return z * std + mean;
}

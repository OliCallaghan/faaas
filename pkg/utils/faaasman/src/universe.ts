import { resolve } from "node:path";
import { type Universe } from "@faaas/universe";
import { getTaskHash } from "./task";

export async function deployUniverse(
  universeRelativePath: string,
): Promise<void> {
  const universeResolvedPath = resolve(universeRelativePath);
  const mod = require(universeResolvedPath);
  const universe: Universe<unknown> = mod.universe;

  for (const { taskId, wasmPath } of universe.tasks) {
    if (typeof taskId !== "string") throw new Error("Task ID must be a string");

    const wasmHash = await getTaskHash(wasmPath());

    console.log(`Deploying ${wasmPath} as ${taskId} with hash ${wasmHash}`);
  }
}

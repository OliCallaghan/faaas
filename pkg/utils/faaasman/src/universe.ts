import { resolve } from "node:path";

import { PutObjectCommand, S3Client } from "@aws-sdk/client-s3";
import { type Universe } from "@faaas/universe";
import { getTaskHash } from "./task";
import { createReadStream } from "node:fs";

export async function deployUniverse(
  universeRelativePath: string,
): Promise<void> {
  const universeResolvedPath = resolve(universeRelativePath);
  const mod = require(universeResolvedPath);
  const universe: Universe<unknown> = mod.universe;

  const storage = new S3Client({
    endpoint: "http://localhost:9000",
    region: "us-west-2",
    forcePathStyle: true,
    credentials: {
      accessKeyId: "faaas",
      secretAccessKey: "wasm-rocks",
    },
  });

  for (const { taskId, wasmPath } of universe.tasks) {
    if (typeof taskId !== "string") throw new Error("Task ID must be a string");

    const wasmHash = await getTaskHash(wasmPath());

    await storage.send(
      new PutObjectCommand({
        Bucket: "universe",
        Key: `tasks/${taskId}`,
        Body: createReadStream(wasmPath()),
      }),
    );

    console.log(`Deploying ${wasmPath} as ${taskId} with hash ${wasmHash}`);
  }
}

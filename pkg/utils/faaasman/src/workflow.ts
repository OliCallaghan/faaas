import { resolve } from "node:path";
import { Context, type Workflow } from "@faaas/workflow";
import { PutObjectCommand, S3Client } from "@aws-sdk/client-s3";

export async function deployWorkflow(
  workflowRelativePath: string,
): Promise<void> {
  const workflowResolvedPath = resolve(workflowRelativePath);
  const mod = require(workflowResolvedPath);
  const workflow: Workflow = mod.workflow;
  const workflowId = mod.workflowId;

  const storage = new S3Client({
    endpoint: "http://localhost:9000",
    region: "us-west-2",
    forcePathStyle: true,
    credentials: {
      accessKeyId: "faaas",
      secretAccessKey: "wasm-rocks",
    },
  });

  // Pass the workflow a Proxy context object, and
  // record when functions are called, and with what.
  const ctx = new Context();

  workflow(ctx);

  let graph = ctx.build();

  console.log("Composition graph looks like this", ctx.build());

  await storage.send(
    new PutObjectCommand({
      Bucket: "universe",
      Key: `workflows/${workflowId}`,
      Body: JSON.stringify(graph),
    }),
  );
}

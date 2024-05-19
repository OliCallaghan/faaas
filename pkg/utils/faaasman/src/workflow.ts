import { resolve } from "node:path";
import { type Workflow } from "@faaas/workflow";

export async function deployWorkflow(
  workflowRelativePath: string,
): Promise<void> {
  const workflowResolvedPath = resolve(workflowRelativePath);
  const mod = require(workflowResolvedPath);
  const workflow: Workflow = mod.workflow;

  // Pass the workflow a Proxy context object, and
  // record when functions are called, and with what.
  workflow({});
}

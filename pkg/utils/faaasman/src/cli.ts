import { program } from "commander";
import { deployUniverse } from "./universe";
import { deployWorkflow } from "./workflow";

import { register } from "ts-node";

register({
  transpileOnly: true,
});

program
  .name("faaasman")
  .description("CLI to deploy WASM tasks and workflows to FaaAS")
  .version("1.0.0");

program
  .command("universe")
  .command("deploy")
  .argument("<universe.ts>", "universe to deploy")
  .action(deployUniverse);

program
  .command("workflow")
  .command("deploy")
  .argument("<workflow.ts>", "workflow to deploy")
  .action(deployWorkflow);

program.parse();

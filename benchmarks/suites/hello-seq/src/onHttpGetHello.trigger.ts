import { TaskContext, result } from "@faaas/handler";
import { database, exec } from "@faaas/sql";

const PG_USER = "postgres";
const PG_PASS = "password";
const PG_HOST = "localhost";
const PG_PORT = 5432;

const sql = database(PG_USER, PG_PASS, PG_HOST, PG_PORT, "postgres");

export async function handler(_: TaskContext) {
  ("use async");
  const names = await exec(sql`SELECT name FROM pets`);

  ("use async");
  const ages = await exec(sql`SELECT ages FROM pets`);

  return result({
    status: "success",
    names,
    ages,
  });
}

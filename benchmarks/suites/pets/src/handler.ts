import { result, type Context } from "@faaas/handler";
import { database } from "@faaas/sql";

const sql = database({
  host: "postgres-db.cno4eviwxzxv.eu-west-2.rds.amazonaws.com",
  port: 5432,
  username: "faaasuser",
  password: "securepassword",
  database: "postgres",
  ssl: { rejectUnauthorized: false },
});

// Original function
export async function handler(ctx: Context) {
  const [data = "{}"] = ctx.data;
  const { name } = JSON.parse(data);
  const foo = 11;

  ("use async");
  const rows = await sql(`SELECT * FROM pets WHERE name = '${name}'`);

  let sumAge = 0;
  for (const row of rows) {
    if (row.age) {
      sumAge += row.age;
    }
  }

  const age = sumAge + foo;
  console.log("age", age);

  return result({ age });
}

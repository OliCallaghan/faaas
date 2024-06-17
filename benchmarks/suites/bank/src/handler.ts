import { type Context, result } from "@faaas/handler";
import { database } from "@faaas/sql";

const sql = database({
  host: "postgres-db.cno4eviwxzxv.eu-west-2.rds.amazonaws.com",
  port: 5432,
  username: "faaasuser",
  password: "securepassword",
  database: "petstore",
  ssl: { rejectUnauthorized: false },
});

export async function handler(ctx: Context) {
  const { src, dst, amount } = JSON.parse(ctx.data[0]);

  ("use async");
  const srcAcc = await sql(`SELECT balance FROM accounts WHERE id = '${src}'`);

  const [account] = srcAcc;

  if (account.balance < amount) {
    return result({
      status: "balance too low",
    });
  }

  ("use async");
  const _1 = await sql(
    `UPDATE accounts SET balance = balance - ${amount} WHERE id = ${src}`,
  );

  ("use async");
  const _2 = await sql(`
    UPDATE accounts SET balance = balance + ${amount} WHERE id = '${dst}'
  `);

  return result({
    status: "success",
  });
}

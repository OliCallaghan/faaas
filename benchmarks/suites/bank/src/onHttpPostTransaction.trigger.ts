import { type Context, result } from "@faaas/handler";
import { database } from "@faaas/sql";

const PG_USER = "postgres";
const PG_PASS = "password";
const PG_HOST = "localhost";
const PG_PORT = 5432;

const sql = database({
  host: PG_USER,
  port: PG_PORT,
  user: PG_HOST,
  pass: PG_PASS,
  database: "postgres",
  ssl: { rejectUnauthorized: false },
});

export async function handler(ctx: Context) {
  const { src, dst, amount } = JSON.parse(ctx.data[0]);

  ("use async");
  const srcAcc = await sql(`SELECT balance FROM accounts WHERE id == '${src}'`);

  const [account] = srcAcc;

  if (account.balance < amount) {
    return result({
      status: "balance too low",
    });
  }

  ("use async");
  const _ = await sql(`
    BEGIN;

    -- Decrement the balance of the sender's account
    UPDATE accounts
    SET balance = balance - ${amount}
    WHERE id = ${src};

    -- Increment the balance of the receiver's account
    UPDATE accounts
    SET balance = balance + ${amount}
    WHERE id = '${dst}';

    COMMIT;
  `);

  return result({
    status: "success",
  });
}

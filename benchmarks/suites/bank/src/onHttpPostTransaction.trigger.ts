import { TaskContext, result } from "@faaas/handler";
import { database, exec } from "@faaas/sql";

const PG_USER = "postgres";
const PG_PASS = "password";
const PG_HOST = "localhost";
const PG_PORT = 5432;

const sql = database(PG_USER, PG_PASS, PG_HOST, PG_PORT, "postgres");

export async function handler(ctx: TaskContext) {
  const { src, dst, amount } = ctx;

  ("use async");
  const srcAcc = await exec(
    sql`SELECT balance FROM accounts WHERE id == ${src}`,
  );

  const [account] = srcAcc;

  if (account.balance < amount) {
    return result({
      status: "balance too low",
    });
  }

  ("use async");
  const _ = await exec(sql`
    BEGIN;

    -- Decrement the balance of the sender's account
    UPDATE accounts
    SET balance = balance - ${amount}
    WHERE id = ${src};

    -- Increment the balance of the receiver's account
    UPDATE accounts
    SET balance = balance + ${amount}
    WHERE id = ${dst};

    COMMIT;
  `);

  return result({
    status: "success",
  });
}

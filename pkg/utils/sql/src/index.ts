import postgres, { MaybeRow, PendingQuery } from "postgres";

export function database(
  user: string,
  pass: string,
  host: string,
  port: number,
  database: string,
) {
  return postgres({
    host,
    port,
    user,
    pass,
    database,
  });
}

export function exec<TRow extends readonly MaybeRow[]>(
  query: PendingQuery<TRow>,
) {
  return query;
}

exec.continuation = "io/sql/pg";

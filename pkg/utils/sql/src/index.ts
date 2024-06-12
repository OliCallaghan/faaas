import postgres, { Options } from "postgres";

export function database(options: Options<{}>) {
  const sql = postgres(options);

  function exec(query: string) {
    return sql.unsafe(query);
  }

  const db = options.database ?? "default";
  exec.proxy = `proxy.sql.pg.${db}`;

  return exec;
}

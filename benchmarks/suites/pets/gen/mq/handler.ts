import { buildEntrypoint } from "@faaas/aws-adaptor";
import { continuation, type FaaascInternalState, type FaaascInternalContext } from "@faaas/faaasc";
import { result, type Context } from "@faaas/handler";
import { database } from "@faaas/sql";
const sql = database({
    host: "postgres-db.cno4eviwxzxv.eu-west-2.rds.amazonaws.com",
    port: 5432,
    username: "faaasuser",
    password: "securepassword",
    database: "petstore",
    ssl: {
        rejectUnauthorized: false
    }
});
export async function handler(ctx: Context) {
    const [data = "{}"] = ctx.data;
    const { name } = JSON.parse(data);
    const foo = 11;
    ("use async");
    const rows = await sql(`SELECT * FROM pets WHERE name = '${name}'`);
    let sumAge = 0;
    for (const row of rows){
        if (row.age) {
            sumAge += row.age;
        }
    }
    const age = sumAge + foo;
    console.log("age", age);
    return result({
        age
    });
}
export async function handler_0(ctx: FaaascInternalContext, state: FaaascInternalState) {
    const {} = state;
    const [data = "{}"] = ctx.data;
    const { name } = JSON.parse(data);
    const foo = 11;
    const rows = [
        `SELECT * FROM pets WHERE name = '${name}'`
    ];
    return continuation(sql, [
        "handler_1",
        ...rows
    ], {
        name,
        foo
    });
}
export async function handler_1(ctx: FaaascInternalContext, state: FaaascInternalState) {
    const { name, foo } = state;
    const rows = JSON.parse(ctx.data[0]);
    let sumAge = 0;
    for (const row of rows){
        if (row.age) {
            sumAge += row.age;
        }
    }
    const age = sumAge + foo;
    console.log("age", age);
    return result({
        age
    });
}
export const entrypoint = buildEntrypoint({
    handler_0,
    handler_1,
    handler
});

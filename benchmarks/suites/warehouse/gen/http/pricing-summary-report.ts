import { buildEntrypoint } from "@faaas/aws-http-adaptor";
import { continuation, type FaaascInternalState, type FaaascInternalContext } from "@faaas/faaasc";
import { result, type Context } from "@faaas/handler";
import { database } from "@faaas/sql";
import { compile } from "handlebars";
const TEMPLATE = `
<table>
  <tr>
    <th>l_returnflag</th>
    <th>l_linestatus</th>
    <th>sum_qty</th>
    <th>sum_base_price</th>
    <th>sum_disc_price</th>
    <th>sum_charge</th>
    <th>avg_qty</th>
    <th>avg_price</th>
    <th>avg_disc</th>
    <th>count_order</th>
  </tr>
  <tr>
    {{#each rows}}
      <td>{{l_returnflag}}</td>
      <td>{{l_linestatus}}</td>
      <td>{{sum_qty}}</td>
      <td>{{sum_base_price}}</td>
      <td>{{sum_disc_price}}</td>
      <td>{{sum_charge}}</td>
      <td>{{avg_qty}}</td>
      <td>{{avg_price}}</td>
      <td>{{avg_disc}}</td>
      <td>{{count_order}}</td>
    {{/each}}
  </tr>
</table>`;
const template = compile(TEMPLATE);
const sql = database({
    host: "postgres-db.cno4eviwxzxv.eu-west-2.rds.amazonaws.com",
    port: 5432,
    username: "faaasuser",
    password: "securepassword",
    database: "tpch",
    ssl: {
        rejectUnauthorized: false
    }
});
export async function handler(ctx: Context) {
    const [data = "{}"] = ctx.data;
    const { days: daysRaw } = JSON.parse(data);
    const days = parseInt(daysRaw);
    if (days < 60 || days > 120) return result({
        message: "Days must be between 60 and 120"
    });
    ("use async");
    const rows = await sql(`
    SELECT
        l_returnflag,
        l_linestatus,
        SUM(l_quantity) AS sum_qty,
        SUM(l_extendedprice) AS sum_base_price,
        SUM(l_extendedprice * (1 - l_discount)) AS sum_disc_price,
        SUM(l_extendedprice * (1 - l_discount) * (1 + l_tax)) AS sum_charge,
        AVG(l_quantity) AS avg_qty,
        AVG(l_extendedprice) AS avg_price,
        AVG(l_discount) AS avg_disc,
        COUNT(*) AS count_order
    FROM
        lineitem
    WHERE
        l_shipdate <= DATE '1998-12-01' - INTERVAL '${days} day'
    GROUP BY
        l_returnflag,
        l_linestatus
    ORDER BY
        l_returnflag,
        l_linestatus
    LIMIT 10;
  `);
    const html = template({
        rows
    });
    return result(html, {
        raw: true
    });
}
export async function handler_0(ctx: FaaascInternalContext, state: FaaascInternalState) {
    const {} = state;
    const [data = "{}"] = ctx.data;
    const { days: daysRaw } = JSON.parse(data);
    const days = parseInt(daysRaw);
    if (days < 60 || days > 120) return result({
        message: "Days must be between 60 and 120"
    });
    const rows = [
        `
    SELECT
        l_returnflag,
        l_linestatus,
        SUM(l_quantity) AS sum_qty,
        SUM(l_extendedprice) AS sum_base_price,
        SUM(l_extendedprice * (1 - l_discount)) AS sum_disc_price,
        SUM(l_extendedprice * (1 - l_discount) * (1 + l_tax)) AS sum_charge,
        AVG(l_quantity) AS avg_qty,
        AVG(l_extendedprice) AS avg_price,
        AVG(l_discount) AS avg_disc,
        COUNT(*) AS count_order
    FROM
        lineitem
    WHERE
        l_shipdate <= DATE '1998-12-01' - INTERVAL '${days} day'
    GROUP BY
        l_returnflag,
        l_linestatus
    ORDER BY
        l_returnflag,
        l_linestatus
    LIMIT 10;
  `
    ];
    return continuation(sql, [
        "handler_1",
        ...rows
    ], {
        days
    });
}
export async function handler_1(ctx: FaaascInternalContext, state: FaaascInternalState) {
    const { days } = state;
    const rows = JSON.parse(ctx.data[0]);
    const html = template({
        rows
    });
    return result(html, {
        raw: true
    });
}
export const entrypoint = buildEntrypoint({
    handler_0,
    handler_1,
    handler
});

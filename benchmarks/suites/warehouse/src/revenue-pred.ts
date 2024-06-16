import { result, type Context } from "@faaas/handler";
import { database } from "@faaas/sql";

import * as tfc from "@tensorflow/tfjs-core";
require("@tensorflow/tfjs-backend-cpu");
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
  ssl: { rejectUnauthorized: false },
});

// Generate a pricing summary report
export async function handler(ctx: Context) {
  const [data = "{}"] = ctx.data;
  const { region: regionRaw, year: yearRaw } = JSON.parse(data);

  const regions = ["AFRICA", "AMERICA", "ASIA", "EUROPE", "MIDDLE EAST"];

  const region = String(regionRaw);
  const year = parseInt(yearRaw);

  if (year < 1993 || year > 1995)
    return result({
      message: "Days must be between 60 and 120",
    });

  if (!regions.includes(region))
    return result({
      message: `Region must be one of ${regions}`,
    });

  ("use async");
  const rows = await sql(`
    SELECT
        n_name,
        SUM(l_extendedprice * (1 - l_discount)) AS revenue
    FROM
        customer,
        orders,
        lineitem,
        supplier,
        nation,
        region
    WHERE
        c_custkey = o_custkey AND
        l_orderkey = o_orderkey AND
        l_suppkey = s_suppkey AND
        c_nationkey = s_nationkey AND
        s_nationkey = n_nationkey AND
        n_regionkey = r_regionkey AND
        r_name = '${region}' AND
        o_orderdate >= DATE '${year}-01-01' AND
        o_orderdate < DATE '${year}-01-01' + INTERVAL '1 year'
    GROUP BY
        n_name
    ORDER BY
        revenue DESC;
  `);

  const country = tfc.tensor(rows.map((row) => row.n_name));
  const revenue = tfc.tensor(rows.map((row) => row.revenue));

  const weights = tfc.rand([rows.length, rows.length], Math.random, "float32");

  let output = revenue;

  for (let i = 0; i < 1000; i++) {
    output = tfc.dot(output, weights);
  }

  return result("Inference complete", { raw: true });
}

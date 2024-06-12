import { result, type Context } from "@faaas/handler";
import { database } from "@faaas/sql";

const sql = database({
  host: "postgres-db.cno4eviwxzxv.eu-west-2.rds.amazonaws.com",
  port: 5432,
  username: "faaasuser",
  password: "securepassword",
  database: "tpch",
  ssl: { rejectUnauthorized: false },
});

// Note: this is an HTTP handler, however liekly it would respond
// to an event from an internal pubsub queue based on real time sales data.

// Reorder parts from a supplier when their stock runs low
export async function handler(ctx: Context) {
  const [data = "{}"] = ctx.data;
  const { partSize: partSizeRaw, partType, regionName } = JSON.parse(data);

  const partSize = parseInt(partSizeRaw);

  if (partSize < 1 || partSize > 50)
    return result({
      message: "Days must be between 50 and 1",
    });

  const allowedPartTypes = ["TIN", "NICKEL", "BRASS", "STEEL", "COPPER"];
  const allowedRegionNames = [
    "AFRICA",
    "AMERICA",
    "ASIA",
    "EUROPE",
    "MIDDLE EAST",
  ];

  if (!allowedPartTypes.includes(partType))
    return result({
      message: `Part type must be one of: ${allowedPartTypes}`,
    });

  if (!allowedRegionNames.includes(regionName))
    return result({
      message: `Region must be one of: ${allowedRegionNames}`,
    });

  ("use async");
  const suppliers = await sql(`
    SELECT
        s_acctbal,
      s_name,
      n_name,
      p_partkey,
      p_mfgr,
      s_address,
      s_phone,
      s_comment
    FROM
        part,
      supplier,
      partsupp,
      nation,
      region
    WHERE
        p_partkey = ps_partkey
        AND s_suppkey = ps_suppkey
        AND p_size = ${partSize}
        AND p_type LIKE '%${partType}'
        AND s_nationkey = n_nationkey
        AND n_regionkey = r_regionkey
        AND r_name = '${regionName}'
        AND ps_supplycost = (
      SELECT
                MIN(ps_supplycost)
            FROM
                partsupp,
      supplier,
      nation,
      region
            WHERE
                p_partkey = ps_partkey
                AND s_suppkey = ps_suppkey
                AND s_nationkey = n_nationkey
                AND n_regionkey = r_regionkey
                AND r_name = '${regionName}'
    )
    ORDER BY
        s_acctbal DESC,
        n_name,
        s_name,
        p_partkey
    LIMIT 10;
  `);

  const selectedSupplier = suppliers[0];

  if (!selectedSupplier)
    return result({
      message: "No suppliers found",
    });

  const selectedSupplierSms = selectedSupplier["s_phone"];
  const notificationMessage = `New order for ${partSize} ${partType} parts in ${regionName}`;

  // await fetch("https://sms-provider.example.com");

  console.log("selectedSupplierSms:", selectedSupplierSms);
  console.log("notificationMessage:", notificationMessage);

  return result({
    message: "Supplier notified of new order",
    supplier: selectedSupplier,
  });
}

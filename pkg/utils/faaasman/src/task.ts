import { createHash } from "node:crypto";
import { createReadStream } from "node:fs";

export async function getTaskHash(taskPath: string): Promise<string> {
  const hash = createHash("sha1").setEncoding("hex");

  await new Promise((resolve, reject) =>
    createReadStream(taskPath)
      .on("end", resolve)
      .on("error", reject)
      .pipe(hash),
  );

  return hash.end().read();
}

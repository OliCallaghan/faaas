import { PrismaClient } from "@faaas/perf-v8-event-loop-db";

export const db = new PrismaClient();

export function sql<T>(callback: () => T) {
  return callback();
}

type Value = string | number | boolean;
type TaskContext = Record<string, any>;

type Continuation = {
  what: "continuation";
  taskId: string;
  args: Value[];
  ctx: TaskContext;
};

type Complete = {
  what: "complete";
  ctx: TaskContext;
};

type Result = Continuation | Complete;

export function continuation(
  taskId: string,
  args: Value[],
  ctx: TaskContext,
): Result {
  return {
    what: "continuation",
    taskId,
    args,
    ctx,
  };
}

export function result(ctx: TaskContext) {
  return {
    what: "complete",
    ctx,
  };
}

async function sql(query: string): Promise<string[]> {
  console.log(`Executing ${query}`);

  return [];
}

function listUserPets(ownerId: string, petType: string): string {
  return `SELECT name FROM pets WHERE type == ${petType} AND owner == ${ownerId}`;
}

async function getUsername(_userId: string): Promise<string> {
  // Dummy function
  return "bob";
}

// SOURCE CODE

export async function handler(ctx: TaskContext) {
  const { userId } = ctx;
  const username = await getUsername(userId);

  ("use async");
  const dogs = await sql(listUserPets(userId, "dog"));

  ("use async");
  const cats = await sql(listUserPets(userId, "cat"));

  return result({
    message: `Hello ${username}`,
    dogNames: dogs,
    catNames: cats,
  });
}

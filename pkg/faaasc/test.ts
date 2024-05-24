interface InvocationContext {
  userId: string;
}

async function sql(query: string): Promise<string[]> {
  console.log(`Executing ${query}`);

  return [];
}

function listUserPets(ownerId: string): string {
  return `SELECT name FROM pets WHERE owner == ${ownerId}`;
}

async function getUsername(_userId: string): Promise<string> {
  // Dummy function
  return "bob";
}

// SOURCE CODE

export async function handler(ctx: InvocationContext) {
  const { userId } = ctx;
  const username = await getUsername(userId);

  ("use async");
  const pets = await sql(listUserPets(userId));

  return {
    message: `Hello ${username}`,
    petNames: pets,
  };
}

// GENERATED CODE

type State = Record<string, any>;

async function sendToQueue(_cb: Function, state: State) {}

export async function handler_one(ctx: InvocationContext, _state: State) {
  const { userId } = ctx;
  const username = await getUsername(userId);

  // GENERATE FROM HERE
  return sendToQueue(() => sql(listUserPets(userId)), { username });
  // TO HERE
}

export async function handler_two(
  ctx: InvocationContext,
  state: State,
  resolved: string[],
) {
  // GENERATE FROM HERE
  const { username } = state;

  const pets = resolved;
  // GENERATE TO HERE

  return {
    message: `Hello ${username}`,
    petNames: pets,
  };
}

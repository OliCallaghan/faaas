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

  let x = 0;
  do {
    x += 1;
  } while (x < 5);

  for (let i = 0; i++; i < 5) {
    console.log("I");
  }

  for (const x in {}) {
    console.log("why");
  }

  if (x == 0) {
    console.log("happy");
  } else {
    console.log("sad");
  }

  let foo = "";
  switch (foo) {
    case "hello":
      console.log("Hi");
    case "bar":
      console.log("Buz");
    default:
      console.log("Quz");
  }

  if (foo == "never") {
    throw new Error("OH NO!");
  }

  try {
    throw new Error("expected");
  } catch (err) {
    console.log("Phew that was close");
  } finally {
    console.log("Cleanup");
  }

  while (foo == "not foo") {
    console.log("Hello world");
  }

  return {
    message: `Hello ${username}`,
    petNames: pets,
  };
}

// GENERATED CODE

type State = Record<string, any>;

async function sendToQueue(_cb: Function, state: State) {}

export async function generated_handler_one(
  ctx: InvocationContext,
  _state: State,
) {
  const { userId } = ctx;
  const username = await getUsername(userId);

  // GENERATE FROM HERE
  return sendToQueue(() => sql(listUserPets(userId)), { username });
  // TO HERE
}

export async function generated_handler_two(
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

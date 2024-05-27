type ContinuationState = Record<string, any>;
interface InvocationContext {
    userId: string;
}
async function sql(query: string): Promise<string[]> {
    console.log(`Executing ${query}`);
    return [];
}
function listUserPets(ownerId: string, petType: string): string {
    return `SELECT name FROM pets WHERE type == ${petType} AND owner == ${ownerId}`;
}
async function getUsername(_userId: string): Promise<string> {
    return "bob";
}
export async function handler(ctx: InvocationContext) {
    const { userId } = ctx;
    const username = await getUsername(userId);
    ("use async");
    const dogs = await sql(listUserPets(userId, "dog"));
    ("use async");
    const cats = await sql(listUserPets(userId, "cat"));
    return {
        message: `Hello ${username}`,
        dogNames: dogs,
        catNames: cats
    };
}
export async function handler_0(ctx: InvocationContext, state: ContinuationState) {
    const {} = state;
    const { userId } = ctx;
    const username = await getUsername(userId);
    const dogs = await sql(listUserPets(userId, "dog"));
    return {
        username,
        userId,
        dogs
    };
}
export async function handler_1(ctx: InvocationContext, state: ContinuationState) {
    const { username, userId, dogs } = state;
    const cats = await sql(listUserPets(userId, "cat"));
    return {
        username,
        dogs,
        cats
    };
}
export async function handler_2(ctx: InvocationContext, state: ContinuationState) {
    const { username, dogs, cats } = state;
    return {
        message: `Hello ${username}`,
        dogNames: dogs,
        catNames: cats
    };
    return {};
}

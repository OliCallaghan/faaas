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
export async function handler_0() {
    const { userId } = ctx;
    const username = await getUsername(userId);
}
export async function handler_1() {
    const dogs = await sql(listUserPets(userId, "dog"));
}
export async function handler_2() {
    const cats = await sql(listUserPets(userId, "cat"));
    return {
        message: `Hello ${username}`,
        dogNames: dogs,
        catNames: cats
    };
}

type Value = string | number | boolean;
type TaskContext = Record<string, any>;
type TaskContextState = TaskContext;
type Continuation = {
    what: "continuation";
    taskId: string;
    args: Value[];
    ctx: TaskContext;
};
type Result = {
    what: "complete";
    ctx: TaskContext;
};
function continuation(taskId: string, args: Value[], ctx: TaskContext): Continuation {
    return {
        what: "continuation",
        taskId,
        args,
        ctx
    };
}
function result(ctx: TaskContext): Result {
    return {
        what: "complete",
        ctx
    };
}
async function sql(query: string): Promise<string[]> {
    console.log(`Executing ${query}`);
    return [];
}
sql.continuation = "io/sql/pg";
function listUserPets(ownerId: string, petType: string): string {
    return `SELECT name FROM pets WHERE type == ${petType} AND owner == ${ownerId}`;
}
async function getUsername(_userId: string): Promise<string> {
    return "bob";
}
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
        catNames: cats
    });
}
export async function handler_0(ctx: TaskContext, state: TaskContextState) {
    const {} = state;
    const { userId } = ctx;
    const username = await getUsername(userId);
    const dogs = [
        listUserPets(userId, "dog")
    ];
    return continuation(sql.continuation, [
        "handler_1",
        "dogs",
        ...dogs
    ], {
        dogs,
        username,
        userId
    });
}
export async function handler_1(ctx: TaskContext, state: TaskContextState) {
    const { dogs, username, userId } = state;
    const cats = [
        listUserPets(userId, "cat")
    ];
    return continuation(sql.continuation, [
        "handler_2",
        "cats",
        ...cats
    ], {
        username,
        dogs,
        cats
    });
}
export async function handler_2(ctx: TaskContext, state: TaskContextState) {
    const { username, dogs, cats } = state;
    return result({
        message: `Hello ${username}`,
        dogNames: dogs,
        catNames: cats
    });
}

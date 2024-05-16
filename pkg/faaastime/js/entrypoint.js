import { Response } from 'faaas'
import { performance } from 'perf_hooks'

// import { sql } from 'faaas:sql'

export async function handler(req) {
    const now = performance.now();

    const data = req.json();
    const name = data.name ?? "faaas";

    // const oldies = await sql('SELECT name FROM person WHERE person.age > 50')
    const oldies = []

    return new Response(200).text(`Hello ${name}! Its ${now}, and there are ${oldies.length} oldies`);
}

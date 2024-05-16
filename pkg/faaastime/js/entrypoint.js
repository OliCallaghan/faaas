import { Buffer } from 'buffer'
import { randomBytes, pbkdf2Sync } from 'crypto'
import { Response } from 'faaas'
import { Performance } from 'perf_hooks'

export function handler(req) {
    const now = Performance.now();

    const data = req.json();
    const name = data.name ?? "faaas";

    return new Response(200).text(`Hello ${name}! Its ${now}`);
}

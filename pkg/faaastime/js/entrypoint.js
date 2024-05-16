import { Buffer } from 'buffer'
import { randomBytes, pbkdf2Sync } from 'crypto'
import { Response } from 'faaas'
import { Performance } from 'perf_hooks'

export function handler(req) {
    const now = Performance.now();

    return new Response(200).text(`Hello faaas! Its ${now}`);
}

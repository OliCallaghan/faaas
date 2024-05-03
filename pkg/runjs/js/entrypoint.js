import { Buffer } from 'buffer'
import { randomBytes, pbkdf2Sync } from 'crypto'
import { Response } from 'faaas'

export function handler(req) {
    return new Response(200).text("Hello faaas!");
}

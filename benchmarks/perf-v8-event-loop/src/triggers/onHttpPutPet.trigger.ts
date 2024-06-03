import express, { Request, Response } from "express";

import { db, sql } from "../database";

const app = express();

app.use(express.json());

async function handlePutPet(req: Request, res: Response) {
  const pet = await sql(() =>
    db.pet.create({
      data: {
        name: String(req.body.name),
        tag: String(req.body.tag),
      },
    }),
  );

  return void res.json(pet);
}

app.put("/", handlePutPet);

app.listen(3000, () => console.log("onHttpPutPet listening on PORT=3000"));

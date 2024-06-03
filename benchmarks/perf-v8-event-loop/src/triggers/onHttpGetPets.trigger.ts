import express, { Request, Response } from "express";

import { db, sql } from "../database";

const app = express();

app.use(express.json());

async function handleGetPets(_req: Request, res: Response) {
  const pets = await sql(() => db.pet.findMany());

  return void res.json(pets);
}

app.get("/", handleGetPets);

app.listen(3000, () => console.log("onHttpGetPets listening on PORT=3000"));

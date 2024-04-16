import express from "express";

import { db } from "../database";

const app = express();

app.use(express.json());

app.get("/", async (_req, res) => {
  const pets = await db.pet.findMany();

  return void res.json(pets);
});

app.listen(3000, () => console.log("onHttpGetPets listening on PORT=3000"));

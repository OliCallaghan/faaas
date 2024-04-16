import express from "express";

import { db } from "../database";

const app = express();

app.use(express.json());

app.put("/", async (_req, res) => {
  const pet = await db.pet.create({
    data: {
      name: String(_req.body.name),
      tag: String(_req.body.tag),
    },
  });

  return void res.json(pet);
});

app.listen(3000, () => console.log("onHttpPutPet listening on PORT=3000"));

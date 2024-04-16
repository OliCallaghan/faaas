import express from "express";

import { db } from "../database";

const app = express();

app.use(express.json());

app.delete("/:petId", async (_req, res) => {
  const pet = await db.pet.delete({
    where: {
      id: _req.params.petId,
    },
  });

  return void res.json(pet);
});

app.listen(3000, () => console.log("onHttpDeletePet listening on PORT=3000"));

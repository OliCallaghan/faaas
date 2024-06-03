import express, { Request, Response } from "express";

import { db, sql } from "../database";

const app = express();

app.use(express.json());

interface DeletePetParams {
  petId: string;
}
async function handleDeletePet(req: Request<DeletePetParams>, res: Response) {
  const pet = await sql(() =>
    db.pet.delete({
      where: {
        id: req.params.petId,
      },
    }),
  );

  return void res.json(pet);
}

app.delete("/:petId", handleDeletePet);

app.listen(3000, () => console.log("onHttpDeletePet listening on PORT=3000"));

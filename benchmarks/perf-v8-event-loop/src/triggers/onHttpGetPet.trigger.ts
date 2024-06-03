import express, { Request, Response } from "express";

import { db, sql } from "../database";

const app = express();

app.use(express.json());

interface GetPetParams {
  petId: string;
}
async function handleGetPet(req: Request<GetPetParams>, res: Response) {
  const pet = await sql(() =>
    db.pet.findUnique({
      where: {
        id: req.params.petId,
      },
    }),
  );

  return void res.json(pet);
}

app.get("/:petId", handleGetPet);

app.listen(3000, () => console.log("onHttpGetPet listening on PORT=3000"));

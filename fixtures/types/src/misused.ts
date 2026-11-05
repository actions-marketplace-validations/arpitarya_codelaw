import { save } from "./floating";

[1, 2].forEach(async () => {
  await save();
});

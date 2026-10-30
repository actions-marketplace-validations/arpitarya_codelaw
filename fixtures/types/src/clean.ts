import { save } from "./floating";

export async function run(): Promise<number> {
  await save();
  const n: number = await Promise.resolve(1);
  return n;
}

export async function settle(value: unknown): Promise<unknown> {
  return await value;
}

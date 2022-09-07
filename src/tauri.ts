import { invoke } from "@tauri-apps/api/tauri";
import { writeText } from "@tauri-apps/api/clipboard";

export type GenerationMethod = "schema" | "nonsense";

export async function gen_nonsense_passwords(
  length: number,
  amount: number,
  useRandomBytes: boolean,
  useSpecial: boolean
) {
  return await invoke<string[]>("generate_nonsense_passwords", {
    length,
    amount,
    useRandomBytes,
    useSpecial,
  });
}

export async function gen_schemed_passwords(
  hashLength: number,
  amount: number,
  isInverted: boolean,
  listOfWords: string[],
  useSpecial: boolean
) {
  return await invoke<string[]>("generate_schemed_passwords", {
    hashLength,
    amount,
    isInverted,
    listOfWords,
    useSpecial,
  });
}

export async function writeClipboard(text: string) {
  try {
    await writeText(text);
  } catch (e) {
    console.error(e);
  }
}

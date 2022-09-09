import { invoke } from "@tauri-apps/api/tauri";
import { writeText } from "@tauri-apps/api/clipboard";

export type GenerationMethod = "schema" | "nonsense" | "random-bytes";

export interface PasswordInfo {
  text: string;
  entropy: number;
}

export async function gen_nonsense_passwords(
  amount: number,
  length: number,
  useSpecial: boolean
) {
  return await invoke<PasswordInfo[]>("generate_nonsense_passwords", {
    amount,
    length,
    useSpecial,
  });
}

export async function gen_schemed_passwords(
  amount: number,
  hashLength: number,
  isInverted: boolean,
  listOfWords: string[],
  useSpecial: boolean
) {
  return await invoke<PasswordInfo[]>("generate_schemed_passwords", {
    amount,
    hashLength,
    isInverted,
    listOfWords,
    useSpecial,
  });
}

export async function gen_rand_bytes_passwords(amount: number, length: number) {
  return await invoke<PasswordInfo[]>("generate_random_bytes_passwords", {
    amount,
    length,
  });
}

export async function writeClipboard(text: string) {
  try {
    await writeText(text);
  } catch (e) {
    console.error(e);
  }
}

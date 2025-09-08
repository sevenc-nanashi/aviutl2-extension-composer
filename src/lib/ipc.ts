import { invoke } from "@tauri-apps/api/core";

export async function initializeProfile(
  name: string,
  path: string,
): Promise<string> {
  return await invoke("initialize_profile", { name, path });
}

export type IndexProfile = {
  id: string;
  name: string;
};
export async function listProfiles(): Promise<Record<string, IndexProfile>> {
  return await invoke("list_profiles");
}

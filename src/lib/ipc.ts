import { invoke } from "@tauri-apps/api/core";

export async function initializeProfile(options: {
  name: string;
  path: string;
  reinit: boolean;
}): Promise<string> {
  return await invoke("initialize_profile", options);
}

export type IndexProfile = {
  name: string;
  path: string;
};
export async function listProfiles(): Promise<Record<string, IndexProfile>> {
  return await invoke("list_profiles");
}

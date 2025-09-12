import { invoke } from "@tauri-apps/api/core";
import { Registry } from "./models/Registry.ts";

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

export async function listRegistries(): Promise<Record<string, string>> {
  return await invoke("list_registries");
}

export async function addRegistry(registry: string): Promise<void> {
  return await invoke("add_registry", { registry });
}

export async function fetchRegistry(registry: string): Promise<Registry> {
  return await invoke("fetch_registry", { registry });
}

export async function fetchRegistryCached(registry: string): Promise<Registry> {
  return await invoke("fetch_registry_cached", { registry });
}

export async function getRegistryUrl(registry: string): Promise<string> {
  return await invoke("get_registry_url", { registry });
}

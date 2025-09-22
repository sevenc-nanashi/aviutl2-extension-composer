import { invoke } from "@tauri-apps/api/core";
import { toBase64 } from "fast-base64";
import type { Manifest } from "./models/Manifest.d.ts";
import type { Registry } from "./models/Registry.d.ts";

export function parseErrorMessage<T extends unknown[], E>(
  error: E,
):
  | {
      type: "other";
      error: E;
    }
  | {
      type: "managed";
      kind: string;
      args: T;
    } {
  if (typeof error !== "string") {
    return { type: "other", error };
  }
  const pattern = /#(\w+)(\[(.*)\])?/;
  const match = error.match(pattern);
  if (!match) {
    return { type: "other", error };
  }
  const kind = match[1];
  const argsString = match[3];
  let args: T = [] as unknown as T;
  if (argsString) {
    try {
      args = JSON.parse(`[${argsString}]`) as T;
    } catch {
      return { type: "other", error };
    }
  }
  return { type: "managed", kind, args };
}

export type InitializeOnExist = "reuse_existing" | "remove_existing" | "abort";
export async function initializeProfile(options: {
  name: string;
  path: string;
  onExist: InitializeOnExist;
}): Promise<string> {
  return await invoke("initialize_profile", {
    name: options.name,
    path: options.path,
    onExist: options.onExist,
  });
}

export interface IndexProfile {
  name: string;
  path: string;
}
export async function listProfiles(): Promise<Record<string, IndexProfile>> {
  return await invoke("list_profiles");
}

export async function unregisterProfile(profileId: string): Promise<void> {
  return await invoke("unregister_profile", { profileId });
}

export async function removeProfile(profileId: string): Promise<void> {
  return await invoke("remove_profile", { profileId });
}

export async function openProfileFolder(profileId: string): Promise<void> {
  return await invoke("open_profile_folder", { profileId });
}

export async function listRegistries(): Promise<Record<string, string>> {
  return await invoke("list_registries");
}

export async function addRegistry(registry: string): Promise<void> {
  return await invoke("add_registry", { registry });
}

export async function removeRegistry(registry: string): Promise<void> {
  return await invoke("remove_registry", { registry });
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

export async function fetchManifest(manifestUrl: string): Promise<Manifest> {
  return await invoke("fetch_manifest", { manifestUrl });
}
export async function fetchManifestCached(
  manifestUrl: string,
): Promise<Manifest> {
  return await invoke("fetch_manifest_cached", { manifestUrl });
}

export async function listManifests(): Promise<Record<string, string>> {
  return await invoke("list_manifests");
}
export async function addManifestUrl(manifest: string): Promise<void> {
  return await invoke("add_manifest_url", { manifest });
}
export async function addManifestLocal(file: Uint8Array): Promise<void> {
  return await invoke("add_manifest_local", { file: await toBase64(file) });
}
export async function removeManifest(manifest: string): Promise<void> {
  return await invoke("remove_manifest", { manifest });
}

export interface Version {
  version: string;
  version_number?: number | undefined | null;
}
export interface ProfileStore {
  name: string;
  contents: Record<string, Manifest>;
}
export async function getProfileStore(
  profileId: string,
): Promise<ProfileStore> {
  return await invoke("get_profile_store", { profileId });
}

export interface InstallPlan {
  to_keep: Manifest[];
  to_update: [Manifest, Manifest][];
  to_install: Manifest[];
}
export async function planInstallation(
  profileId: string,
  desiredManifests: Manifest[],
): Promise<InstallPlan> {
  return await invoke("plan_installation", {
    profileId,
    desiredManifests,
  });
}

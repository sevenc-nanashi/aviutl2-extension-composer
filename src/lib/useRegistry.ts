import { flushPromises, peekPromiseState } from "@core/asyncutil";
import { computed, ref, watch } from "vue";
import * as ipc from "./ipc.ts";
import type { Manifest } from "./models/Manifest.d.ts";
import type { Registry } from "./models/Registry.d.ts";
import { useRefreshableAsync } from "./useAsync.ts";
import { compareVersions } from "./version.ts";

export const localRegistry = "local" as const;
export type LocalRegistry = typeof localRegistry;

export function useRegistry() {
  const registryPromises = new Map<string, Promise<Registry>>();
  const registryValues = ref(new Map<string, Registry>());
  const registries = useRefreshableAsync(async () => {
    return await ipc.listRegistries();
  });
  const localManifests = useRefreshableAsync(async () => {
    return await ipc.listManifests();
  });
  const localManifestPromises = new Map<string, Promise<Manifest>>();
  const localManifestValues = ref(new Map<string, Manifest>());

  const registryErrors = ref(new Map<string, unknown>());
  const localManifestErrors = ref(new Map<string, unknown>());

  const registryFetchStatuses = ref(
    new Map<string, "pending" | "fulfilled" | "rejected">(),
  );
  const localManifestFetchStatuses = ref(
    new Map<string, "pending" | "fulfilled" | "rejected">(),
  );

  watch(
    () => registries.value,
    async () => {
      const registryData =
        registries.value.state === "success" ? registries.value.data : {};

      // cleanup removed ids
      const ids = new Set(Object.keys(registryData));
      for (const key of registryValues.value.keys())
        if (!ids.has(key)) registryValues.value.delete(key);
      for (const key of registryErrors.value.keys())
        if (!ids.has(key)) registryErrors.value.delete(key);
      for (const key of registryFetchStatuses.value.keys())
        if (!ids.has(key)) registryFetchStatuses.value.delete(key);

      await flushPromises();
      for (const [id, url] of Object.entries(registryData)) {
        const existingPromise = registryPromises.get(id);
        if (existingPromise) {
          const status = await peekPromiseState(existingPromise);
          if (status === "fulfilled") {
            const data = await existingPromise;
            registryValues.value.set(id, data);
            registryFetchStatuses.value.set(id, "fulfilled");
            registryErrors.value.delete(id);
          } else if (status === "rejected") {
            registryFetchStatuses.value.set(id, "rejected");
            void existingPromise.catch((error) => {
              registryErrors.value.set(id, error);
            });
          } else {
            registryFetchStatuses.value.set(id, "pending");
          }
        } else {
          registryFetchStatuses.value.set(id, "pending");
          const promise = ipc
            .fetchRegistryCached(url)
            .then((data) => {
              registryValues.value.set(id, data);
              registryFetchStatuses.value.set(id, "fulfilled");
              registryErrors.value.delete(id);
              return data;
            })
            .catch((error) => {
              registryFetchStatuses.value.set(id, "rejected");
              registryErrors.value.set(id, error);
              throw error;
            });
          registryPromises.set(id, promise);
        }
      }
    },
  );

  watch(
    () => localManifests.value,
    async () => {
      if (localManifests.value.state !== "success") {
        return;
      }

      const manifestData = localManifests.value.data;
      // cleanup removed ids
      const ids = new Set(Object.keys(manifestData));
      for (const key of localManifestValues.value.keys())
        if (!ids.has(key)) localManifestValues.value.delete(key);
      for (const key of localManifestErrors.value.keys())
        if (!ids.has(key)) localManifestErrors.value.delete(key);
      for (const key of localManifestFetchStatuses.value.keys())
        if (!ids.has(key)) localManifestFetchStatuses.value.delete(key);

      await flushPromises();
      for (const [id, manifestUrl] of Object.entries(manifestData)) {
        const existingPromise = localManifestPromises.get(id);
        if (existingPromise) {
          const status = await peekPromiseState(existingPromise);
          if (status === "fulfilled") {
            const data = await existingPromise;
            localManifestValues.value.set(id, data);
            localManifestFetchStatuses.value.set(id, "fulfilled");
            localManifestErrors.value.delete(id);
          } else if (status === "rejected") {
            localManifestFetchStatuses.value.set(id, "rejected");
            void existingPromise.catch((error) => {
              localManifestErrors.value.set(id, error);
            });
          } else {
            localManifestFetchStatuses.value.set(id, "pending");
          }
        } else {
          localManifestFetchStatuses.value.set(id, "pending");
          const promise = ipc
            .fetchManifestCached(manifestUrl)
            .then((data) => {
              localManifestValues.value.set(id, data);
              localManifestFetchStatuses.value.set(id, "fulfilled");
              localManifestErrors.value.delete(id);
              return data;
            })
            .catch((error) => {
              localManifestFetchStatuses.value.set(id, "rejected");
              localManifestErrors.value.set(id, error);
              throw error;
            });
          localManifestPromises.set(id, promise);
        }
      }
    },
  );

  const contents = computed(() => {
    const contents = new Map<string, Registry["contents"][0] | Manifest>();
    for (const manifest of localManifestValues.value.values()) {
      const existing = contents.get(manifest.id);
      if (!existing || compareVersions(existing, manifest) === -1) {
        contents.set(manifest.id, manifest);
      }
    }
    for (const registry of registryValues.value.values()) {
      for (const content of registry.contents) {
        const existing = contents.get(content.id);
        if (!existing || compareVersions(existing, content) === -1) {
          contents.set(content.id, content);
        }
      }
    }

    return new Map(
      Array.from(contents.entries()).sort((a, b) => a[0].localeCompare(b[0])),
    );
  });

  const contentToRegistry = (
    contentId: string,
  ): LocalRegistry | string | null => {
    const content = contents.value.get(contentId);
    if (!content) return null as string | null;
    for (const manifest of localManifestValues.value.values()) {
      if (
        manifest.id === contentId &&
        manifest.version === content.version &&
        manifest.version_number === content.version_number
      ) {
        return localRegistry;
      }
    }
    for (const [registryId, registry] of registryValues.value.entries()) {
      if (
        registry.contents.some(
          (c) =>
            c.id === contentId &&
            c.version === content.version &&
            c.version_number === content.version_number,
        )
      ) {
        return registryId;
      }
    }
    return null as string | null;
  };

  const contentsLoaded = computed(() => {
    // loaded when registries list resolved and all fetches for registries/manifests settled (fulfilled or rejected)
    const registriesResolved = registries.value.state !== "loading";
    const manifestsResolved = localManifests.value.state !== "loading";
    if (!registriesResolved || !manifestsResolved) return false;

    // If there is no data (error or empty), consider settled for this part
    const regIds =
      registries.value.state === "success" ?
        Object.keys(registries.value.data)
      : [];
    const manIds =
      localManifests.value.state === "success" ?
        Object.keys(localManifests.value.data)
      : [];

    for (const id of regIds) {
      const s = registryFetchStatuses.value.get(id);
      if (s === undefined || s === "pending") return false;
    }
    for (const id of manIds) {
      const s = localManifestFetchStatuses.value.get(id);
      if (s === undefined || s === "pending") return false;
    }
    return true;
  });

  const errors = computed(() => {
    const list: Array<{
      source: "registry" | "manifest";
      id: string;
      error: unknown;
    }> = [];
    for (const [id, err] of registryErrors.value.entries()) {
      list.push({ source: "registry", id, error: err });
    }
    for (const [id, err] of localManifestErrors.value.entries()) {
      list.push({ source: "manifest", id, error: err });
    }
    return list;
  });

  return {
    registries,
    registryValues,
    localManifests,
    localManifestValues,
    contents,
    contentToRegistry,
    contentsLoaded,
    errors,
  } as const;
}

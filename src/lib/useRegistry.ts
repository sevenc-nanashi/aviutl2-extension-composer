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

  watch(
    () => registries.value,
    async () => {
      const registryData =
        registries.value.state === "success" ? registries.value.data : {};
      registryValues.value.clear();
      await flushPromises();
      for (const [id, url] of Object.entries(registryData)) {
        const existingPromise = registryPromises.get(id);
        if (existingPromise) {
          const status = await peekPromiseState(existingPromise);
          if (status === "fulfilled") {
            const data = await existingPromise;
            registryValues.value.set(id, data);
          }
        } else {
          const promise = ipc.fetchRegistryCached(url).then((data) => {
            registryValues.value.set(id, data);
            return data;
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
      localManifestValues.value.clear();
      await flushPromises();
      for (const [id, manifestUrl] of Object.entries(
        localManifests.value.data,
      )) {
        const existingPromise = localManifestPromises.get(id);
        if (existingPromise) {
          const status = await peekPromiseState(existingPromise);
          if (status === "fulfilled") {
            const data = await existingPromise;
            localManifestValues.value.set(id, data);
          }
        } else {
          const promise = ipc.fetchManifestCached(manifestUrl).then((data) => {
            localManifestValues.value.set(id, data);
            return data;
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

  return {
    registries,
    registryValues,
    localManifests,
    localManifestValues,
    contents,
    contentToRegistry,
  } as const;
}

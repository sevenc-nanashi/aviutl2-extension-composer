import { computed, ref, watch } from "vue";
import * as ipc from "./ipc.ts";
import type { Registry } from "./models/Registry.d.ts";
import { useRefreshableAsync } from "./useAsync.ts";
import { compareVersions } from "./version.ts";

export function useRegistry() {
  const registryPromises = new Map<string, Promise<Registry>>();
  const registryValues = ref(new Map<string, Registry>());
  const registries = useRefreshableAsync(async () => {
    return await ipc.listRegistries();
  });

  watch(
    () => registries.value,
    async () => {
      const registryData =
        registries.value.state === "success" ? registries.value.data : {};
      registryValues.value.clear();
      for (const [id, url] of Object.entries(registryData)) {
        if (!registryPromises.has(id)) {
          const promise = ipc.fetchRegistryCached(url).then((data) => {
            registryValues.value.set(id, data);
            return data;
          });
          registryPromises.set(id, promise);
        }
      }
    },
  );

  const contents = computed(() => {
    if (registries.value.state !== "success")
      return new Map<string, Registry["contents"][0]>();
    const contents = new Map<string, Registry["contents"][0]>();
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

  const contentToRegistry = (contentId: string) => {
    const content = contents.value.get(contentId);
    if (!content) return null as string | null;
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

  return { registries, registryValues, contents, contentToRegistry } as const;
}

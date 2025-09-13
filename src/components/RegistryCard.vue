<script setup lang="ts">
import { useI18n } from "vue-i18n";
import { open as openUrl } from "@tauri-apps/plugin-shell";
import { useAsync } from "../lib/useAsync.ts";
import * as ipc from "../lib/ipc.ts";
import { errorToLocalizedString } from "../lib/error.ts";
import { useCopy } from "../lib/useCopy.ts";
import Loading from "./Loading.vue";
import Icon from "./Icon.vue";

const props = defineProps<{
  registry: string;
}>();
const i18n = useI18n();
const { t } = i18n;
const copy = useCopy();

const registry = useAsync(() => ipc.fetchRegistryCached(props.registry));
const url = useAsync(() => ipc.getRegistryUrl(props.registry));
</script>
<template>
  <div
    v-if="registry.state === 'loading' || url.state === 'loading'"
    class="card"
    un-text="inherit"
    un-flex="~ col"
    un-gap="2"
  >
    <Loading />
  </div>
  <div
    v-else-if="registry.state === 'error' || url.state === 'error'"
    class="card error"
    un-text="inherit"
  >
    {{
      registry.state === "error" ? errorToLocalizedString(t, registry.error)
      : url.state === "error" ? errorToLocalizedString(t, url.error)
      : "(unreachable)"
    }}
  </div>
  <div v-else class="card" un-text="inherit" un-flex="~ col" un-gap="2">
    <h3 un-text="lg">
      {{ registry.data.name }}
    </h3>

    <a
      v-if="registry.data.homepage"
      un-font="mono"
      un-text="slate-500"
      un-flex="~ nowrap"
      un-justify="start"
      un-items="center"
      un-gap="1"
      un-overflow="hidden"
      un-cursor="pointer"
      @click.stop.prevent="openUrl(registry.data.homepage)"
    >
      <Icon un-i="fluent-open-24-regular" un-text-lg un-aspect-square />
      <p
        un-text="xs nowrap ellipsis"
        un-overflow="hidden"
        un-whitespace="nowrap"
        un-w="full"
      >
        {{ registry.data.homepage }}
      </p>
    </a>
    <a
      un-font="mono"
      un-text="slate-500"
      un-flex="~ nowrap"
      un-justify="start"
      un-items="center"
      un-gap="1"
      un-overflow="hidden"
      un-cursor="pointer"
      @click.stop="copy(url.data)"
    >
      <Icon un-i="fluent-link-24-regular" un-text-lg un-aspect-square />
      <p
        un-text="xs nowrap ellipsis"
        un-overflow="hidden"
        un-whitespace="nowrap"
        un-w="full"
      >
        {{ url.data }}
      </p>
    </a>
  </div>
</template>

<i18n lang="yaml">
ja:
  copied_to_clipboard: "クリップボードにコピーしました"
</i18n>

<script setup lang="ts">
import { open as openUrl } from "@tauri-apps/plugin-shell";
import { useI18n } from "vue-i18n";
import { errorToLocalizedString } from "../lib/error.ts";
import * as ipc from "../lib/ipc.ts";
import { useAsync } from "../lib/useAsync.ts";
import { useCopy } from "../lib/useCopy.ts";
import CardSmallText from "./CardSmallText.vue";
import Icon from "./Icon.vue";
import Loading from "./Loading.vue";

const props = defineProps<{
  registry: string;
}>();
const i18n = useI18n();
const { t } = i18n;
const copy = useCopy(t);

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
    <slot />
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

    <a
      un-font="mono"
      un-text="slate-500"
      un-flex="~ nowrap"
      un-justify="start"
      un-items="center"
      un-gap="1"
      un-overflow="hidden"
      un-cursor="pointer"
      @click.stop="copy(url.state === 'error' ? props.registry : url.data)"
    >
      <Icon un-i="fluent-link-24-regular" un-text-lg un-aspect-square />
      <p
        un-text="xs nowrap ellipsis"
        un-overflow="hidden"
        un-whitespace="nowrap"
        un-w="full"
      >
        {{ url.state === "error" ? props.registry : url.data }}
      </p>
    </a>
    <slot />
  </div>
  <div v-else class="card" un-text="inherit" un-flex="~ col" un-gap="2">
    <h3 un-text="lg" un-w="max" un-max-w="full">
      {{ registry.data.name }}
    </h3>

    <p un-text="xs pretty" un-w="full">
      {{ registry.data.summary }}
    </p>
    <CardSmallText
      v-if="registry.data.homepage"
      tag="a"
      un-font="mono"
      un-text="slate-500"
      un-i="fluent-open-24-regular"
      un-cursor="pointer"
      @click.stop.prevent="openUrl(registry.data.homepage)"
    >
      <p
        un-text="xs nowrap ellipsis"
        un-overflow="hidden"
        un-whitespace="nowrap"
        un-w="full"
      >
        {{ registry.data.homepage }}
      </p>
    </CardSmallText>
    <CardSmallText
      v-if="url.data"
      tag="a"
      un-font="mono"
      un-text="slate-500"
      un-cursor="pointer"
      un-i="fluent-link-24-regular"
      @click.stop.prevent="copy(url.data)"
    >
      <p
        un-text="xs nowrap ellipsis"
        un-overflow="hidden"
        un-whitespace="nowrap"
        un-w="full"
      >
        {{ url.data }}
      </p>
    </CardSmallText>

    <slot />
  </div>
</template>

<i18n lang="yaml">
ja:
  copied_to_clipboard: "クリップボードにコピーしました"
</i18n>

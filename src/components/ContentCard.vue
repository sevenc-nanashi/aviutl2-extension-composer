<script setup lang="ts">
import { open as openUrl } from "@tauri-apps/plugin-shell";
import { useI18n } from "vue-i18n";
import type { Registry } from "../lib/models/Registry.d.ts";
import { useCopy } from "../lib/useCopy.ts";
import CardSmallText from "./CardSmallText.vue";

const props = defineProps<{
  content: Registry["contents"][number];
}>();
const i18n = useI18n();
const { t } = i18n;
const copy = useCopy(t);
</script>
<template>
  <div class="card" un-text="inherit" un-flex="~ col" un-gap="2">
    <h3 un-text="lg">
      {{ props.content.name }}
    </h3>

    <p un-text="xs pretty" un-w="full">
      {{ props.content.summary }}
    </p>
    <CardSmallText
      v-if="props.content.homepage"
      tag="a"
      un-font="mono"
      un-text="slate-500"
      un-i="fluent-number-symbol-24-regular"
      un-cursor="pointer"
      @click.stop.prevent="copy(props.content.id)"
    >
      <p un-text="xs" un-overflow="hidden" un-w="full">
        {{ props.content.id }}
      </p>
    </CardSmallText>
    <CardSmallText
      v-if="props.content.homepage"
      tag="a"
      un-font="mono"
      un-text="slate-500"
      un-cursor="pointer"
      un-i="fluent-open-24-regular"
      @click.stop.prevent="openUrl(props.content.homepage)"
    >
      <p
        un-text="xs nowrap ellipsis"
        un-overflow="hidden"
        un-whitespace="nowrap"
        un-w="full"
      >
        {{ props.content.homepage }}
      </p>
    </CardSmallText>
  </div>
</template>

<i18n lang="yaml">
ja:
  copied_to_clipboard: "クリップボードにコピーしました"
</i18n>

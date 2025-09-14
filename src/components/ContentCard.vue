<script setup lang="ts">
import { open as openUrl } from "@tauri-apps/plugin-shell";
import { ref } from "vue";
import { useI18n } from "vue-i18n";
import type { Manifest } from "../lib/models/Manifest.d.ts";
import type { Registry } from "../lib/models/Registry.d.ts";
import { ensureNotNullish } from "../lib/null.ts";
import { useCopy } from "../lib/useCopy.ts";
import CardSmallText from "./CardSmallText.vue";
import ContentDialog from "./ContentDialog.vue";

const props = defineProps<{
  content: Registry["contents"][number] | Manifest;
}>();
const i18n = useI18n();
const { t } = i18n;
const copy = useCopy(t);
const showManifestDialog = ref(false);
</script>
<template>
  <div class="card" un-text="inherit" un-flex="~ col" un-gap="2">
    <ContentDialog
      v-model:open="showManifestDialog"
      :manifest-url="ensureNotNullish(props.content.manifest_url)"
    />
    <h3
      un-text="lg"
      un-cursor="pointer"
      un-underline="hover:~"
      un-w="max"
      un-max-w="full"
      @click.stop.prevent="showManifestDialog = true"
    >
      {{ props.content.name }}
    </h3>

    <p un-text="xs pretty" un-w="full">
      {{ props.content.summary }}
    </p>
    <CardSmallText un-text="slate-500" un-i="fluent-person-24-regular">
      <a
        un-text="xs nowrap ellipsis slate-500"
        un-overflow="hidden"
        un-whitespace="nowrap"
        un-max-w="full"
        un-cursor="pointer"
        @click.stop.prevent="
          props.content.authors[0].url && openUrl(props.content.authors[0].url)
        "
      >
        {{ props.content.authors[0].name }}
      </a>
      <template
        v-if="props.content.authors && props.content.authors.length > 1"
      >
        {{
          t("moreN", {
            n: props.content.authors.length - 1,
          })
        }}
      </template>
    </CardSmallText>
    <CardSmallText
      tag="a"
      un-font="mono"
      un-text="slate-500"
      un-i="fluent-number-symbol-24-regular"
      clickable
      @click.stop.prevent="copy(props.content.id)"
    >
      <p un-text="xs" un-overflow="hidden" un-max-w="full" un-cursor="pointer">
        {{ props.content.id }}
      </p>
    </CardSmallText>
    <CardSmallText
      un-font="mono"
      un-text="slate-500"
      un-i="fluent-tag-24-regular"
    >
      <p un-text="xs" un-overflow="hidden">
        {{ props.content.version }}
      </p>
    </CardSmallText>
    <CardSmallText
      v-if="props.content.homepage"
      tag="a"
      un-font="mono"
      un-text="slate-500"
      un-i="fluent-open-24-regular"
      clickable
      @click.stop.prevent="openUrl(props.content.homepage)"
    >
      <p
        un-text="xs nowrap ellipsis"
        un-overflow="hidden"
        un-whitespace="nowrap"
        un-cursor="pointer"
        un-max-w="full"
      >
        {{ props.content.homepage }}
      </p>
    </CardSmallText>
    <slot />
  </div>
</template>

<i18n lang="yaml">
ja:
  copied_to_clipboard: "クリップボードにコピーしました"
</i18n>

<script setup lang="ts">
import { open as openUrl } from "@tauri-apps/plugin-shell";
import { DialogDescription } from "reka-ui";
import { ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import { errorToLocalizedString } from "../lib/error.ts";
import * as ipc from "../lib/ipc.ts";
import type { Manifest } from "../lib/models/Manifest.d.ts";
import { useAsync } from "../lib/useAsync.ts";
import { useCopy } from "../lib/useCopy.ts";
import { useMaybeLocalized } from "../lib/useMaybeLocalized.ts";
import CardSmallText from "./CardSmallText.vue";
import Dialog from "./Dialog.vue";
import Loading from "./Loading.vue";
import Markdown from "./Markdown.vue";

const props = defineProps<{
  manifestUrl?: string;
  manifest?: Manifest;
}>();

const open = defineModel<boolean>("open", {
  required: true,
});
const isVisible = ref(open);
watch(
  () => open,
  (v) => {
    if (v) {
      isVisible.value = true;
    }
  },
);

const manifest = useAsync(() => {
  if (props.manifest) {
    return Promise.resolve(props.manifest);
  }
  if (props.manifestUrl) {
    return ipc.fetchManifestCached(props.manifestUrl);
  }

  return Promise.reject(new Error("No manifest or manifestUrl provided"));
});
const i18n = useI18n();
const localize = useMaybeLocalized(i18n);
const copy = useCopy(i18n.t);
const { t } = i18n;
</script>

<template>
  <Dialog
    :open="isVisible"
    :allow-close="true"
    :color="manifest.state === 'error' ? 'error' : undefined"
    @update:open="(v) => !v && (open = false)"
    @disappeared="isVisible = false"
  >
    <template #title>
      <template v-if="manifest.state === 'loading'">
        {{ t("loading") }}
      </template>
      <template v-else-if="manifest.state === 'error'">
        {{ t("error") }}
      </template>
      <template v-else>
        {{ localize(manifest.data.name) }}
      </template>
    </template>
    <template v-if="manifest.state === 'loading'">
      <Loading />
    </template>
    <template v-else-if="manifest.state === 'error'">
      <DialogDescription>
        {{ errorToLocalizedString(i18n.t, manifest.error) }}
      </DialogDescription>
    </template>
    <template v-else>
      <p un-text="sm pretty" un-w="full">
        {{ localize(manifest.data.summary) }}
      </p>

      <CardSmallText un-text="slate-500" un-i="fluent-person-24-regular">
        <template v-for="(author, index) in manifest.data.authors" :key="index">
          <span v-if="index > 0">{{ t("separator") }}</span>
          <a
            un-text="xs nowrap ellipsis slate-500"
            un-overflow="hidden"
            un-whitespace="nowrap"
            un-cursor="pointer"
            @click.stop.prevent="author.url && openUrl(author.url)"
          >
            {{ author.name }}
          </a>
        </template>
      </CardSmallText>
      <CardSmallText
        tag="a"
        un-font="mono"
        un-text="slate-500"
        un-i="fluent-number-symbol-24-regular"
        un-cursor="pointer"
        @click.stop.prevent="copy(manifest.data.id)"
      >
        <p un-text="xs" un-overflow="hidden">
          {{ manifest.data.id }}
        </p>
      </CardSmallText>
      <CardSmallText
        un-font="mono"
        un-text="slate-500"
        un-i="fluent-tag-24-regular"
      >
        <p un-text="xs" un-overflow="hidden">
          {{ manifest.data.version }}
        </p>
      </CardSmallText>

      <CardSmallText
        v-if="manifest.data.homepage"
        tag="a"
        un-font="mono"
        un-text="slate-500"
        un-cursor="pointer"
        un-i="fluent-open-24-regular"
        @click.stop.prevent="openUrl(manifest.data.homepage)"
      >
        <p
          un-text="xs nowrap ellipsis"
          un-overflow="hidden"
          un-whitespace="nowrap"
        >
          {{ manifest.data.homepage }}
        </p>
      </CardSmallText>

      <div v-if="manifest.data.description" un-text="xs pretty">
        <hr un-my="2" />
        <DialogDescription>
          <Markdown :content="localize(manifest.data.description)" />
        </DialogDescription>
      </div>
    </template>
  </Dialog>
</template>

<i18n lang="yaml">
ja:
  title: マニフェスト
</i18n>

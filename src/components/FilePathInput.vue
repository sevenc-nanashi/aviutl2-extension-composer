<script lang="ts">
export type Filter = {
  name: string;
  extensions: string[];
};
</script>
<script setup lang="ts" generic="T extends boolean">
import {
  open as openFileDialog,
  save as saveFileDialog,
  OpenDialogOptions,
} from "@tauri-apps/plugin-dialog";
import { stat as statFile } from "@tauri-apps/plugin-fs";
import { ref, watch } from "vue";
const model = defineModel<T extends true ? string[] : string>({
  required: true,
});

const props = defineProps<{
  type: "open" | "save";
  title?: string;
  disabled?: boolean;
  placeholder?: string;
  directory?: boolean;
  multiple?: T;
  filters?: Filter[];
  defaultPath?: string;
  mustExist?: boolean;
}>();

const openDialog = async () => {
  if (props.disabled) return;
  const options: OpenDialogOptions = {
    multiple: props.multiple,
    directory: props.directory,
    filters: props.filters,
    canCreateDirectories: true,
    defaultPath:
      (props.multiple ?
        model.value?.[0]
      : (model.value as string | undefined)) || props.defaultPath,
  };
  let selected: string | string[] | null = null;
  if (props.type === "open") {
    selected = await openFileDialog(options);
  } else {
    selected = await saveFileDialog(options);
  }
  if (selected) {
    model.value = selected as T extends true ? string[] : string;
  }
};

const exists = ref(false);
watch(
  () => model.value,
  async (newPath) => {
    const pathsToCheck =
      props.multiple ? (newPath as string[] | undefined) || []
      : newPath ? [newPath as string]
      : [];
    if (pathsToCheck.length === 0) {
      exists.value = false;
      return;
    }

    try {
      const stats = await Promise.all(
        pathsToCheck.map((path) => statFile(path)),
      );
      exists.value = stats.every((stat) =>
        props.directory ? stat.isDirectory : stat.isFile,
      );
    } catch (e) {
      console.error(e);
      exists.value = false;
    }
  },
  { immediate: true },
);
</script>

<template>
  <button
    class="file-path-input"
    :class="{ disabled: disabled, error: mustExist && !exists }"
    un-bg="white"
    un-font="mono"
    un-border="solid 2 slate-300"
    un-shadow="inset sm slate-300/50"
    un-outline="none"
    un-rounded="md"
    un-relative
    un-overflow="hidden"
    un-text="sm left"
    un-p="x-2 y-1.5"
    un-cursor="pointer"
    @click="openDialog"
  >
    <template v-if="model">
      {{ model }}
    </template>
    <span v-else un-text="slate-400">{{ placeholder }}</span>
    <div
      class="file-icon"
      un-absolute
      un-block
      un-top="1/2"
      un-right="2"
      un-translate-y="-1/2"
      un-size="4"
      un-cursor="pointer"
      un-i="fluent-folder-open-16-regular"
      un-pointer-events="none"
    />
  </button>
</template>

<style scoped>
.file-path-input {
  &.disabled {
    cursor: not-allowed;
    --at-apply: bg-slate-200/50;
  }
  &.error {
    --at-apply: border-pink-400;
  }

  & .file-icon {
    transition: color 0.3s;
    --at-apply: text-slate-400;
  }

  &:where(:hover, :focus) .file-icon {
    --at-apply: text-pink-500;
  }
}
</style>

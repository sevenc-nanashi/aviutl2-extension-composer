<script setup lang="ts" generic="T extends boolean">
import { computed, ref, useId } from "vue";
import CommonInput from "./CommonInput.vue";
import CommonInputSideIcon from "./CommonInputSideIcon.vue";

const model = defineModel<T extends true ? File[] : File | null>({
  required: true,
});

const props = withDefaults(
  defineProps<{
    required?: boolean;
    disabled?: boolean;
    multiple?: T;
    accept?: string | string[]; // e.g. ".yml,.yaml,.json" or [".yml",".yaml",".json"]
    placeholder?: string;
    borderless?: boolean;
  }>(),
  {
    required: false,
    disabled: false,
    // @ts-expect-error なんかうまく型が合わない
    multiple: false as T,
    accept: undefined,
    placeholder: "",
    borderless: false,
  },
);

const id = useId();

const acceptAttr = computed(() => {
  if (!props.accept) return undefined;
  return Array.isArray(props.accept) ? props.accept.join(",") : props.accept;
});

const hasValue = computed(() => {
  return Array.isArray(model.value) ? model.value.length > 0 : !!model.value;
});

const fileNames = computed(() => {
  if (!hasValue.value) return "";
  if (Array.isArray(model.value))
    return model.value.map((f) => f.name).join(", ");
  return (model.value as File).name;
});

const error = computed(() => props.required && !hasValue.value);

const inputEl = ref<HTMLInputElement | null>(null);
const openPicker = () => inputEl.value?.click();

const onChange = (e: Event) => {
  const files = (e.target as HTMLInputElement).files;
  if (!files || files.length === 0) {
    model.value = (props.multiple ? [] : null) as T extends true ? File[]
    : File | null;
    return;
  }
  if (props.multiple) {
    model.value = Array.from(files) as T extends true ? File[] : File | null;
  } else {
    model.value = files[0] as T extends true ? File[] : File | null;
  }
};
</script>

<template>
  <CommonInput
    un-cursor="pointer"
    :error="error"
    :disabled="props.disabled"
    :borderless="props.borderless"
  >
    <CommonInputSideIcon
      :for="id"
      side="left"
      un-i="fluent-document-16-regular"
      un-text="slate-400"
    />
    <button
      :id="id"
      type="button"
      class="file-input-btn"
      :disabled="props.disabled"
      un-p="x-2 y-1.5"
      un-pl="8"
      un-text="sm left"
      un-border="none"
      un-shadow="none"
      un-outline="none"
      un-rounded="md"
      un-bg="transparent"
      un-w="full"
      @click="openPicker"
    >
      <template v-if="hasValue">
        {{ fileNames }}
      </template>
      <span v-else un-text="slate-400">{{ props.placeholder }}</span>
    </button>
    <input
      ref="inputEl"
      type="file"
      :multiple="Boolean(props.multiple)"
      :accept="acceptAttr"
      :disabled="props.disabled"
      class="hidden"
      @change="onChange"
    />
  </CommonInput>
</template>

<style scoped>
.hidden {
  display: none;
}
.file-input-btn:disabled {
  cursor: not-allowed;
}
</style>

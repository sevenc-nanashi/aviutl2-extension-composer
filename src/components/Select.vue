<script setup lang="ts" generic="T extends string | number">
import { useId } from "vue";
import Icon from "./Icon.vue";

const model = defineModel<T>({
  required: true,
});

const props = defineProps<{
  options: {
    value: T;
    label: string;
  }[];
  borderless?: boolean;
  disabled?: boolean;
}>();

const id = useId();
</script>

<template>
  <div
    un-relative
    :un-cursor="props.disabled ? 'not-allowed' : 'pointer'"
    :un-border="props.borderless ? 'none' : 'solid 2 slate-300'"
    :un-shadow="props.borderless ? '' : 'inset sm slate-300/50'"
    :un-bg="
      props.borderless
        ? 'transparent hover:blue-200/30'
        : props.disabled
          ? 'slate-200'
          : 'white'
    "
    :un-opacity="props.disabled ? '50' : '100'"
    un-transition-colors
    un-duration="300"
    un-outline="none"
    un-rounded="md"
    un-align-content="center"
  >
    <label
      :for="id"
      un-block
      un-absolute
      un-top="1/2"
      un-right="0"
      un-translate-y="-1/2"
      un-p="2"
      un-cursor="inherit"
      un-pointer-events="none"
    >
      <Icon un-i="fluent-chevron-down-16-regular" />
    </label>
    <select
      :id="id"
      ref="selectRef"
      v-model="model"
      :disabled="props.disabled"
      un-p="x-2 y-1.5"
      un-w="full"
      un-text="sm"
      un-border="none"
      un-bg="transparent"
      un-outline="none"
      un-appearance="none"
      un-cursor="inherit"
      un-font="inherit"
    >
      <option
        v-for="option in props.options"
        :key="option.value"
        :value="option.value"
      >
        {{ option.label }}
      </option>
    </select>
  </div>
</template>

<script setup lang="ts" generic="T extends string | number">
import { useId } from "vue";
import CommonInput from "./CommonInput.vue";
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
  <CommonInput :disabled="props.disabled" :borderless="props.borderless">
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
  </CommonInput>
</template>

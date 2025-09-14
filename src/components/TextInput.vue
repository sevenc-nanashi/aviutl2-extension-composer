<script setup lang="ts">
import { computed, useId } from "vue";
import CommonInput from "./CommonInput.vue";
import CommonInputSideIcon from "./CommonInputSideIcon.vue";

const model = defineModel<string>({
  required: true,
});
const props = withDefaults(
  defineProps<{
    required?: boolean;
    type?: "text" | "url";
    disabled?: boolean;
  }>(),
  {
    required: false,
    type: "text",
    disabled: false,
  },
);

const id = useId();

const isError = computed(() => {
  if (props.required && !model.value) {
    return true;
  }
  if (props.type === "url" && model.value && !URL.canParse(model.value)) {
    return true;
  }
  return false;
});
</script>

<template>
  <CommonInput :error="isError">
    <CommonInputSideIcon
      v-if="props.type === 'url'"
      :for="id"
      side="left"
      un-text="slate-400"
      un-i="fluent-link-16-regular"
    />
    <input
      :id="id"
      v-model="model"
      :type="props.type"
      :disabled="props.disabled"
      un-p="x-2 y-1.5"
      :un-pl="props.type === 'url' ? '8' : '2'"
      un-text="sm"
      un-border="none"
      un-shadow="none"
      un-outline="none"
      un-rounded="md"
      un-bg="transparent"
      un-w="full"
    />
  </CommonInput>
</template>

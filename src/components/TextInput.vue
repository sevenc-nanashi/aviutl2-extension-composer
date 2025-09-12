<script setup lang="ts">
import CommonInput from "./CommonInput.vue";

const model = defineModel<string>({
  required: true,
});
const props = defineProps<{
  required?: boolean;
}>();
</script>

<template>
  <CommonInput :error="props.required && !model">
    <input
      v-model="model"
      type="text"
      un-p="x-2 y-1.5"
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

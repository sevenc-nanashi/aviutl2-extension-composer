<script setup lang="ts">
const model = defineModel({
  required: true,
});
const props = defineProps<{
  required?: boolean;
}>();
</script>

<template>
  <input
    class="text-input"
    :class="{ error: props.required && !model }"
    type="text"
    v-model="model"
    un-p="x-2 y-1.5"
    un-text="sm"
    un-border="solid 2 slate-300"
    un-shadow="inset sm slate-300/50"
    un-outline="none"
    un-rounded="md"
  />
</template>

<style scoped>
.text-input {
  &:disabled {
    cursor: not-allowed;
    --at-apply: bg-slate-200/50;
  }
  &.error {
    --at-apply: border-pink-400;
  }
}
</style>

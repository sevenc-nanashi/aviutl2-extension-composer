<script setup lang="ts">
const model = defineModel({
  required: true,
});
</script>

<template>
  <input
    class="text-input"
    type="text"
    v-model="model"
    un-p="x-2 y-1.5"
    un-text="md"
    un-border="solid 2 slate-300"
    un-shadow="inset sm slate-300/50"
    un-outline="none"
    un-rounded="md"
  />
</template>

<style scoped>
.text-input:disabled {
  cursor: not-allowed;
  --at-apply: bg-slate-200/50;
}
</style>

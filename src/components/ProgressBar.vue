<script setup lang="ts">
const props = defineProps<{
  value: number;
}>();
</script>

<template>
  <div un-w="full" un-bg="slate-200" un-rounded="full" un-overflow="hidden">
    <div
      un-bg="blue-500"
      un-h="1"
      un-rounded="full"
      un-transition="all"
      un-duration="300"
      :style="`width: ${Math.min(Math.max(props.value, 0), 100)}%`"
    />
  </div>
</template>

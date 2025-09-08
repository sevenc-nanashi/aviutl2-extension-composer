<script setup lang="ts">
const props = defineProps<{
  unI: string;
}>();
</script>
<template>
  <div :un-i="props.unI" un-text-lg />
</template>

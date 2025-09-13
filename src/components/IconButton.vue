<script setup lang="ts">
import Icon from "./Icon.vue";

defineProps<{
  unI: string;
}>();
</script>

<template>
  <button class="button square">
    <Icon :un-i="unI" />
  </button>
</template>

<style scoped></style>

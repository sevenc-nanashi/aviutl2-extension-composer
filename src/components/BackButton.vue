<script setup lang="ts">
import { RouterLink } from "vue-router";

const props = defineProps<{
  to: string;
}>();
</script>
<template>
  <RouterLink
    :to="props.to"
    class="color-transition"
    un-text="inherit hover:pink-500"
    un-no-underline
    un-ml="-2"
    un-p="l-2 r-2"
  >
    <div un-i="fluent-arrow-hook-up-left-16-regular" un-size="6" />
  </RouterLink>
</template>

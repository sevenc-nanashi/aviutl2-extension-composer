<script setup lang="ts">
import { RouterLink, useRouter } from "vue-router";

const props = defineProps<{
  to?: string;
}>();
const router = useRouter();
</script>
<template>
  <RouterLink
    v-if="props.to"
    :to="props.to"
    class="color-transition"
    un-text="inherit hover:pink-500"
    un-no-underline
    un-ml="-2"
    un-p="l-2 r-2"
  >
    <div un-i="fluent-arrow-hook-up-left-16-regular" un-size="6" />
  </RouterLink>
  <a
    v-else
    class="color-transition cursor-pointer"
    un-text="inherit hover:pink-500"
    un-no-underline
    un-ml="-2"
    un-p="l-2 r-2"
    @click="router.back()"
  >
    <div un-i="fluent-arrow-hook-up-left-16-regular" un-size="6" />
  </a>
</template>

<script setup lang="ts">
import Icon from "./Icon.vue";
const props = withDefaults(
  defineProps<{
    tag?: string;
    unI: string;
  }>(),
  {
    tag: "div",
  },
);
</script>

<template>
  <component
    :is="props.tag"
    un-flex="~ nowrap"
    un-justify="start"
    un-items="center"
    un-gap="1"
    un-overflow="hidden"
  >
    <Icon :un-i="props.unI" un-text-lg un-aspect-square />
    <slot un-w="full" />
  </component>
</template>

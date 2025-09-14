<script setup lang="ts">
import Icon from "./Icon.vue";
const props = withDefaults(
  defineProps<{
    tag?: string;
    unI: string;
    clickable?: boolean;
  }>(),
  {
    tag: "div",
    clickable: false,
  },
);
</script>

<template>
  <component
    :is="props.tag"
    un-flex="~ nowrap"
    un-justify="start"
    un-items="center"
    un-overflow="hidden"
  >
    <div :un-cursor="props.clickable ? 'pointer' : undefined" un-pr="1">
      <Icon :un-i="props.unI" un-text-lg un-box-content />
    </div>
    <slot />
  </component>
</template>

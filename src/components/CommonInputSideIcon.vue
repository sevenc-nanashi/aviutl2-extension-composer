<script setup lang="ts">
import Icon from "./Icon.vue";

const props = defineProps<{
  for: string;
  unI: string;
  side: "left" | "right";
}>();
</script>
<template>
  <label
    :for="props.for"
    un-block
    un-absolute
    un-top="1/2"
    :un-left="props.side === 'left' ? '0' : undefined"
    :un-right="props.side === 'right' ? '0' : undefined"
    un-translate-y="-1/2"
    un-p="2"
    un-cursor="inherit"
    un-pointer-events="none"
  >
    <Icon :un-i="props.unI" />
  </label>
</template>

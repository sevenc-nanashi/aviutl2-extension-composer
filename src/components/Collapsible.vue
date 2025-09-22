<script setup lang="ts">
import {
  CollapsibleContent,
  CollapsibleRoot,
  CollapsibleTrigger,
} from "reka-ui";

const props = withDefaults(
  defineProps<{
    disabled?: boolean;
  }>(),
  {
    disabled: false,
  },
);
</script>

<template>
  <CollapsibleRoot
    class="card"
    un-p="!0"
    un-w="min"
    un-h="min"
    :disabled="props.disabled"
  >
    <CollapsibleTrigger class="button" un-w="full" un-text="left">
      <slot name="trigger" />
    </CollapsibleTrigger>
    <CollapsibleContent>
      <div un-p="4">
        <slot />
      </div>
    </CollapsibleContent>
  </CollapsibleRoot>
</template>

<script setup lang="ts">
const props = defineProps<{
  label?: string;
  error?: boolean;
  disabled?: boolean;
  borderless?: boolean;
}>();
</script>

<template>
  <div
    class="common-input"
    :class="{
      error: props.error,
      disabled: props.disabled,
      borderless: props.borderless,
    }"
    un-relative
    un-rounded="md"
    un-transition-colors
    un-duration="300"
    un-outline="none"
    un-align-content="center"
  >
    <slot />
  </div>
</template>

<style scoped>
.common-input {
  --at-apply: border-solid border-2 border-slate-300 shadow-inset shadow-sm
    shadow-slate-300/50;

  &:not(.borderless):not(.disabled) {
    --at-apply: bg-white;
  }

  &.borderless {
    --at-apply: border-none shadow-none;
    &:not(.disabled) {
      --at-apply: hover: bg-blue-200/30;
    }
  }

  &:has(input:disabled),
  &.disabled {
    cursor: not-allowed;
    --at-apply: bg-slate-200 opacity-50;
  }

  &:has(select) {
    cursor: pointer;
  }

  &.error {
    --at-apply: border-pink-400;
  }
}
</style>

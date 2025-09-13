<script setup lang="ts">
import { RouterLink } from "vue-router";
import Icon from "./Icon.vue";

const props = withDefaults(
  defineProps<{
    unI: string;
    label: string;
    to?: string;
    disabled?: boolean;
    color?: "default" | "primary" | "success" | "warning" | "danger" | "error";
    type?: "button" | "submit" | "reset";
  }>(),
  {
    type: "button",
    to: undefined,
    color: "default",
  },
);

const emit = defineEmits<{ (e: "click", ev: MouseEvent): void }>();
</script>

<template>
  <!-- RouterLink when `to` is provided -->
  <RouterLink
    v-if="props.to"
    :to="props.to"
    class="button"
    :class="{ [props.color]: props.color !== 'default' }"
    v-bind="$attrs"
    un-flex
    un-items="center"
    un-gap="1"
  >
    <Icon un-text-lg :un-i="props.unI" />
    <span
      ><slot>{{ props.label }}</slot></span
    >
  </RouterLink>

  <!-- Native button otherwise -->
  <button
    v-else
    :type="props.type"
    class="button"
    :class="{ [props.color]: props.color !== 'default' }"
    :disabled="props.disabled"
    v-bind="$attrs"
    un-flex
    un-items="center"
    un-gap="1"
    @click="(ev) => emit('click', ev)"
  >
    <Icon un-text-lg :un-i="props.unI" />
    <span
      ><slot>{{ props.label }}</slot></span
    >
  </button>
</template>

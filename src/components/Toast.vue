<script setup lang="ts">
import { useToast } from "../plugins/toast.ts";
import Icon from "./Icon.vue";

const toastController = useToast();
</script>

<template>
  <div
    un-fixed
    un-inset="4"
    un-z="50"
    un-pointer-events="none"
    un-flex="~ col"
    un-gap="2"
    un-items="center"
    un-justify="end"
  >
    <TransitionGroup name="toast">
      <div
        v-for="toast in toastController.list().filter((t) => !t.closing)"
        :key="toast.id"
        class="toast"
        :class="`toast-${toast.type}`"
        un-backdrop-blur="sm"
        :un-bg="
          toast.type === 'info' ? 'blue-50/50'
          : toast.type === 'success' ? 'green-50/50'
          : toast.type === 'warning' ? 'yellow-50/50'
          : toast.type === 'danger' ? 'pink-50/50'
          : toast.type === 'error' ? 'red-50/50'
          : 'white/50'
        "
        un-rounded="md"
        un-p="2 l-4"
        un-shadow="md"
        un-flex="~ row"
        un-items="center"
        un-gap="2"
        un-pointer-events="auto"
      >
        <Icon
          v-if="toast.type"
          un-inline-block
          :un-text="
            toast.type === 'info' ? 'blue-500'
            : toast.type === 'success' ? 'green-500'
            : toast.type === 'warning' ? 'yellow-500'
            : toast.type === 'danger' ? 'pink-500'
            : toast.type === 'error' ? 'red-500'
            : ''
          "
          :un-i="
            toast.type === 'info' ? 'fluent-info-20-filled'
            : toast.type === 'success' ? 'fluent-checkmark-circle-20-filled'
            : toast.type === 'warning' ? 'fluent-warning-20-filled'
            : toast.type === 'danger' ? 'fluent-error-circle-20-filled'
            : toast.type === 'error' ? 'fluent-dismiss-circle-20-filled'
            : ''
          "
          un-mr="1"
          un-size="6"
        />
        <div>{{ toast.message }}</div>
        <button
          class="button borderless"
          un-p="!2"
          un-cursor="pointer"
          @click="toastController.close(toast.id)"
        >
          <Icon un-i="fluent-dismiss-16-regular" />
        </button>
      </div>
    </TransitionGroup>
  </div>
</template>

<style scoped>
.toast-enter-active,
.toast-leave-active {
  transition: all 0.3s ease;
}

.toast-enter-from,
.toast-leave-to {
  opacity: 0;
  transform: translateY(20px);
}
</style>

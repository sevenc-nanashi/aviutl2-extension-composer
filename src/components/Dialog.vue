<script setup lang="ts">
import {
  DialogContent,
  DialogOverlay,
  DialogPortal,
  DialogRoot,
  DialogTitle,
} from "reka-ui";
import { useAttrs } from "vue";

defineOptions({
  inheritAttrs: false,
});

const open = defineModel<boolean>("open");
const emit = defineEmits<{
  disappeared: [];
}>();
const props = withDefaults(
  defineProps<{
    allowClose?: boolean;
  }>(),
  {
    allowClose: true,
  },
);
const attrs = useAttrs();
</script>

<template>
  <DialogRoot v-model:open="open">
    <DialogPortal>
      <Transition name="fade" appear>
        <DialogOverlay
          un-z="999"
          un-bg="slate-950/10"
          un-fixed
          un-inset="0"
          un-backdrop-blur="sm"
        />
      </Transition>
      <div
        un-fixed
        un-inset="0"
        un-grid
        un-place-content="center"
        un-pointer-events="none"
        un-z="1000"
      >
        <Transition name="slide-up" appear @after-leave="emit('disappeared')">
          <DialogContent
            class="dialog-content"
            un-bg="white/80"
            un-backdrop-blur="sm"
            un-w="[clamp(300px,90vw,600px)]"
            un-rounded="md"
            un-shadow="lg"
            un-p="4"
            un-flex="~ col"
            un-gap="2"
            un-pointer-events="auto"
            v-bind="attrs"
            @interact-outside="
              props.allowClose ? $event : $event.preventDefault()
            "
          >
            <DialogTitle un-text="xl" un-flex="~ row" un-items="center">
              <slot name="title" />
            </DialogTitle>
            <slot />
            <div un-flex="~ row" un-gap="2" un-justify="end">
              <slot name="actions" />
            </div>
          </DialogContent>
        </Transition>
      </div>
    </DialogPortal>
  </DialogRoot>
</template>

<style scoped>
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.3s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}

.slide-up-enter-active,
.slide-up-leave-active {
  transition:
    transform 0.3s ease,
    opacity 0.3s ease;
}
.slide-up-enter-from,
.slide-up-leave-to {
  transform: translateY(20px);
  opacity: 0;
}
</style>

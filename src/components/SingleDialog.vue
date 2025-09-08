<script setup lang="ts">
import {
  DialogClose,
  DialogContent,
  DialogDescription,
  DialogOverlay,
  DialogPortal,
  DialogRoot,
  DialogTitle,
} from "reka-ui";
import Spacer from "./Spacer.vue";
import { DialogState } from "../lib/dialog.ts";
import { useDialog } from "../lib/dialog.ts";
import { computed } from "vue";
import Icon from "./Icon.vue";

const props = defineProps<{
  dialog: DialogState;
}>();

const dialog = useDialog();
const isOpen = computed(() => !props.dialog.closing);

const onClick = (index: number) => {
  const action = props.dialog.actions[index];
  if (action.onClick) {
    action.onClick();
  }
  dialog.close(props.dialog.id);
};
</script>

<template>
  <DialogRoot :open="isOpen">
    <DialogPortal>
      <Transition name="fade" appear>
        <DialogOverlay un-z="999" un-bg="slate-950/10" un-fixed un-inset="0" />
      </Transition>
      <div
        un-fixed
        un-inset="0"
        un-grid
        un-place-content="center"
        un-pointer-events="none"
        un-z="1000"
      >
        <Transition
          name="slide-up"
          appear
          @after-leave="dialog.remove(props.dialog.id)"
        >
          <DialogContent
            class="dialog-content"
            un-bg="white"
            un-w="[clamp(300px,90vw,600px)]"
            un-rounded="md"
            un-shadow="lg"
            un-p="4"
            un-flex="~ col"
            un-gap="2"
            un-pointer-events="auto"
          >
            <DialogTitle
              v-if="props.dialog.title"
              un-text="xl"
              un-flex="~ row"
              un-items="center"
            >
              <Icon
                v-if="props.dialog.type === 'error'"
                un-text="red-500"
                un-inline-block
                un-i="fluent-error-circle-16-filled"
                un-mr="1"
                un-size="6"
              />
              {{ props.dialog.title }}</DialogTitle
            >
            <DialogDescription> {{ props.dialog.message }}</DialogDescription>
            <Spacer un-h="2" />
            <div un-flex="~ row" un-gap="2" un-justify="end">
              <DialogClose
                v-for="(action, index) in props.dialog.actions"
                :key="index"
                @click="onClick(index)"
                class="button"
                :class="action.color"
                un-py="2"
              >
                {{ action.label }}
              </DialogClose>
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

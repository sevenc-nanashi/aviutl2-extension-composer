<script setup lang="ts">
import { DialogClose, DialogDescription } from "reka-ui";
import { computed } from "vue";
import { DialogState, useDialog } from "../plugins/dialog.ts";
import Dialog from "./Dialog.vue";
import Icon from "./Icon.vue";
import Spacer from "./Spacer.vue";

const props = defineProps<{
  dialog: DialogState & { type: "general" };
}>();

const dialogController = useDialog();
const isOpen = computed(() => !props.dialog.closing);
let answered = false;
const onClose = async () => {
  await new Promise((resolve) => setTimeout(resolve, 0));
  if (answered) return;
  answered = true;
  props.dialog.options.onDismiss?.();
  dialogController.close(props.dialog.id);
};

const onClick = (index: number) => {
  const action = props.dialog.options.actions[index];
  if (answered) return;
  answered = true;
  if (action.onClick) {
    const maybePromise = action.onClick();
    if (maybePromise instanceof Promise) {
      maybePromise.then(() => {
        dialogController.close(props.dialog.id);
      });
    } else {
      dialogController.close(props.dialog.id);
    }
  } else {
    dialogController.close(props.dialog.id);
  }
};
</script>

<template>
  <Dialog
    :open="isOpen"
    :color="props.dialog.options.color || undefined"
    :allow-close="props.dialog.options.allowDismiss !== false"
    @update:open="(v) => !v && onClose()"
    @disappeared="dialogController.remove(props.dialog.id)"
  >
    <template #title>
      <Icon
        v-if="props.dialog.options.color"
        un-inline-block
        :un-text="
          props.dialog.options.color === 'info' ? 'blue-500'
          : props.dialog.options.color === 'success' ? 'green-500'
          : props.dialog.options.color === 'warning' ? 'yellow-500'
          : props.dialog.options.color === 'danger' ? 'pink-500'
          : props.dialog.options.color === 'error' ? 'red-500'
          : ''
        "
        :un-i="
          props.dialog.options.color === 'info' ? 'fluent-info-20-filled'
          : props.dialog.options.color === 'success' ?
            'fluent-checkmark-circle-20-filled'
          : props.dialog.options.color === 'warning' ?
            'fluent-warning-20-filled'
          : props.dialog.options.color === 'danger' ?
            'fluent-error-circle-20-filled'
          : props.dialog.options.color === 'error' ?
            'fluent-dismiss-circle-20-filled'
          : ''
        "
        un-mr="1"
        un-size="6"
      />
      {{ props.dialog.options.title }}
    </template>

    <DialogDescription>
      {{ props.dialog.options.message }}
    </DialogDescription>

    <template #actions>
      <Spacer un-h="2" />
      <DialogClose
        v-for="(action, index) in props.dialog.options.actions"
        :key="index"
        class="button"
        :class="action.color"
        un-py="2"
        @click.prevent="onClick(index)"
      >
        {{ action.label }}
      </DialogClose>
    </template>
  </Dialog>
</template>

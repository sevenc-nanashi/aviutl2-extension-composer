<script setup lang="ts">
import { DialogClose, DialogDescription } from "reka-ui";
import { computed } from "vue";
import { DialogState, useDialog } from "../plugins/dialog.ts";
import Spacer from "./Spacer.vue";
import Icon from "./Icon.vue";
import Dialog from "./Dialog.vue";

const props = defineProps<{
  dialog: DialogState & { type: "general" };
}>();

const dialogController = useDialog();
const isOpen = computed(() => !props.dialog.closing);

const onClick = (index: number) => {
  const action = props.dialog.options.actions[index];
  if (action.onClick) {
    action.onClick();
  }
  dialogController.close(props.dialog.id);
};
</script>

<template>
  <Dialog
    :open="isOpen"
    @update:open="(v) => !v && dialogController.close(props.dialog.id)"
    @disappeared="dialogController.remove(props.dialog.id)"
  >
    <template #title>
      <Icon
        v-if="props.dialog.options.type"
        un-inline-block
        :un-text="
          props.dialog.options.type === 'info' ? 'blue-500'
          : props.dialog.options.type === 'success' ? 'green-500'
          : props.dialog.options.type === 'warning' ? 'yellow-500'
          : props.dialog.options.type === 'danger' ? 'pink-500'
          : props.dialog.options.type === 'error' ? 'red-500'
          : ''
        "
        :un-i="
          props.dialog.options.type === 'info' ? 'fluent-info-20-filled'
          : props.dialog.options.type === 'success' ?
            'fluent-checkmark-circle-20-filled'
          : props.dialog.options.type === 'warning' ? 'fluent-warning-20-filled'
          : props.dialog.options.type === 'danger' ?
            'fluent-error-circle-20-filled'
          : props.dialog.options.type === 'error' ?
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
        @click="onClick(index)"
      >
        {{ action.label }}
      </DialogClose>
    </template>
  </Dialog>
</template>

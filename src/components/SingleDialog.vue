<script setup lang="ts">
import { DialogClose } from "reka-ui";
import { computed } from "vue";
import { DialogState, useDialog } from "../lib/dialog.ts";
import Spacer from "./Spacer.vue";
import Icon from "./Icon.vue";
import Dialog from "./Dialog.vue";

const props = defineProps<{
  dialog: DialogState;
}>();

const dialogController = useDialog();
const isOpen = computed(() => !props.dialog.closing);

const onClick = (index: number) => {
  const action = props.dialog.actions[index];
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
        v-if="props.dialog.type === 'error'"
        un-text="red-500"
        un-inline-block
        un-i="fluent-error-circle-16-filled"
        un-mr="1"
        un-size="6"
      />
      {{ props.dialog.title }}
    </template>

    {{ props.dialog.message }}

    <template #actions>
      <Spacer un-h="2" />
      <DialogClose
        v-for="(action, index) in props.dialog.actions"
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

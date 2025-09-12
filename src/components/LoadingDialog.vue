<script setup lang="ts">
import { computed } from "vue";
import { DialogDescription } from "reka-ui";
import { DialogState, useDialog } from "../plugins/dialog.ts";
import Loading from "./Loading.vue";
import Dialog from "./Dialog.vue";

const props = defineProps<{
  dialog: DialogState & { type: "loading" };
}>();

const dialogController = useDialog();
const isOpen = computed(() => !props.dialog.closing);
</script>

<template>
  <Dialog
    :open="isOpen"
    :allow-close="false"
    un-flex
    un-w="auto"
    @update:open="(v) => !v && dialogController.close(props.dialog.id)"
    @disappeared="dialogController.remove(props.dialog.id)"
  >
    <Loading />

    <DialogDescription>
      {{ props.dialog.options.message }}
    </DialogDescription>
  </Dialog>
</template>

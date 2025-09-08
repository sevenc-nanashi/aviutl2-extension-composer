<script setup lang="ts">
import { RouterView, useRouter } from "vue-router";
import { useDialog } from "./lib/dialog.ts";
import SingleDialog from "./components/SingleDialog.vue";
import { onMounted, onUnmounted } from "vue";

const router = useRouter();
const dialogs = useDialog();

const konamiSequence = [
  "ArrowUp",
  "ArrowUp",
  "ArrowDown",
  "ArrowDown",
  "ArrowLeft",
  "ArrowRight",
  "ArrowLeft",
  "ArrowRight",
  "b",
  "a",
];
let konamiIndex = 0;
let konamiTimer: number | null = null;
const handleKeyDown = (event: KeyboardEvent) => {
  if (event.key === konamiSequence[konamiIndex]) {
    konamiIndex++;
    if (konamiIndex === konamiSequence.length) {
      router.push("/debug");
      konamiIndex = 0;
    }
    if (konamiTimer) {
      clearTimeout(konamiTimer);
    }
    konamiTimer = window.setTimeout(() => {
      konamiIndex = 0;
      konamiTimer = null;
    }, 1000);
  } else {
    konamiIndex = 0;
    if (konamiTimer) {
      clearTimeout(konamiTimer);
    }
  }
};

onMounted(() => {
  window.addEventListener("keydown", handleKeyDown);
});

onUnmounted(() => {
  window.removeEventListener("keydown", handleKeyDown);
});
</script>

<template>
  <SingleDialog
    v-for="dialog in dialogs.list()"
    :key="dialog.id"
    :dialog="dialog"
  />
  <RouterView v-slot="{ Component }">
    <Transition name="fade" mode="out-in">
      <div
        un-flex="~ col"
        un-p="2"
        un-h="screen"
        :key="router.currentRoute.value.fullPath"
      >
        <Component :is="Component" />
      </div>
    </Transition>
  </RouterView>
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
</style>

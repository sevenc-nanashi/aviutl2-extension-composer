<script setup lang="ts">
import { RouterView, useRouter } from "vue-router";

const router = useRouter();
</script>

<template>
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

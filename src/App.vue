<script setup lang="ts">
import { RouterView, useRouter } from "vue-router";
import GeneralDialog from "./components/GeneralDialog.vue";
import Header from "./components/Header.vue";
import LoadingDialog from "./components/LoadingDialog.vue";
import Toast from "./components/Toast.vue";
import { useDialog } from "./plugins/dialog.ts";

const router = useRouter();
const dialogs = useDialog();
</script>

<template>
  <Toast />
  <template v-for="dialog in dialogs.list()" :key="dialog.id">
    <GeneralDialog v-if="dialog.type === 'general'" :dialog="dialog" />
    <LoadingDialog v-else-if="dialog.type === 'loading'" :dialog="dialog" />
  </template>
  <!-- トランジションでヘッダーの線が消えるのを強引に解決 -->
  <Header
    un-absolute
    un-top="0"
    un-left="0"
    un-w="full"
    un-p="2"
    un-pointer-events="none"
  >
    &ZeroWidthSpace;
  </Header>
  <RouterView v-slot="{ Component }">
    <Transition name="fade" mode="out-in">
      <Suspense>
        <div
          :key="router.currentRoute.value.fullPath"
          un-flex="~ col"
          un-p="2"
          un-min-h="screen"
        >
          <Component :is="Component" />
        </div>
      </Suspense>
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

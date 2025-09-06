<script setup lang="ts">
import { RouterView } from "vue-router";
</script>

<template>
  <RouterView v-slot="{ Component }">
    <Transition name="fade" mode="out-in">
      <Component :is="Component" />
    </Transition>
  </RouterView>
</template>

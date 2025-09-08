<script setup lang="ts">
import Header from "../components/Header.vue";
import Footer from "../components/Footer.vue";
import Icon from "../components/Icon.vue";
import { RouterLink } from "vue-router";
</script>
<template>
  <Header>AviUtl2 Extension Composer</Header>
  <main
    un-p="2"
    un-flex="~ col"
    un-gap="2"
    un-grow
    un-w="[clamp(60vw,600px,90vw)]"
    un-mx="auto"
  >
    <p>プロファイルを選択してください。</p>
    <div class="card button">Test</div>
    <RouterLink
      to="/profiles/new"
      class="card button primary"
      un-flex
      un-items="center"
      un-gap="1"
    >
      <Icon un-text-lg un-i="fluent-add-circle-16-filled" />
      新しく作成
    </RouterLink>
  </main>
  <div un-grow />
  <Footer />
</template>

<script setup lang="ts">
import Header from "../components/Header.vue";
import BackButton from "../components/BackButton.vue";
import { useI18n } from "vue-i18n";
import { useAsync } from "../lib/asyncData.ts";
import * as ipc from "../lib/ipc.ts";

const i18n = useI18n();
const { t } = i18n;

const registries = useAsync(async () => {
  return await ipc.listRegistries();
});
</script>

<template>
  <Header>
    <BackButton />

    {{ t("title") }}
  </Header>
  <main un-grid="~ cols-2" un-gap="4" un-p="4" un-mx="auto">
    <section>
      <h2>{{ t("registries") }}</h2>

      {{ JSON.stringify(registries) }}
    </section>
    <section>
      <h2>{{ t("contents") }}</h2>
    </section>
  </main>
</template>

<i18n lang="yaml">
ja:
  title: "レジストリの管理"
  registries: "レジストリ"
  contents: "コンテンツ"
</i18n>

<script setup lang="ts">
import { useI18n } from "vue-i18n";
import Header from "../components/Header.vue";
import BackButton from "../components/BackButton.vue";
import { useAsync } from "../lib/asyncData.ts";
import * as ipc from "../lib/ipc.ts";
import ScrollArea from "../components/ScrollArea.vue";

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
  <main
    un-grid="~ cols-2"
    un-gap="4"
    un-p="4"
    un-w="full"
    un-h="max"
    un-flex-grow
  >
    <section un-flex="~ col" un-gap="2" un-h="full">
      <h2>{{ t("registries") }}</h2>

      <ScrollArea un-flex-grow>
        <Loading v-if="registries.state === 'loading'" />
        <template
          v-else-if="
            registries.state === 'success' &&
            Object.keys(registries.data).length > 0
          "
        >
          <RouterLink
            v-for="(url, id) in registries.data"
            :key="id"
            :to="`/profiles/${id}`"
            class="card button"
            un-text="inherit"
            un-flex="~ col"
            un-gap="2"
          >
            {{ url }}
          </RouterLink>
        </template>
        <p v-else-if="registries.state === 'success'">
          {{ t("noRegistries") }}
        </p>
        <p v-else-if="registries.state === 'error'" un-text="red-600">
          {{ t("failedToLoadRegistries", { error: registries.error }) }}
        </p>
      </ScrollArea>
      <hr />
      <button class="button primary" un-w="full">A</button>
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
  noRegistries: "登録されているレジストリはありません。"
</i18n>

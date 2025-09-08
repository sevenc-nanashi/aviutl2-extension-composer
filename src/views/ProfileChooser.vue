<script setup lang="ts">
import { RouterLink } from "vue-router";
import Header from "../components/Header.vue";
import Footer from "../components/Footer.vue";
import Icon from "../components/Icon.vue";
import Loading from "../components/Loading.vue";
import * as ipc from "../lib/ipc.ts";
import { onMounted, ref } from "vue";
import { useAsync } from "../lib/asyncData.ts";
import { useI18n } from "vue-i18n";

const i18n = useI18n();
const profiles = useAsync(async () => await ipc.listProfiles());
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
    <Loading v-if="profiles.state === 'loading'" />
    <div
      v-else-if="
        profiles.state === 'success' && Object.keys(profiles.data).length > 0
      "
      un-flex="~ col"
      un-gap="2"
    >
      <RouterLink
        v-for="profile in profiles.data"
        :key="profile.id"
        :to="`/profiles/${profile.id}`"
        class="card button"
        un-flex="~ row"
        un-items="center"
        un-gap="2"
      >
        {{ profile.name }}
      </RouterLink>
    </div>
    <p
      v-else-if="
        profiles.state === 'success' && Object.keys(profiles.data).length === 0
      "
      un-text="gray-600"
    >
      プロファイルがありません。新しく作成してください。
    </p>
    <p v-else-if="profiles.state === 'error'" un-text="red-600">
      プロファイルの読み込みに失敗しました。
    </p>
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

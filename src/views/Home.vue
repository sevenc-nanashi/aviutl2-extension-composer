<script setup lang="ts">
import { RouterLink } from "vue-router";
import { computed } from "vue";
import { useI18n } from "vue-i18n";
import Header from "../components/Header.vue";
import Footer from "../components/Footer.vue";
import Icon from "../components/Icon.vue";
import Loading from "../components/Loading.vue";
import Spacer from "../components/Spacer.vue";
import Select from "../components/Select.vue";
import * as ipc from "../lib/ipc.ts";
import { useAsync } from "../lib/asyncData.ts";
import { errorToString } from "../lib/error.ts";
import { i18n as globalI18n } from "../plugins/i18n.ts";

const i18n = useI18n();
const { t } = i18n;
const profiles = useAsync(async () => await ipc.listProfiles());

const locales = [
  { label: "日本語", value: "ja" },
  { label: "English", value: "en" },
];
const currentLocale = computed({
  get: () => globalI18n.global.locale.value as string,
  set: (value: string) => {
    globalI18n.global.locale.value = value;
  },
});
</script>
<template>
  <Header>
    AviUtl2 Extension Composer
    <Spacer un-flex-grow />
    <Select
      v-model="currentLocale"
      :options="locales"
      un-w="24"
      borderless
      un-mt="-1"
    />
  </Header>
  <main
    un-p="2"
    un-flex="~ col"
    un-gap="2"
    un-grow
    un-w="[clamp(60vw,600px,90vw)]"
    un-mx="auto"
  >
    <p>{{ t("selectProfile") }}</p>
    <Loading v-if="profiles.state === 'loading'" />
    <div
      v-else-if="
        profiles.state === 'success' && Object.keys(profiles.data).length > 0
      "
      un-flex="~ col"
      un-gap="2"
    >
      <RouterLink
        v-for="(profile, id) in profiles.data"
        :key="id"
        :to="`/profiles/${id}`"
        class="card button"
        un-text="inherit"
        un-flex="~ col"
        un-gap="2"
      >
        <h3>{{ profile.name }}</h3>
        <p un-font="mono" un-text="sm" un-line-height="[1]">
          {{ profile.path }}
        </p>
      </RouterLink>
    </div>
    <p
      v-else-if="
        profiles.state === 'success' && Object.keys(profiles.data).length === 0
      "
      un-text="gray-600"
    >
      {{ t("noProfiles") }}
    </p>
    <p v-else-if="profiles.state === 'error'" un-text="red-600">
      {{
        t("failedToLoadProfiles", {
          error: errorToString(profiles.error),
        })
      }}
    </p>
    <RouterLink
      to="/profiles/new"
      un-block
      class="button primary"
      un-flex
      un-items="center"
      un-gap="1"
    >
      <Icon un-text-lg un-i="fluent-add-circle-16-filled" />
      {{ t("createNewProfile") }}
    </RouterLink>

    <hr />

    <RouterLink
      to="/registries"
      un-block
      class="button"
      un-flex
      un-items="center"
      un-gap="1"
    >
      <Icon un-text-lg un-i="fluent-database-16-regular" />
      {{ t("manageRegistry") }}
    </RouterLink>
  </main>
  <div un-grow />
  <Footer />
</template>

<i18n lang="yaml">
ja:
  selectProfile: "プロファイルを選択してください。"
  noProfiles: "プロファイルがありません。新しく作成してください。"
  failedToLoadProfiles: "プロファイルの読み込みに失敗しました：{error}"
  createNewProfile: "新しく作成"
  manageRegistry: "レジストリの管理"

en:
  selectProfile: "Select a profile."
  noProfiles: "No profiles found. Please create a new one."
  failedToLoadProfiles: "Failed to load profiles: {error}"
  createNewProfile: "Create New Profile"
  manageRegistry: "Manage Registry"
</i18n>

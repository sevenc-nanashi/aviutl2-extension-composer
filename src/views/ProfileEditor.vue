<script setup lang="ts">
import { useI18n } from "vue-i18n";
import { useRouter } from "vue-router";
import BackButton from "../components/BackButton.vue";
import Header from "../components/Header.vue";
import Loading from "../components/Loading.vue";
import { errorToLocalizedString } from "../lib/error.ts";
import * as ipc from "../lib/ipc.ts";
import { useAsync } from "../lib/useAsync.ts";

const router = useRouter();
const profileId = router.currentRoute.value.params.id as string;
const i18n = useI18n();
const { t } = i18n;
const profile = useAsync(async () => await ipc.getProfileStore(profileId));
</script>

<template>
  <template v-if="profile.state === 'loading'">
    <Header>
      <BackButton to="/" />

      {{ t("loading") }}
    </Header>
    <main un-grid un-place-items-center un-flex-1>
      <div un-text-center>
        <Loading un-inline-block un-mt-4 />
      </div>
    </main>
  </template>
  <template v-else-if="profile.state === 'error'">
    <Header>
      <BackButton to="/" />

      {{ t("failedToLoad") }}
    </Header>
    <main un-w="main" un-mx="auto">
      <p un-text="red-600" un-mt-4>
        {{ errorToLocalizedString(t, profile.error) }}
      </p>
    </main>
  </template>
  <template v-else-if="profile.state === 'success'">
    <Header>
      <BackButton to="/" />

      {{ t("title", { name: profile.data.name }) }}
    </Header>
  </template>
</template>

<i18n lang="yaml">
ja:
  title: "プロファイル：{name}"
  failedToLoad: "プロファイルの読み込みに失敗しました"
</i18n>

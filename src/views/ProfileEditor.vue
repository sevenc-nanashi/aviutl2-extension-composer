<script setup lang="ts">
import { useI18n } from "vue-i18n";
import BackButton from "../components/BackButton.vue";
import Header from "../components/Header.vue";

const i18n = useI18n();
const { t } = i18n;
</script>

<template>
  <Header>
    <BackButton />

    {{ t("title", { name: "TODO" }) }}
  </Header>
</template>

<i18n lang="yaml">
ja:
  title: "プロファイル：{name}"
</i18n>

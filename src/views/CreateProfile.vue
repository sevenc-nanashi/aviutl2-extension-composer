<script setup lang="ts">
import Header from "../components/Header.vue";
import Spacer from "../components/Spacer.vue";
import Icon from "../components/Icon.vue";
import { RouterLink, useRouter } from "vue-router";
import { ref } from "vue";
import FilePathInput from "../components/FilePathInput.vue";
import { useDialog } from "../lib/dialog.ts";

const profilePath = ref("");
const router = useRouter();
const dialog = useDialog();

const createProfile = () => {
  dialog.open({
    title: "エラー",
    message: "プロファイルのパスを指定してください。",
    type: "error",
    actions: [{ label: "OK", onClick: () => undefined }],
  });
};
</script>
<template>
  <Header>
    <RouterLink
      to="/profiles"
      class="color-transition"
      un-text="inherit hover:pink-500"
      un-no-underline
      un-ml="-2"
      un-p="l-2 r-2"
    >
      <div un-i="fluent-arrow-hook-up-left-16-regular" un-size="6" />
    </RouterLink>
    新しくプロファイルを作成
  </Header>
  <main
    un-p="2"
    un-flex="~ col"
    un-gap="2"
    un-grow
    un-w="[clamp(60vw,600px,90vw)]"
    un-mx="auto"
  >
    <p>プロファイルを作成します。</p>
    <Spacer un-h="2" />
    <h2 un-text="xl">プロファイルのパス</h2>
    <p>
      AviUtl2の環境設定のディレクトリを指定してください。<br />
      基本的には<code>C:\ProgramData\aviutl2</code>ですが、<code>data</code>ディレクトリを指定することもできます。
    </p>
    <FilePathInput
      v-model="profilePath"
      default-path="C:\ProgramData\aviutl2"
      type="open"
      directory
      placeholder="C:\ProgramData\aviutl2"
      must-exist
      un-w="full"
    />
    <Spacer un-h="4" />
    <button
      to="/profiles/new/configure"
      class="button primary"
      un-flex
      un-items="center"
      un-gap="1"
      :disabled="!profilePath"
      @click="createProfile()"
    >
      <Icon un-text-lg un-i="fluent-checkmark-circle-16-filled" />
      完了
    </button>
  </main>
</template>

<script setup lang="ts">
import Header from "../components/Header.vue";
import Spacer from "../components/Spacer.vue";
import Icon from "../components/Icon.vue";
import { useRouter } from "vue-router";
import { ref } from "vue";
import FilePathInput from "../components/FilePathInput.vue";
import { useDialog } from "../lib/dialog.ts";
import * as ipc from "../lib/ipc.ts";
import TextInput from "../components/TextInput.vue";
import BackButton from "../components/BackButton.vue";
import { errorToString } from "../lib/error.ts";

const router = useRouter();
const profileName = ref("");
const profilePath = ref("");
const dialog = useDialog();

const createProfile = async () => {
  try {
    if (!profileName.value) {
      throw new Error("プロファイルの名前を入力してください。");
    }
    if (!profilePath.value) {
      throw new Error("環境設定のフォルダを指定してください。");
    }
    const id = await ipc.initializeProfile(
      profileName.value,
      profilePath.value,
    );
    router.push(`/profiles/${id}`);
  } catch (error) {
    dialog.open({
      title: "エラー",
      message: errorToString(error),
      type: "error",
      actions: [{ label: "OK" }],
    });
    return;
  }
};
</script>
<template>
  <Header>
    <BackButton to="/profiles" />
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

    <h2 un-text="xl">プロファイルの名前</h2>
    <p>プロファイルの名前を入力してください。</p>
    <TextInput v-model="profileName" required type="open" un-w="full" />

    <h2 un-text="xl">環境設定のフォルダ</h2>
    <p>
      AviUtl2の環境設定が保存されているフォルダを指定してください。
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
      :disabled="!profileName || !profilePath"
      @click="createProfile()"
    >
      <Icon un-text-lg un-i="fluent-checkmark-circle-16-filled" />
      完了
    </button>
  </main>
</template>

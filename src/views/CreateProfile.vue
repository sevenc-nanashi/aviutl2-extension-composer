<script setup lang="ts">
import { ref } from "vue";
import { useI18n } from "vue-i18n";
import { useRouter } from "vue-router";
import BackButton from "../components/BackButton.vue";
import FilePathInput from "../components/FilePathInput.vue";
import Header from "../components/Header.vue";
import IconLabelButton from "../components/IconLabelButton.vue";
import Markdown from "../components/Markdown.vue";
import Spacer from "../components/Spacer.vue";
import TextInput from "../components/TextInput.vue";
import { errorToLocalizedString } from "../lib/error.ts";
import * as ipc from "../lib/ipc.ts";
import { useDialog } from "../plugins/dialog.ts";

const router = useRouter();
const profileName = ref("");
const profilePath = ref("");
const dialog = useDialog();

const i18n = useI18n();
const { t } = i18n;

const createProfile = async (onExist: ipc.InitializeOnExist = "abort") => {
  try {
    if (!profileName.value) {
      throw new Error("プロファイルの名前を入力してください。");
    }
    if (!profilePath.value) {
      throw new Error("環境設定のフォルダを指定してください。");
    }
    const id = await ipc.initializeProfile({
      name: profileName.value,
      path: profilePath.value,
      onExist,
    });
    router.push(`/profiles/${id}`);
  } catch (error) {
    if (error === "#already_exists") {
      dialog.open({
        title: t("onExist.title"),
        message: t("onExist.description"),
        type: "warning",
        actions: [
          { label: t("onExist.options.abort") },
          {
            label: t("onExist.options.reuse_existing"),
            color: "warning",
            onClick: () => {
              createProfile("reuse_existing");
            },
          },
          {
            label: t("onExist.options.remove_existing"),
            color: "danger",
            onClick: () => {
              createProfile("remove_existing");
            },
          },
        ],
      });
      return;
    }
    dialog.open({
      title: t("error"),
      message: errorToLocalizedString(t, error),
      type: "error",
      actions: [{ label: t("ok") }],
    });
    return;
  }
};
</script>
<template>
  <Header>
    <BackButton />
    {{ t("title") }}
  </Header>
  <main un-p="2" un-flex="~ col" un-gap="2" un-grow un-w="main" un-mx="auto">
    <p>{{ t("description") }}</p>

    <Spacer un-h="2" />

    <h2 un-text="xl">{{ t("name.title") }}</h2>
    <Markdown :content="t('name.placeholder')" />
    <TextInput v-model="profileName" required un-w="full" />

    <h2 un-text="xl">{{ t("path.title") }}</h2>
    <Markdown :content="t('path.description')" />
    <FilePathInput
      v-model="profilePath"
      default-path="C:\ProgramData\aviutl2"
      type="open"
      directory
      :placeholder="t('path.placeholder')"
      must-exist
      un-w="full"
    />

    <Spacer un-h="4" />

    <Spacer un-h="4" />

    <IconLabelButton
      color="primary"
      :disabled="!profileName || !profilePath"
      un-i="fluent-checkmark-circle-16-regular"
      :label="t('done')"
      @click="createProfile()"
    />
  </main>
</template>

<i18n lang="yaml">
ja:
  title: プロファイルの作成
  description: プロファイルを新しく作成します。

  name:
    title: プロファイルの名前
    placeholder: プロファイルの名前を入力してください。

  path:
    title: 環境設定のフォルダ
    description: |
      AviUtl2の環境設定が保存されているフォルダを指定してください。
      基本的には`C:\ProgramData\aviutl2`ですが、`data`ディレクトリを指定することもできます。
    placeholder: C:\ProgramData\aviutl2

  done: 作成

  onExist:
    title: 既存の設定が存在します
    description: |
      指定されたフォルダには既にAviUtl2 Extension Composerの設定が存在します。
      どのように処理しますか？
    options:
      reuse_existing: 既存の設定を再利用する
      remove_existing: 既存の設定を削除して新規作成する
      abort: 中止する

  errors:
    name_empty: プロファイルの名前を入力してください。
    not_exists: 環境設定のフォルダが存在しません。
    not_directory: 環境設定のフォルダがディレクトリではありません。
    already_initialized: 指定されたフォルダには既にAviUtl2 Extension Composerの環境設定が登録されています。
    already_exists: 指定されたフォルダには既に設定が存在します。
</i18n>

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
import { errorToLocalizedString, errorToString } from "../lib/error.ts";
import Markdown from "../components/Markdown.vue";
import { useI18n } from "vue-i18n";

const router = useRouter();
const profileName = ref("");
const profilePath = ref("");
const dialog = useDialog();

const i18n = useI18n();
const { t } = i18n;

const createProfile = async (options: { reinit?: boolean } = {}) => {
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
      reinit: options.reinit || false,
    });
    router.push(`/profiles/${id}`);
  } catch (error) {
    if (error === "#reinit_required") {
      dialog.open({
        title: t("reinit.title"),
        message: t("reinit.message"),
        type: "warning",
        actions: [
          { label: t("no") },
          { label: t("yes"), onClick: () => createProfile({ reinit: true }) },
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
    <BackButton to="/profiles" />
    {{ t("title") }}
  </Header>
  <main
    un-p="2"
    un-flex="~ col"
    un-gap="2"
    un-grow
    un-w="[clamp(60vw,600px,90vw)]"
    un-mx="auto"
  >
    <p>{{ t("description") }}</p>

    <Spacer un-h="2" />

    <h2 un-text="xl">{{ t("name.title") }}</h2>
    <Markdown :content="t('name.placeholder')" />
    <TextInput v-model="profileName" required type="open" un-w="full" />

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
      {{ t("done") }}
    </button>
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

  reinit:
    title: 環境設定の初期化
    message: |
      指定されたフォルダには既にAviUtl2 Extension Composerの環境設定が存在します。
      環境設定を初期化して新しく作成しますか？

  errors:
    name_empty: プロファイルの名前を入力してください。
    not_exists: 環境設定のフォルダが存在しません。
    not_directory: 環境設定のフォルダがディレクトリではありません。
    already_initialized: 指定されたフォルダには既にAviUtl2 Extension Composerの環境設定が存在します。

en:
  title: Create Profile
  description: Create a new profile.

  name:
    title: Profile Name
    placeholder: Please enter the profile name.

  path:
    title: Configuration Folder
    description: |
      Specify the folder where AviUtl2's configuration is saved.
      It is usually `C:\ProgramData\aviutl2`, but you can also specify the `data` directory.
    placeholder: C:\ProgramData\aviutl2

  done: Create

  reinit:
    title: Reinitialize Configuration
    message: |
      The specified folder already contains AviUtl2 Extension Composer configuration.
      Do you want to reinitialize the configuration and create a new one?

  errors:
    name_empty: Please enter the profile name.
    not_exists: The configuration folder does not exist.
    not_directory: The configuration folder is not a directory.
    already_initialized: The specified folder already contains AviUtl2 Extension Composer configuration.
</i18n>

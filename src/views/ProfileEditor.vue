<script setup lang="ts">
import { useI18n } from "vue-i18n";
import { useRouter } from "vue-router";
import BackButton from "../components/BackButton.vue";
import ContentCard from "../components/ContentCard.vue";
import Header from "../components/Header.vue";
import IconLabelButton from "../components/IconLabelButton.vue";
import Loading from "../components/Loading.vue";
import ScrollArea from "../components/ScrollArea.vue";
import { errorToLocalizedString } from "../lib/error.ts";
import * as ipc from "../lib/ipc.ts";
import { useAsync } from "../lib/useAsync.ts";
import { useRegistry } from "../lib/useRegistry.ts";
import { useDialog } from "../plugins/dialog.ts";
import { useToast } from "../plugins/toast.ts";
import IconButton from "../components/IconButton.vue";

const router = useRouter();
const toast = useToast();
const dialog = useDialog();
const i18n = useI18n();
const { registries, contents } = useRegistry();

const { t } = i18n;

const profileId = router.currentRoute.value.params.id as string;
const profile = useAsync(async () => await ipc.getProfileStore(profileId));

const unregisterProfile = async () => {
  const ret = await dialog.ask({
    title: t("unregister.title"),
    message: t("unregister.description"),
    type: "warning",
    actions: [
      { label: t("cancel"), value: false },
      { label: t("ok"), value: true, color: "danger" },
    ],
  });
  if (!ret) return;

  try {
    using _context = dialog.loading(t("unregister.loading"));
    await ipc.unregisterProfile(profileId);
    toast.open({ message: t("unregister.success"), type: "success" });
    router.push("/");
  } catch (error) {
    dialog.open({
      title: t("error"),
      message: errorToLocalizedString(t, error),
      type: "error",
      actions: [{ label: t("ok") }],
    });
  }
};

const removeProfile = async () => {
  const ret = await dialog.ask({
    title: t("remove.title"),
    message: t("remove.description"),
    type: "danger",
    actions: [
      { label: t("cancel"), value: false },
      { label: t("ok"), value: true, color: "danger" },
    ],
  });
  if (!ret) return;

  try {
    using _context = dialog.loading(t("remove.loading"));
    await ipc.removeProfile(profileId);
    toast.open({ message: t("remove.success"), type: "success" });
    router.push("/");
  } catch (error) {
    dialog.open({
      title: t("error"),
      message: errorToLocalizedString(t, error),
      type: "error",
      actions: [{ label: t("ok") }],
    });
  }
};

const openProfileFolder = async () => {
  await ipc.openProfileFolder(profileId);
};
</script>

<template>
  <Header>
    <BackButton to="/" />

    <template v-if="profile.state === 'loading'">
      {{ t("loading") }}
    </template>
    <template v-else-if="profile.state === 'error'">
      {{ t("failedToLoad") }}
    </template>
    <template v-else-if="profile.state === 'success'">
      {{ t("title", { name: profile.data.name }) }}
    </template>
  </Header>

  <main
    un-w="main"
    un-mx="auto"
    un-mt-4
    un-flex="~ col"
    un-gap="4"
    un-flex-grow
  >
    <template v-if="profile.state === 'loading'">
      <div un-text-center>
        <Loading un-inline-block un-mt-4 />
      </div>
    </template>
    <template v-else-if="profile.state === 'error'">
      <p un-text="red-600" un-mt-4>
        {{ errorToLocalizedString(t, profile.error) }}
      </p>
    </template>

    <div un-flex un-gap="2">
      <IconLabelButton
        :label="t('openFolder')"
        un-i="fluent-folder-open-20-filled"
        @click="openProfileFolder"
      />
      <IconLabelButton
        :label="t('unregister.label')"
        un-i="fluent-dismiss-20-filled"
        color="warning"
        @click="unregisterProfile"
      />
      <IconLabelButton
        :label="t('remove.label')"
        un-i="fluent-delete-20-filled"
        color="danger"
        @click="removeProfile"
      />
    </div>

    <div
      v-if="profile.state === 'success'"
      un-flex-grow
      un-w="main"
      un-mx="auto"
      un-grid="~ cols-2"
      un-gap="4"
    >
      <section un-flex="~ col" un-gap="2" un-h="full">
        <h2>{{ t("installedContents.title") }}</h2>
        <p>
          {{ t("installedContents.description") }}
        </p>

        <ScrollArea un-flex-grow>
          <Loading v-if="registries.value.state === 'loading'" />
          <template
            v-else-if="
              registries.value.state === 'success' && contents.size > 0
            "
          >
            <ContentCard
              v-for="content in Array.from(contents.values())"
              :key="content.id"
              :content="content"
            >
              <div un-flex un-justify="end">
                <IconButton
                  class="warning"
                  un-i="fluent-dismiss-circle-16-filled"
                />
              </div>
            </ContentCard>
          </template>
          <p
            v-else-if="registries.value.state === 'success'"
            un-text="sm slate-500"
          >
            {{ t("noContents") }}
          </p>
          <p v-else-if="registries.value.state === 'error'" un-text="red-600">
            {{
              t("failedToLoadContents", {
                error: errorToLocalizedString(t, registries.value.error),
              })
            }}
          </p>
        </ScrollArea>
      </section>
      <section un-flex="~ col" un-gap="2" un-h="full">
        <h2>{{ t("availableContents.title") }}</h2>
        <p>
          {{ t("availableContents.description") }}
        </p>

        <ScrollArea un-flex-grow>
          <Loading v-if="registries.value.state === 'loading'" />
          <template
            v-else-if="
              registries.value.state === 'success' && contents.size > 0
            "
          >
            <ContentCard
              v-for="content in Array.from(contents.values())"
              :key="content.id"
              :content="content"
            >
              <div un-flex un-justify="end">
                <IconButton
                  class="primary"
                  un-i="fluent-add-circle-16-filled"
                />
              </div>
            </ContentCard>
          </template>
          <p
            v-else-if="registries.value.state === 'success'"
            un-text="sm slate-500"
          >
            {{ t("noContents") }}
          </p>
          <p v-else-if="registries.value.state === 'error'" un-text="red-600">
            {{
              t("failedToLoadContents", {
                error: errorToLocalizedString(t, registries.value.error),
              })
            }}
          </p>
        </ScrollArea>
      </section>
    </div>
  </main>
</template>

<i18n lang="yaml">
ja:
  title: "プロファイル：{name}"
  failedToLoad: "プロファイルの読み込みに失敗しました"
  openFolder: "フォルダを開く"
  unregister:
    label: "登録解除"
    title: "登録解除"
    description: |
      本当にこのプロファイルの登録を解除しますか？
      後で再度登録することも可能です。
    loading: "プロファイルの登録を解除しています..."
    success: "プロファイルの登録を解除しました。"
  remove:
    label: "削除"
    title: "プロファイルの削除"
    description: |
      本当にこのプロファイルを削除しますか？
      この操作は元に戻せません。
    loading: "プロファイルを削除しています..."
    success: "プロファイルを削除しました。"
  installedContents:
    title: "インストール済み"
    description: "このプロファイルにインストールされているコンテンツの一覧です。"
  availableContents:
    title: "利用可能"
    description: "レジストリに登録されているコンテンツの一覧です。"
  noContents: "登録されているレジストリがありません。"
  failedToLoadContents: "ユーザーコンテンツの読み込みに失敗しました：{error}"
</i18n>

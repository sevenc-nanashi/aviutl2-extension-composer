<script setup lang="ts">
import { useI18n } from "vue-i18n";
import { computed, ref } from "vue";
import { DialogDescription } from "reka-ui";
import Header from "../components/Header.vue";
import Dialog from "../components/Dialog.vue";
import Loading from "../components/Loading.vue";
import Icon from "../components/Icon.vue";
import BackButton from "../components/BackButton.vue";
import ScrollArea from "../components/ScrollArea.vue";
import TextInput from "../components/TextInput.vue";
import { useRefreshableAsync } from "../lib/asyncData.ts";
import * as ipc from "../lib/ipc.ts";
import { errorToLocalizedString } from "../lib/error.ts";
import { useDialog } from "../plugins/dialog.ts";
import RegistryCard from "../components/RegistryCard.vue";

const i18n = useI18n();
const { t } = i18n;

const dialog = useDialog();

const registries = useRefreshableAsync(async () => {
  return await ipc.listRegistries();
});

const showAddRegistryDialog = ref(false);
const newRegistryUrl = ref("");

const showAddRegistry = () => {
  showAddRegistryDialog.value = true;
  newRegistryUrl.value = "";
};

const isValid = computed(
  () =>
    newRegistryUrl.value &&
    newRegistryUrl.value.startsWith("http") &&
    URL.canParse(newRegistryUrl.value) &&
    (registries.value.state === "success" ?
      !Object.values(registries.value.data).includes(newRegistryUrl.value)
    : false),
);

const addRegistry = async () => {
  if (!isValid.value) return;
  try {
    await ipc.addRegistry(newRegistryUrl.value);
    void registries.refresh();
    showAddRegistryDialog.value = false;
  } catch (error) {
    dialog.open({
      title: t("error"),
      message: errorToLocalizedString(t, error),
      type: "error",
      actions: [{ label: t("ok") }],
    });
  }
};
</script>

<template>
  <Dialog v-model:open="showAddRegistryDialog">
    <template #title>
      {{ t("addRegistry") }}
    </template>
    <DialogDescription>
      {{ t("addRegistryDescription") }}
    </DialogDescription>
    <TextInput v-model="newRegistryUrl" type="url" un-w="full" required />

    <template #actions>
      <button class="button" @click="showAddRegistryDialog = false">
        {{ t("cancel") }}
      </button>
      <button
        class="button primary"
        :disabled="!isValid"
        un-flex
        un-items="center"
        un-gap="1"
        @click="addRegistry"
      >
        <Icon un-text-lg un-i="fluent-add-circle-16-filled" />
        {{ t("add") }}
      </button>
    </template>
  </Dialog>
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
        <Loading v-if="registries.value.state === 'loading'" />
        <template
          v-else-if="
            registries.value.state === 'success' &&
            Object.keys(registries.value.data).length > 0
          "
        >
          <RegistryCard
            v-for="(_url, id) in registries.value.data"
            :key="id"
            :registry="id"
          />
        </template>
        <p v-else-if="registries.value.state === 'success'">
          {{ t("noRegistries") }}
        </p>
        <p v-else-if="registries.value.state === 'error'" un-text="red-600">
          {{ t("failedToLoadRegistries", { error: registries.value.error }) }}
        </p>
      </ScrollArea>
      <hr />
      <button
        class="button primary"
        un-w="full"
        un-block
        un-flex
        un-items="center"
        un-gap="1"
        @click="showAddRegistry"
      >
        <Icon un-text-lg un-i="fluent-add-circle-16-filled" />
        {{ t("addRegistry") }}
      </button>
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
  addRegistry: "レジストリを追加"
  addRegistryDescription: "追加するレジストリのURLを入力してください。"

  add: "追加"
</i18n>

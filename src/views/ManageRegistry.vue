<script setup lang="ts">
import { DialogDescription } from "reka-ui";
import { computed, ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import BackButton from "../components/BackButton.vue";
import ContentCard from "../components/ContentCard.vue";
import Dialog from "../components/Dialog.vue";
import Header from "../components/Header.vue";
import Icon from "../components/Icon.vue";
import IconLabelButton from "../components/IconLabelButton.vue";
import Loading from "../components/Loading.vue";
import RegistryCard from "../components/RegistryCard.vue";
import ScrollArea from "../components/ScrollArea.vue";
import TextInput from "../components/TextInput.vue";
import { errorToLocalizedString } from "../lib/error.ts";
import * as ipc from "../lib/ipc.ts";
import { Registry } from "../lib/models/Registry";
import { useRefreshableAsync } from "../lib/useAsync.ts";
import { compareVersions } from "../lib/version.ts";
import { useDialog } from "../plugins/dialog.ts";
import IconButton from "../components/IconButton.vue";

const i18n = useI18n();
const { t } = i18n;

const dialog = useDialog();

const registryPromises = new Map<string, Promise<Registry>>();
const registryValues = ref(new Map<string, Registry>());
const registries = useRefreshableAsync(async () => {
  return await ipc.listRegistries();
});
watch(
  () => registries.value,
  async () => {
    const registryData =
      registries.value.state === "success" ? registries.value.data : {};
    registryValues.value.clear();
    for (const [id, url] of Object.entries(registryData)) {
      if (!registryPromises.has(id)) {
        const promise = ipc.fetchRegistryCached(url).then((data) => {
          registryValues.value.set(id, data);
          return data;
        });
        registryPromises.set(id, promise);
      }
    }
  },
);

const contents = computed(() => {
  if (registries.value.state !== "success") return [];
  const contents = new Map<string, Registry["contents"][0]>();
  for (const registry of registryValues.value.values()) {
    for (const content of registry.contents) {
      const existing = contents.get(content.id);
      if (!existing || compareVersions(existing, content) === -1) {
        contents.set(content.id, content);
      }
    }
  }
  return Array.from(contents.values()).toSorted();
});

const hoveredRegistry = ref<string | null>(null);
const hoveredContent = ref<string | null>(null);
const hoveredContentRegistry = computed(() => {
  if (!hoveredContent.value) return null;
  return contentToRegistry(hoveredContent.value);
});
const contentToRegistry = (contentId: string) => {
  const content = contents.value.find((c) => c.id === contentId);
  if (!content) return null;
  for (const [registryId, registry] of registryValues.value.entries()) {
    if (
      registry.contents.some(
        (c) =>
          c.id === contentId &&
          c.version === content.version &&
          c.version_number === content.version_number,
      )
    ) {
      return registryId;
    }
  }
  return null;
};

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
    using _context = dialog.loading(t("addingRegistry"));
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

const removeRegistry = async (id: string) => {
  const { promise, resolve } = Promise.withResolvers<boolean>();
  dialog.open({
    title: t("removeRegistry"),
    message: t("removeRegistryDescription"),
    type: "warning",
    allowDismiss: false,
    actions: [
      { label: t("cancel"), onClick: () => resolve(false) },
      { label: t("remove"), color: "danger", onClick: () => resolve(true) },
    ],
  });
  if (!(await promise)) return;

  try {
    using _context = dialog.loading(t("removingRegistry"));
    await ipc.removeRegistry(id);
    void registries.refresh();
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
      <IconLabelButton
        color="primary"
        :disabled="!isValid"
        un-i="fluent-add-circle-16-filled"
        :label="t('add')"
        @click="addRegistry"
      />
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
    un-w="main"
    un-mx="auto"
    un-h="max"
    un-flex-grow
  >
    <section un-flex="~ col" un-gap="2" un-h="full">
      <h2>{{ t("registries") }}</h2>
      <p>
        {{ t("registriesDescription") }}
      </p>

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
            :class="{
              primary:
                hoveredRegistry === id ||
                (hoveredContentRegistry && hoveredContentRegistry === id),
            }"
            :registry="id"
            @mouseover="hoveredRegistry = id"
            @mouseleave="hoveredRegistry = null"
            @remove="removeRegistry"
          >
            <div un-flex un-justify="end">
              <IconButton
                class="danger"
                un-i="fluent-delete-16-regular"
                @click="removeRegistry(id)"
              />
            </div>
          </RegistryCard>
        </template>
        <p
          v-else-if="registries.value.state === 'success'"
          un-text="sm slate-500"
        >
          {{ t("noRegistries") }}
        </p>
        <p v-else-if="registries.value.state === 'error'" un-text="red-600">
          {{ t("failedToLoadRegistries", { error: registries.value.error }) }}
        </p>
      </ScrollArea>
      <hr />
      <IconLabelButton
        color="primary"
        un-w="full"
        un-block
        :label="t('addRegistry')"
        un-i="fluent-add-circle-16-filled"
        @click="showAddRegistry"
      />
    </section>
    <section un-flex="~ col" un-gap="2" un-h="full">
      <h2>{{ t("contents") }}</h2>
      <p>
        {{ t("contentsDescription") }}
      </p>
      <ScrollArea un-flex-grow>
        <Loading v-if="registries.value.state === 'loading'" />
        <template
          v-else-if="
            registries.value.state === 'success' && contents.length > 0
          "
        >
          <ContentCard
            v-for="content in contents"
            :key="content.id"
            :content="content"
            :class="{
              primary:
                (hoveredRegistry &&
                  contentToRegistry(content.id) === hoveredRegistry) ||
                hoveredContent === content.id,
            }"
            @mouseover="hoveredContent = content.id"
            @mouseleave="hoveredContent = null"
          />
        </template>
        <p
          v-else-if="
            registries.value.state === 'success' && contents.length === 0
          "
          un-text="sm slate-500"
        >
          {{ t("noContents") }}
        </p>
      </ScrollArea>
    </section>
  </main>
</template>

<i18n lang="yaml">
ja:
  title: "レジストリの管理"
  registries: "レジストリ"
  registriesDescription: "AviUtl2のプラグインやスクリプトを配布しているレジストリを管理します。"
  contents: "ユーザーコンテンツ"
  contentsDescription: "レジストリに登録されているプラグインやスクリプトを閲覧できます。"
  noRegistries: "登録されているレジストリはありません。"
  noContents: "登録されているユーザーコンテンツはありません。"
  addRegistry: "レジストリを追加"
  addRegistryDescription: "追加するレジストリのURLを入力してください。"
  addingRegistry: "レジストリを追加しています..."
  removeRegistry: "レジストリを削除"
  removeRegistryDescription: "このレジストリを削除しますか？この操作は元に戻せません。"
  removingRegistry: "レジストリを削除しています..."

  add: "追加"
  remove: "削除"

  errors:
    invalid_as_registry: "このURLはレジストリとして有効ではありません。"
</i18n>

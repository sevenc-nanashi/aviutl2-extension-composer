<script setup lang="ts">
import { DialogDescription } from "reka-ui";
import { computed, ref } from "vue";
import { useI18n } from "vue-i18n";
import BackButton from "../components/BackButton.vue";
import ContentCard from "../components/ContentCard.vue";
import Dialog from "../components/Dialog.vue";
import FileInput from "../components/FileInput.vue";
import Header from "../components/Header.vue";
import IconButton from "../components/IconButton.vue";
import IconLabelButton from "../components/IconLabelButton.vue";
import Loading from "../components/Loading.vue";
import RegistryCard from "../components/RegistryCard.vue";
import ScrollArea from "../components/ScrollArea.vue";
import TextInput from "../components/TextInput.vue";
import { errorToLocalizedString, UnreachableError } from "../lib/error.ts";
import { fileToUint8Array } from "../lib/file.ts";
import * as ipc from "../lib/ipc.ts";
import { localRegistry, useRegistry } from "../lib/useRegistry.ts";
import { useDialog } from "../plugins/dialog.ts";

const i18n = useI18n();
const { t } = i18n;
const dialog = useDialog();
const {
  registries,
  contents,
  localManifests,
  localManifestValues,
  contentToRegistry,
  contentsLoaded,
  errors,
} = useRegistry();

const showAddManifestDialog = ref(false);
const addManifestType = ref<"url" | "file">("url");
const newManifestUrl = ref("");
const newManifestFile = ref<File | null>(null);

const showAddManifest = () => {
  showAddManifestDialog.value = true;
  newManifestUrl.value = "";
  newManifestFile.value = null;
};

const isManifestUrlValid = computed(
  () => newManifestUrl.value && URL.canParse(newManifestUrl.value),
);
const isManifestValid = computed(
  () =>
    (addManifestType.value === "url" && isManifestUrlValid.value) ||
    (addManifestType.value === "file" && newManifestFile.value !== null),
);

const addManifest = async () => {
  try {
    using _context = dialog.loading(t("addManifest.loading"));
    if (newManifestFile.value) {
      await ipc.addManifestLocal(await fileToUint8Array(newManifestFile.value));
    } else if (isManifestUrlValid.value) {
      await ipc.addManifestUrl(newManifestUrl.value);
    } else {
      throw new UnreachableError();
    }
    void registries.refresh();
    showAddManifestDialog.value = false;
  } catch (error) {
    dialog.open({
      title: t("error"),
      message: errorToLocalizedString(t, error),
      color: "error",
      actions: [{ label: t("ok") }],
    });
  }
};

const removeLocalManifest = async (id: string) => {
  const manifestUuid = [...localManifestValues.value.entries()].find(
    ([, value]) => value.id === id,
  )?.[0];
  if (!manifestUuid) throw new UnreachableError();

  const response = await dialog.ask({
    title: t("removeManifest.title"),
    message: t("removeManifest.description"),
    color: "warning",
    actions: [
      { label: t("cancel"), value: false },
      { label: t("remove"), color: "warning", value: true },
    ],
  });
  if (!response) return;

  try {
    using _context = dialog.loading(t("removeManifest.loading"));
    await ipc.removeManifest(manifestUuid);
    void localManifests.refresh();
  } catch (error) {
    dialog.open({
      title: t("error"),
      message: errorToLocalizedString(t, error),
      color: "error",
      actions: [{ label: t("ok") }],
    });
  }
};

const hoveredRegistry = ref<string | null>(null);
const hoveredContent = ref<string | null>(null);
const hoveredContentRegistry = computed(() => {
  if (!hoveredContent.value) return null;
  return contentToRegistry(hoveredContent.value);
});

const showAddRegistryDialog = ref(false);
const newRegistryUrl = ref("");

const showAddRegistry = () => {
  showAddRegistryDialog.value = true;
  newRegistryUrl.value = "";
};

const isRegistryValid = computed(
  () =>
    newRegistryUrl.value &&
    newRegistryUrl.value.startsWith("http") &&
    URL.canParse(newRegistryUrl.value) &&
    (registries.value.state === "success" ?
      !Object.values(registries.value.data).includes(newRegistryUrl.value)
    : false),
);

const addRegistry = async () => {
  if (!isRegistryValid.value) return;
  try {
    using _context = dialog.loading(t("addRegistry.loading"));
    await ipc.addRegistry(newRegistryUrl.value);
    void registries.refresh();
    showAddRegistryDialog.value = false;
  } catch (error) {
    dialog.open({
      title: t("error"),
      message: errorToLocalizedString(t, error),
      color: "error",
      actions: [{ label: t("ok") }],
    });
  }
};

const removeRegistry = async (id: string) => {
  const response = await dialog.ask({
    title: t("removeRegistry.title"),
    message: t("removeRegistry.description"),
    color: "warning",
    actions: [
      { label: t("cancel"), value: false },
      { label: t("remove"), color: "warning", value: true },
    ],
  });
  if (!response) return;

  try {
    using _context = dialog.loading(t("removeRegistry.loading"));
    await ipc.removeRegistry(id);
    void registries.refresh();
  } catch (error) {
    dialog.open({
      title: t("error"),
      message: errorToLocalizedString(t, error),
      color: "error",
      actions: [{ label: t("ok") }],
    });
  }
};
</script>

<template>
  <Dialog v-model:open="showAddRegistryDialog">
    <template #title>
      {{ t("addRegistry.title") }}
    </template>
    <DialogDescription>
      {{ t("addRegistry.description") }}
    </DialogDescription>
    <TextInput v-model="newRegistryUrl" type="url" un-w="full" required />

    <template #actions>
      <button class="button" @click="showAddRegistryDialog = false">
        {{ t("cancel") }}
      </button>
      <button
        class="button primary"
        :disabled="!isRegistryValid"
        @click="addRegistry"
      >
        {{ t("add") }}
      </button>
    </template>
  </Dialog>
  <Dialog v-model:open="showAddManifestDialog">
    <template #title>
      {{ t("addManifest.title") }}
    </template>
    <DialogDescription>
      {{ t("addManifest.description") }}
    </DialogDescription>
    <template v-if="addManifestType === 'url'">
      <div un-flex un-gap="2" un-mb="2">
        <button
          class="button primary"
          disabled
          @click="addManifestType = 'url'"
        >
          {{ t("addManifest.type.url") }}
        </button>
        <button class="button" @click="addManifestType = 'file'">
          {{ t("addManifest.type.file") }}
        </button>
      </div>
      <TextInput v-model="newManifestUrl" type="url" un-w="full" required />
    </template>
    <template v-else>
      <div un-flex un-gap="2" un-mb="2">
        <button class="button" @click="addManifestType = 'url'">
          {{ t("addManifest.type.url") }}
        </button>
        <button
          class="button primary"
          disabled
          @click="addManifestType = 'file'"
        >
          {{ t("addManifest.type.file") }}
        </button>
      </div>
      <FileInput
        v-model="newManifestFile"
        accept=".json,.yaml,.yml"
        un-w="full"
        required
      />
    </template>

    <template #actions>
      <button class="button" @click="showAddManifestDialog = false">
        {{ t("cancel") }}
      </button>
      <button
        class="button primary"
        :disabled="!isManifestValid"
        @click="addManifest"
      >
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
        :label="t('addRegistry.title')"
        un-i="fluent-add-circle-16-regular"
        @click="showAddRegistry"
      />
    </section>
    <section un-flex="~ col" un-gap="2" un-h="full">
      <h2>{{ t("contents") }}</h2>
      <p>
        {{ t("contentsDescription") }}
      </p>
      <ScrollArea un-flex-grow>
        <Loading v-if="!contentsLoaded" />
        <template v-else>
          <template v-if="contents.size > 0">
            <ContentCard
              v-for="content in Array.from(contents.values())"
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
            >
              <div
                v-if="contentToRegistry(content.id) === localRegistry"
                un-flex
                un-justify="end"
              >
                <IconButton
                  class="danger"
                  un-i="fluent-delete-16-regular"
                  @click="removeLocalManifest(content.id)"
                />
              </div>
            </ContentCard>
          </template>
          <p v-else un-text="sm slate-500">
            {{ t("noContents") }}
          </p>
          <div
            v-if="errors.length > 0"
            class="card error"
            un-mt="2"
            un-p="2"
            un-flex="~ col"
            un-gap="1"
          >
            <p un-text="red-600 sm">{{ t("contentLoadIssues") }}</p>
            <ul>
              <li v-for="e in errors" :key="e.source + ':' + e.id" un-text="xs">
                <strong
                  >{{
                    e.source === "registry" ?
                      t("registryN", { id: e.id })
                    : t("manifestN", { id: e.id })
                  }}:</strong
                >
                {{ errorToLocalizedString(t, e.error) }}
              </li>
            </ul>
          </div>
        </template>
      </ScrollArea>
      <hr />
      <IconLabelButton
        color="primary"
        un-w="full"
        un-block
        :label="t('addManifest.title')"
        un-i="fluent-add-circle-16-regular"
        @click="showAddManifest"
      />
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
  addRegistry:
    title: "レジストリを追加"
    description: "追加するレジストリのURLを入力してください。"
    loading: "レジストリを追加しています..."
  removeRegistry:
    title: "レジストリを削除"
    description: |
      このレジストリを削除しますか？
    loading: "レジストリを削除しています..."

  addManifest:
    title: "マニフェストを追加"
    description: "追加するマニフェストのURLを入力してください。"
    type:
      url: "URLから追加"
      file: "ファイルから追加"
    loading: "マニフェストを追加しています..."

  removeManifest:
    title: "マニフェストを削除"
    description: |
      このマニフェストを削除しますか？
    loading: "マニフェストを削除しています..."

  add: "追加"
  remove: "削除"

  errors:
    invalid_as_registry: "このURLはレジストリとして有効ではありません。"
  contentLoadIssues: "一部のコンテンツを読み込めませんでした:"
  registryN: "レジストリ {id}"
  manifestN: "マニフェスト {id}"
</i18n>

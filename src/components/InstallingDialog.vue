<script setup lang="ts">
import { DialogDescription } from "reka-ui";
import { ref, onMounted } from "vue";
import { useI18n } from "vue-i18n";
import { InstallChannel, InstallPlan, InstallProgress } from "../lib/ipc.ts";
import Dialog from "./Dialog.vue";
import Spacer from "./Spacer.vue";

const props = defineProps<{
  plan: InstallPlan;
  channel: InstallChannel;
  resolver: (success: boolean) => void;
}>();

const isOpen = ref(true);
const isOk = ref(false);

const i18n = useI18n();
const { t } = i18n;

const progress = ref(new Map<string, InstallProgress>());

onMounted(() => {
  // eslint-disable-next-line vue/no-mutating-props
  props.channel.onmessage = ([manifestId, installProgress]) => {
    progress.value.set(manifestId, installProgress);
  };
});
</script>
<template>
  <Dialog v-model:open="isOpen" @disappeared="props.resolver(isOk)">
    <template #title>
      <Loading />
      {{ t("title") }}
    </template>

    <DialogDescription>
      <section>
        <h2>
          {{ t("toUpdate.title", { count: plan.to_update.length }) }}
        </h2>
        <p v-if="plan.to_update.length === 0">
          {{ t("none") }}
        </p>
        <table v-else un-w="full">
          <thead>
            <tr un-text="lg">
              <th un-pr="2">
                {{ t("toUpdate.name") }}
              </th>
              <th un-pr="2">
                {{ t("toUpdate.oldVersion") }}
              </th>
              <th un-pr="2">
                {{ t("toUpdate.newVersion") }}
              </th>
            </tr>
            <tr un-h="1" un-border="b-1 slate-300 solid" />
            <tr un-h="1" />
          </thead>
          <tbody>
            <tr v-for="[before, after] in plan.to_update" :key="before.id">
              <td un-pr="2">
                {{ before.name }}
              </td>
              <td un-pr="2">
                {{ before.version }}
              </td>
              <td un-pr="2">
                {{ after.version }}
              </td>
            </tr>
          </tbody>
        </table>
      </section>
      <section un-mt="4">
        <h2>
          {{ t("toInstall.title", { count: plan.to_install.length }) }}
        </h2>
        <p v-if="plan.to_install.length === 0">
          {{ t("none") }}
        </p>
        <table v-else un-w="full">
          <thead>
            <tr un-text="lg">
              <th un-pr="4">
                {{ t("toInstall.name") }}
              </th>
              <th>
                {{ t("toInstall.version") }}
              </th>
            </tr>
            <tr un-h="1" un-border="b-1 slate-300 solid" />
            <tr un-h="1" />
          </thead>
          <tbody>
            <tr v-for="item in plan.to_install" :key="item.id">
              <td un-pr="4">
                {{ item.name }}
              </td>
              <td>
                {{ item.version }}
              </td>
            </tr>
          </tbody>
        </table>
      </section>
    </DialogDescription>

    <template #actions>
      <Spacer un-h="2" />
      <div un-flex="~" un-gap="2">
        <button
          class="button"
          un-bg="slate-200 hover:slate-300 active:slate-400"
          @click="
            isOpen = false;
            isOk = false;
          "
        >
          {{ t("cancel") }}
        </button>
        <button
          class="button primary"
          un-flex="~"
          un-items="center"
          un-gap="2"
          @click="
            isOpen = false;
            isOk = true;
          "
        >
          <span>{{ t("continue") }}</span>
        </button>
      </div>
    </template>
  </Dialog>
</template>

<i18n lang="yaml">
ja:
  title: "インストール中"
  none: "（なし）"
  continue: "続行"
</i18n>

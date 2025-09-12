<script setup lang="ts">
import { ref } from "vue";
import Header from "../components/Header.vue";
import BackButton from "../components/BackButton.vue";
import TextInput from "../components/TextInput.vue";
import Select from "../components/Select.vue";

const testInput = ref("");
const selectOptions = [
  { label: "Option 1", value: "option1" },
  { label: "Option 2", value: "option2" },
  { label: "Option 3", value: "option3" },
];
const selectedOption = ref(selectOptions[0].value);
</script>

<template>
  <Header>
    <BackButton />

    Debug View</Header
  >

  <main un-p="2" un-flex="~ col">
    <section>
      <h2 un-text="xl">Button Styles</h2>
      <div un-flex="~ row" un-gap="2" un-mb="2">
        <button class="button">Default</button>
        <button class="button primary">Primary</button>
        <button class="button success">Success</button>
        <button class="button warning">Warning</button>
        <button class="button danger">Danger</button>
        <button class="button error">Error</button>
      </div>
    </section>
    <section>
      <h2 un-text="xl">Borderless Button Styles</h2>
      <div un-flex="~ row" un-gap="2" un-mb="2">
        <button class="button borderless">Default</button>
        <button class="button borderless primary">Primary</button>
        <button class="button borderless success">Success</button>
        <button class="button borderless warning">Warning</button>
        <button class="button borderless danger">Danger</button>
        <button class="button borderless error">Error</button>
      </div>
    </section>
    <section>
      <h2 un-text="xl">Cards</h2>
      <div un-flex="~ row" un-gap="2" un-mb="2">
        <div class="card">Default</div>
        <div class="card primary">Primary</div>
        <div class="card success">Success</div>
        <div class="card warning">Warning</div>
        <div class="card danger">Danger</div>
        <div class="card error">Error</div>
      </div>
    </section>
    <section>
      <h2 un-text="xl">Clickable Cards</h2>
      <div un-flex="~ row" un-gap="2" un-mb="2">
        <div class="button card">Default</div>
        <div class="button card primary">Primary</div>
        <div class="button card success">Success</div>
        <div class="button card warning">Warning</div>
        <div class="button card danger">Danger</div>
        <div class="button card error">Error</div>
      </div>
    </section>
    <section>
      <h2 un-text="xl">Inputs</h2>
      <div un-flex="~ col" un-gap="2" un-mb="2" un-w="64">
        <TextInput v-model="testInput" placeholder="Type something..." />
        <TextInput v-model="testInput" placeholder="Disabled input" disabled />
        <TextInput
          v-model="testInput"
          placeholder="Error input"
          class="error"
        />
      </div>
    </section>

    <section>
      <h2 un-text="xl">Select</h2>
      <div un-flex="~ col" un-gap="2" un-mb="2" un-w="64">
        <Select v-model="selectedOption" :options="selectOptions" />
        <Select v-model="selectedOption" :options="selectOptions" disabled />
        <Select v-model="selectedOption" :options="selectOptions" borderless />
      </div>
    </section>
  </main>
</template>

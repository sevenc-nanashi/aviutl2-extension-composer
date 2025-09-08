<script setup lang="ts">
import { ref } from "vue";
import Header from "../components/Header.vue";
import BackButton from "../components/BackButton.vue";
import TextInput from "../components/TextInput.vue";

const testInput = ref("");
</script>

<template>
  <Header>
    <BackButton to="/profiles" />

    Debug View</Header
  >

  <main un-p="2" un-flex="~ col">
    <section>
      <h2 un-text="xl">Button Styles</h2>
      <div un-flex="~ row" un-gap="2" un-mb="2">
        <button class="button">Default</button>
        <button class="button primary">Primary</button>
        <button class="button warning">Warning</button>
        <button class="button danger">Danger</button>
        <button class="button error">Error</button>
      </div>
    </section>
    <section>
      <h2 un-text="xl">Cards</h2>
      <div un-flex="~ row" un-gap="2" un-mb="2">
        <div class="card">Default</div>
        <div class="card primary">Primary</div>
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
  </main>
</template>

<script setup lang="ts">
import MarkdownIt from "markdown-it";
import { computed } from "vue";

const props = defineProps<{
  content: string;
}>();

const md = new MarkdownIt({
  breaks: true,
});
const renderedContent = computed(() => md.render(props.content));
</script>

<template>
  <!-- eslint-disable-next-line vue/no-v-html -->
  <div v-html="renderedContent" />
</template>

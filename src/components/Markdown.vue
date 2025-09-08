<script setup lang="ts">
import { computed } from "vue";
import MarkdownIt from "markdown-it";

const props = defineProps<{
  content: string;
}>();

const md = new MarkdownIt({
  breaks: true,
});
const renderedContent = computed(() => md.render(props.content));
</script>

<template>
  <div v-html="renderedContent" />
</template>

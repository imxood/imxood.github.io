<template>
  <t-layout style="height: 100%;">
    <t-header>Header</t-header>
    <t-layout>
      <t-aside>Aside</t-aside>
      <t-content>Content</t-content>
    </t-layout>
    <t-footer>Footer</t-footer>
  </t-layout>
</template>

<script lang="ts">
import { defineComponent, ref } from 'vue';
export default defineComponent({
  setup() {
    return {
      checked: ref(false),
    };
  },
});
</script>
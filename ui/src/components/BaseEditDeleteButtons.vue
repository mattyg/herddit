<template>
  <div class="flex justify-start items-center space-x-4">
    <div
      class="text-2xl font-bold text-accent hover:text-accent-focus"
      @click="$emit('edit')"
    >
      <svg
        xmlns="http://www.w3.org/2000/svg"
        width="24"
        height="24"
        viewBox="0 0 24 24"
      ><path
        fill="currentColor"
        d="m12.9 6.858l4.242 4.243L7.242 21H3v-4.243l9.9-9.9zm1.414-1.414l2.121-2.122a1 1 0 0 1 1.414 0l2.829 2.829a1 1 0 0 1 0 1.414l-2.122 2.121l-4.242-4.242z"
      /></svg>
    </div>

    <div
      class="text-2xl font-bold text-accent hover:text-accent-focus "
      @click="$emit('delete')"
    >
      <svg
        xmlns="http://www.w3.org/2000/svg"
        width="24"
        height="24"
        viewBox="0 0 24 24"
      ><path
        fill="currentColor"
        d="M6 7H5v13a2 2 0 0 0 2 2h10a2 2 0 0 0 2-2V7H6zm10.618-3L15 2H9L7.382 4H3v2h18V4z"
      /></svg>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { defineEmits } from 'vue'

defineEmits(['edit', 'delete']);
</script>

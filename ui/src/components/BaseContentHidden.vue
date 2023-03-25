<template>
  <div
    class="w-full flex flex-row justify-between items-center bg-base-200 px-8 py-4 space-x-8 text-gray-400 font-bold"
  >
    <div>
      <slot>
        Content Hidden
      </slot>
    </div>
    <button
      class="btn btn-ghost btn-xs"
      @click="$emit('show')"
    >
      <svg
        class="mr-2"
        xmlns="http://www.w3.org/2000/svg"
        width="24"
        height="24"
        viewBox="0 0 24 24"
      ><path
        fill="currentColor"
        d="M12 4.5C7 4.5 2.73 7.61 1 12c1.73 4.39 6 7.5 11 7.5s9.27-3.11 11-7.5c-1.73-4.39-6-7.5-11-7.5zM12 17c-2.76 0-5-2.24-5-5s2.24-5 5-5s5 2.24 5 5s-2.24 5-5 5zm0-8c-1.66 0-3 1.34-3 3s1.34 3 3 3s3-1.34 3-3s-1.34-3-3-3z"
      /></svg>
      Peek
    </button>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue'

export default defineComponent({
  emits: ['show']
})
</script>
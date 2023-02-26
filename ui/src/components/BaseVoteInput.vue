<template>
  <div class="flex flex-col justify-center items-center text-base-300">
    <div
      class="text-2xl font-bold "
      :class="{'text-accent': myVote === 1, 'hover:text-accent-focus': myVote !== 1}"
      @click="$emit('upvote')"
    >
      <svg
        xmlns="http://www.w3.org/2000/svg"
        width="24"
        height="24"
        viewBox="0 0 24 24"
      ><path
        fill="currentColor"
        d="M4 14h4v7a1 1 0 0 0 1 1h6a1 1 0 0 0 1-1v-7h4a1.001 1.001 0 0 0 .781-1.625l-8-10c-.381-.475-1.181-.475-1.562 0l-8 10A1.001 1.001 0 0 0 4 14z"
      /></svg>
    </div>
   
    <div class="w-4 flex justify-center">
      <div
        class="text-lg my-2 font-bold text-accent"
      >
        {{ votes }}
      </div>
    </div>

    <div
      class="text-2xl font-bold hover:text-accent-focus"
      :class="{'text-accent': myVote === -1, 'hover:text-accent-focus': myVote !== -1}"
      @click="$emit('downvote')"
    >
      <svg
        xmlns="http://www.w3.org/2000/svg"
        width="24"
        height="24"
        viewBox="0 0 24 24"
      ><path
        fill="currentColor"
        d="M20.901 10.566A1.001 1.001 0 0 0 20 10h-4V3a1 1 0 0 0-1-1H9a1 1 0 0 0-1 1v7H4a1.001 1.001 0 0 0-.781 1.625l8 10a1 1 0 0 0 1.562 0l8-10c.24-.301.286-.712.12-1.059z"
      /></svg>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue'

export default defineComponent({
  props: {
    votes: {
      type: Number,
      default: 0
    },
    myVote: {
      type: Number,
      default: 0
    }
  },
  emits: ['upvote', 'downvote']
})
</script>

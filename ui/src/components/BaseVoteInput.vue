<template>
  <div class="flex flex-col justify-center items-center text-base-300">
    <mwc-icon-button
      class="text-2xl font-bold"
      :class="{'text-lg': size === 'sm', 'text-primary': myVote === 1 }"
      icon="arrow_upward"
      @click="$emit('upvote')"
    />
    <div
      class="text-2xl font-bold text-primary"
      :class="{'text-lg': size === 'sm'}"
    >
      {{ votes }}
    </div>
    <mwc-icon-button
      class="text-2xl font-bold"
      :class="{'text-lg': size === 'sm', 'text-primary': myVote === -1}"
      icon="arrow_downward"
      @click="$emit('downvote')"
    />
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
    },
    size: {
      type: String,
      default: 'lg'
    }
  },
  emits: ['upvote', 'downvote']
})
</script>

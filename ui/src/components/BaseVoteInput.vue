<template>
  <div class="flex flex-col justify-center items-center">
    <mwc-icon-button
      class="text-2xl font-bold text-gray-400"
      :class="{'text-lg': size === 'sm', 'text-blue-500': myVote === 1 }"
      icon="arrow_upward"
      @click="$emit('upvote')">
    </mwc-icon-button>
    <div class="text-2xl text-gray-400 font-bold"
      :class="{'text-lg': size === 'sm'}"
    >
      {{ votes }}
    </div>
    <mwc-icon-button
      class="text-2xl font-bold text-gray-400"
      :class="{'text-lg': size === 'sm', 'text-blue-500': myVote === -1}"
      icon="arrow_downward"
      @click="$emit('downvote')">
    </mwc-icon-button>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue'

export default defineComponent({
  props: {
    votes: {
      default: 0
    },
    myVote: {
      default: 0
    },
    size: {
      default: 'lg'
    }
  }
})
</script>

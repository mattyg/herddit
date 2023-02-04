<template>
  <div>
    <button class="text-2xl font-bold text-gray-400 font-bold" :class="{'text-lg': size === 'sm',}" @click="upvotePost">⇧</button>
    <div class="text-2xl font-bold text-gray-400 font-bold" :class="{'text-blue-500': my_vote !== undefined, 'text-lg': size === 'sm'}">{{ votes }}</div>
    <button class="text-2xl font-bold text-gray-400 font-bold" :class="{'text-lg': size === 'sm'}"  @click="downvotePost">⇩</button>
  </div>
</template>

<script lang="ts">
import { AppAgentClient } from '@holochain/client';
import { ComputedRef, defineComponent, inject, PropType } from 'vue'

export default defineComponent({
  props: {
    dnaHash: {
      type: Object as PropType<Uint8Array>,
      required: true
    },
    postHash: {
      type: Object as PropType<Uint8Array>,
      required: true
    },
    votes: {
      default: 0
    },
    size: {
      default: "lg"
    }
  },
  data(): { my_vote?: number; } {
    return {
      my_vote: undefined,
    }
  },
  methods: {
    async upvotePost() {
      if(this.my_vote === 1) return;

      try {
        await this.client.callZome({
          cell_id: [this.dnaHash, this.client.myPubKey],
          cap_secret: null,
          zome_name: 'posts',
          fn_name: 'upvote_post',
          payload: this.postHash,
        });
        this.$emit('upvote');
      } catch (e: any) {
        this.$emit('error', e.data.data);
      }
    },
    async downvotePost() {
      if(this.my_vote === -1) return;

      try {
        await this.client.callZome({
          cell_id: [this.dnaHash, this.client.myPubKey],
          cap_secret: null,
          zome_name: 'posts',
          fn_name: 'downvote_post',
          payload: this.postHash,
        });
        this.$emit('downvote');
      } catch (e: any) {
        this.$emit('error', e.data.data);
      }
    },
  },
  setup() {
    const client = (inject('client') as ComputedRef<AppAgentClient>).value;
    return {
      client
    };
  },
})
</script>

<style scoped>

</style>
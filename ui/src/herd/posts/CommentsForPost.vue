
<template>  
  <div
    v-if="!loading"
    class="w-full md:max-w-screen-md"
  >
    <CreateComment
      class="mb-12"
      :dna-hash="dnaHash"
      :post-hash="postHash"
      @created="fetchComments"
    />

    <span v-if="error">Error fetching the comments: {{ error.data.data }}.</span>
    <div
      v-else-if="hashes && hashes.length > 0"
      class="w-full"
    >
      <CommentDetail 
        v-for="hash in hashes" 
        :key="encodeHashToBase64(hash)"
        :dna-hash="dnaHash"
        :comment-hash="hash" 
        :post-author-hash="postAuthorHash"
        class="my-8"
        @deleted="fetchComments"
      />
    </div>
    <div
      v-else
      class="text-gray-400 font-bold text-center"
    >
      No comments found for this post.
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, inject, ComputedRef, PropType } from 'vue';
import { AppAgentClient, ActionHash, encodeHashToBase64 } from '@holochain/client';
import CommentDetail from './CommentDetail.vue';
import CreateComment from './CreateComment.vue';

export default defineComponent({
  components: {
    CreateComment,
    CommentDetail,
  },
  props: {
    dnaHash: {
      type: Object as PropType<Uint8Array>,
      required: true
    },
    postHash: {
      type: Object as PropType<Uint8Array>,
      required: true
    },
    postAuthorHash: {
      type: Object as PropType<Uint8Array>,
      required: true
    }
  },
  setup() {
    const client = (inject('client') as ComputedRef<AppAgentClient>).value;
    return {
      client,
    };
  },
  data(): { hashes: Array<ActionHash> | undefined; loading: boolean; error: any } {
    return {
      hashes: undefined,
      loading: true,
      error: undefined
    }
  },
  async mounted() {
    await this.fetchComments()
  },
  methods: {
    async fetchComments() {   
      console.log('fetching comments again');   
      try {
        this.hashes = await this.client.callZome({
          cell_id: [this.dnaHash, this.client.myPubKey],
          zome_name: 'posts',
          fn_name: 'get_comments_for_post',
          payload: this.postHash,
        });
      } catch (e) {
        this.error = e;
      }

      this.loading = false;
    },
    encodeHashToBase64(val: Uint8Array) {
      return encodeHashToBase64(val);
    }
  },
})
</script>

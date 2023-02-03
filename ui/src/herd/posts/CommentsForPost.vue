
<template>
  <div v-if="loading" style="display: flex; flex: 1; align-items: center; justify-content: center">
    <mwc-circular-progress indeterminate></mwc-circular-progress>
  </div>
  
  <div v-else style="display: flex; flex-direction: column">
    <CreateComment :dnaHash="dnaHash" :postHash="postHash" @created="fetchComments" />

    <span v-if="error">Error fetching the comments: {{error.data.data}}.</span>
    <div v-else-if="hashes && hashes.length > 0">
      <CommentDetail 
        v-for="hash in hashes" 
        :dnaHash="dnaHash"
        :commentHash="hash" 
        style="margin-bottom: 8px">
      </CommentDetail>
    </div>
    <span v-else>No comments found for this post.</span>
  </div>

</template>

<script lang="ts">
import { defineComponent, inject, ComputedRef, PropType } from 'vue';
import { decode } from '@msgpack/msgpack';
import { AppAgentClient, Record, AgentPubKey, EntryHash, ActionHash } from '@holochain/client';
import '@material/mwc-circular-progress';
import CommentDetail from './CommentDetail.vue';
import CreateComment from './CreateComment.vue';
import { error } from 'console';

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
    }
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
    }
  },
  setup() {
    const client = (inject('client') as ComputedRef<AppAgentClient>).value;
    return {
      client,
    };
  },
})
</script>

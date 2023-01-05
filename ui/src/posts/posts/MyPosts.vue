<template>
  <div v-if="loading" style="display: flex; flex: 1; align-items: center; justify-content: center">
    <mwc-circular-progress indeterminate></mwc-circular-progress>
  </div>

  <div v-else style="display: flex; flex-direction: column">
    <span v-if="error">Error fetching the posts: {{error.data.data}}.</span>
    <div v-else-if="hashes && hashes.length > 0">
      <PostDetail 
        v-for="hash in hashes" 
        :post-hash="hash"
        @post-deleted="fetchPost()"
        style="margin-bottom: 8px">
      </PostDetail>
    </div>
    <span v-else>No posts found for this author.</span>
  </div>

</template>

<script lang="ts">
import { defineComponent, inject, ComputedRef } from 'vue';
import { decode } from '@msgpack/msgpack';
import { AppAgentClient, Record, AgentPubKey, EntryHash, ActionHash } from '@holochain/client';
import '@material/mwc-circular-progress';
import PostDetail from './PostDetail.vue';

export default defineComponent({
  components: {
    PostDetail
  },
  props: { 
    author: {
      type: Object,
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
    await this.fetchPost();
  },
  methods: {
    async fetchPost() {
      try {
        this.hashes = await this.client.callZome({
          cap_secret: null,
          role_name: 'posts',
          zome_name: 'posts',
          fn_name: 'get_my_posts',
          payload: this.author,
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

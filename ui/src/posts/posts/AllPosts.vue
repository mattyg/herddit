<template>
  <div v-if="loading" style="display: flex; flex: 1; align-items: center; justify-content: center">
    <mwc-circular-progress indeterminate></mwc-circular-progress>
  </div>

  <div v-else class="flex justify-center">
    <div class="max-w-screen-md">
      <div v-if="error">Error fetching the posts: {{error.data.data}}.</div>
      <div v-else-if="hashes && hashes.length > 0" class="flex-row space-y-8">
        <PostListItem 
          v-for="hash in hashes" 
          :post-hash="hash">
        </PostListItem>
      </div>
      <span v-else>No posts found.</span>
    </div>
  </div>

</template>

<script lang="ts">
import { defineComponent, inject, ComputedRef } from 'vue';
import { decode } from '@msgpack/msgpack';
import { AppAgentClient, Record, AgentPubKey, EntryHash, ActionHash } from '@holochain/client';
import '@material/mwc-circular-progress';
import PostListItem from './PostListItem.vue';

export default defineComponent({
  components: {
    PostListItem
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
          fn_name: 'get_all_posts',
          payload: null,
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

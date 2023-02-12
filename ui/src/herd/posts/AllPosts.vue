<template>
  <div
    v-if="loading"
    style="display: flex; flex: 1; align-items: center; justify-content: center"
  >
    <mwc-circular-progress indeterminate />
  </div>

  <div
    v-else
    class="w-full flex justify-center"
  >
    <div v-if="error">
      Error fetching the posts: {{ error.data.data }}.
    </div>
    <div
      v-else-if="hashes && hashes.length > 0"
      class="w-full md:max-w-screen-md flex flex-col justify-center items-start space-y-8"
    >
      <PostListItem 
        v-for="hash in hashes" 
        :key="encodeHashToBase64(hash)"
        :dna-hash="dnaHash"
        :post-hash="hash"
        class="w-full"
      />
    </div>
    <div
      v-else
      class="flex flex-col items-center justify-center"
    >
      <div class="text-4xl font-bold mb-16 text-center">
        The herd was quiet...
      </div>

      <RouterLink
        :to="`${$route.fullPath}/posts/create`"
        class="btn btn-neutral btn-lg"
      >
        Call to Herd
      </RouterLink>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, inject, ComputedRef, PropType } from 'vue';
import { AppAgentClient, ActionHash, encodeHashToBase64 } from '@holochain/client';
import PostListItem from './PostListItem.vue';

export default defineComponent({
  components: {
    PostListItem
  },
  props: {
    dnaHash: {
      type: Object as PropType<Uint8Array>,
      required: true
    },
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
    await this.fetchAllPosts();
  },
  methods: {
    async fetchAllPosts() {
      try {
        const response = await this.client.callZome({
          cell_id: [this.dnaHash, this.client.myPubKey],
          cap_secret: null,
          zome_name: 'posts',
          fn_name: 'get_all_posts_sorted_by_votes',
          payload: null,
        });
        console.log('post hashes received is',response);

        this.hashes = response;
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

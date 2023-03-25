<template>
  <div
    v-if="!hashes && loading"
    class="h-full flex flex-1 justify-center items-center"
  >
    <BaseSpinner>Tracking down the herd...</BaseSpinner>
  </div>

  <div
    v-else
    class="w-full flex justify-center"
  >
    <div
      v-if="hashes && hashes.length > 0"
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

<script lang="ts" setup>
import { inject, ComputedRef, defineProps } from 'vue';
import { AppAgentClient,encodeHashToBase64 } from '@holochain/client';
import PostListItem from './PostListItem.vue';
import { toast } from 'vue3-toastify';
import BaseSpinner from '../../components/BaseSpinner.vue';
import { useRequest } from 'vue-request';

const client = (inject('client') as ComputedRef<AppAgentClient>).value;

const props = defineProps<{
  dnaHash: Uint8Array
}>();

const fetchAllPosts = async () => {
  const response = await client.callZome({
    cell_id: [props.dnaHash, client.myPubKey],
    zome_name: 'posts',
    fn_name: 'get_all_posts_sorted_by_votes',
    payload: null,
  });

  return response;
};

const { data: hashes, loading } = useRequest(fetchAllPosts, {
  pollingInterval: 1000,
  onError: (e: any) => {
    toast.error(`Error fetching calls ${e.data.data}`);
  }
})

</script>

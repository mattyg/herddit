
<template>  
  <div class="w-full md:max-w-screen-md">
    <CreateComment
      class="mb-12"
      :dna-hash="dnaHash"
      :post-hash="postHash"
      @created="runFetchComments"
    />
    <div
      v-if="hashes && hashes.length > 0"
      class="w-full"
    >
      <CommentDetail 
        v-for="(hash, i) in hashes" 
        :key="i"
        :dna-hash="dnaHash"
        :original-action-hash="hash" 
        :post-author-hash="postAuthorHash"
        class="my-8"
        @deleted="runFetchComments"
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

<script lang="ts" setup>
import { inject, ComputedRef } from 'vue';
import { AppAgentClient } from '@holochain/client';
import CommentDetail from './CommentDetail.vue';
import CreateComment from './CreateComment.vue';
import { toast } from 'vue3-toastify';
import { useRequest } from 'vue-request';

const props = defineProps<{
  dnaHash: Uint8Array,
  postHash: Uint8Array,
  postAuthorHash: Uint8Array
}>();
const client = (inject('client') as ComputedRef<AppAgentClient>).value;

const fetchComments = async () => {   
  const res = await client.callZome({
    cell_id: [props.dnaHash, client.myPubKey],
    zome_name: 'posts',
    fn_name: 'get_comments_for_post',
    payload: props.postHash,
  });

  return res;
};

const { data: hashes, run: runFetchComments } = useRequest(fetchComments, {
  pollingInterval: 1000,
  onError: (e: any) => {
    toast.error(`Failed to fetch comments: ${e.data.data}`);
  }
});
</script>

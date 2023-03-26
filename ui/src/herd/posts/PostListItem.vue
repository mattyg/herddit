<template>
  <div v-if="post_metadata?.record">
    <div
      v-if="isDeleted"
      class="w-full bg-base-200 px-8 py-4 space-x-8 text-gray-400 font-bold"
    >
      <div>Call deleted by author</div>
    </div>
    <div
      v-else-if="votesCount >= 0 || showIfVoteNegative"
      class="w-full flex flex-row justify-start items-center bg-base-200 text-base-content px-8 py-4 space-x-8"
    >
      <PostVotes 
        :votes="votesCount" 
        :dna-hash="dnaHash" 
        :post-hash="postHash"
        size="sm"
        @upvote="fetchPost()"
        @downvote="fetchPost()"
      />
      <RouterLink
        :to="`${$route.fullPath}/posts/${postHashString}`"
        class="w-full flex flex-col space-y-1"
      >
        <div class="w-full text-3xl mb-4">
          {{ post?.title }}
        </div>
        <div
          v-if="authorHash"
          class="flex flex-row items-center justify-between"
        >
          <div class="text-md text-gray-400 font-bold">
            Submitted {{ dateRelative }} 
          </div>

          <AgentProfile
            :agent-pub-key="authorHash"
            size="sm"
            :muted="true"
          /> 
        </div>
      </RouterLink>
    </div>
    <BaseContentHidden
      v-else
      :allowPeeking="true"
      @show="showIfVoteNegative = true"
    >
      Call trampled by the herd
    </BaseContentHidden>
  </div>
</template>

<script lang="ts" setup>
import { ref, inject, ComputedRef, computed, watch} from 'vue';
import { decode } from '@msgpack/msgpack';
import { AppAgentClient, Record, encodeHashToBase64 } from '@holochain/client';
import { Post } from './types';
import PostVotes from './PostVotes.vue';
import AgentProfile from '../profiles/AgentProfile.vue';
import BaseContentHidden from '../../components/BaseContentHidden.vue';
import dayjs from 'dayjs';
import relativeTime from 'dayjs/plugin/relativeTime';
import { useRequest } from 'vue-request';
import { toast } from 'vue3-toastify';
dayjs.extend(relativeTime);

const props = defineProps<{
  dnaHash: Uint8Array,
  postHash: Uint8Array
}>();
const client = (inject('client') as ComputedRef<AppAgentClient>).value;

const showIfVoteNegative = ref(false);

const votesCount = computed(() => {
  if (!post_metadata.value) return 0;

  return post_metadata.value.upvotes - post_metadata.value.downvotes
});

const post = computed(() => {
  if (!post_metadata.value?.record) return undefined;
  return decode((post_metadata.value.record.entry as any).Present.entry) as Post;
});

const isDeleted = computed(() => {
  if (!post_metadata.value?.record) return undefined;
  return post_metadata.value.record.signed_action.hashed.content.type === 'Delete';
});

const authorHash = computed(() => {
  if (!post_metadata.value?.record) return undefined;
  return post_metadata.value.record.signed_action.hashed.content.author;
});

const dateRelative = computed(() => {
  if(!post_metadata.value?.record.signed_action.hashed.content.timestamp) return;
  return dayjs(post_metadata.value.record.signed_action.hashed.content.timestamp/1000).fromNow();
});

const postHashString = computed(() => {      
  return encodeHashToBase64(props.postHash);
});

const fetchPost = async (): Promise<{ original_action_hash: Uint8Array, record: Record, upvotes: number, downvotes: number }> => {
  const res = await client.callZome({
    cell_id: [props.dnaHash, client.myPubKey],
    cap_secret: null,
    zome_name: 'posts',
    fn_name: 'get_post_metadata',
    payload: props.postHash,
  });

  return res;
};

const {data: post_metadata, run: runFetchPost } = useRequest(fetchPost, {
  onError: (e: any) => {
    toast.error(`Failed to fetch post ${e.data.data}`)
  }
});

watch(props, () => {
  runFetchPost();
});
</script>

<template>
  <div class="w-full flex justify-center text-base-content">
    <div class="w-full md:max-w-screen-xl">
      <div v-if="post_metadata">
        <div v-if="editing && post_metadata.record">
          <EditPost 
            :dna-hash="dnaHash" 
            :post-hash="postHash" 
            :current-record="post_metadata.record" 
            @updated="() => { editing = false; runFetchPost(); }" 
            @cancelled="() => {editing = false;}"
          />
        </div>
        <div
          v-else-if="post_metadata.record && postContent && authorHash"
          class="flex flex-row justify-center items-start space-x-4"
        >
          <PostVotes 
            class="mr-2"
            :votes="post_metadata.upvotes - post_metadata.downvotes" 
            :dna-hash="dnaHash" 
            :post-hash="postHash"
            @upvote="runFetchPost"
            @downvote="runFetchPost"
          />

          <div class="w-full pr-8">
            <div class="flex flex-col justify-start items-center space-y-4 mb-4">
              <div class="w-full text-5xl">
                {{ post?.title }}
              </div>
                   
              <div class="w-full flex flex-row justify-between items-center">
                <div class="text-lg text-gray-400 font-bold">
                  Submitted {{ dateRelative }}
                </div>
                <AgentProfile
                  :agent-pub-key="authorHash"
                  size="md"
                  class="mx-4"
                />
              </div>
            </div>

            <div class="relative w-full bg-base-200 p-12 shadow-sm flex flex-col items-center mb-12">
              <!-- eslint-disable vue/no-v-html -->
              <div
                class="w-full pb-4 prose md:prose-md lg:prose-xl"
                v-html="postContent"
              /> 
              <!-- eslint-enable vue/no-v-html -->
              <BaseEditDeleteButtons
                v-if="myPost"
                class="absolute right-0 bottom-0 p-4"
                @edit="editing = true"
                @delete="deletePost()"
              />
            </div>
            
            <div class="w-full flex justify-center">
              <CommentsForPost
                :dna-hash="dnaHash"
                :post-hash="postHash"
                :post-author-hash="authorHash"
              />
            </div>
          </div>
        </div>
        
        <span v-else>The requested post was not found.</span>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { inject, ComputedRef, ref, computed } from 'vue';
import { decode } from '@msgpack/msgpack';
import { AppAgentClient, decodeHashFromBase64, Record } from '@holochain/client';
import { Post } from './types';
import PostVotes from './PostVotes.vue';
import CommentsForPost from './CommentsForPost.vue';
import AgentProfile from '../profiles/AgentProfile.vue';
import EditPost from './EditPost.vue';
import {marked} from 'marked';
import dayjs from 'dayjs';
import relativeTime from 'dayjs/plugin/relativeTime';
import { isEqual } from 'lodash';
import { toast } from 'vue3-toastify';
import BaseEditDeleteButtons from '../../components/BaseEditDeleteButtons.vue';
import DOMPurify from 'dompurify';
import { useRoute, useRouter } from 'vue-router';
import { useRequest } from 'vue-request';
dayjs.extend(relativeTime);

const route = useRoute();
const router = useRouter();
const props = defineProps<{
  dnaHash: Uint8Array
}>();
const emit = defineEmits(['post-deleted'])
const editing = ref(false);
const client = (inject('client') as ComputedRef<AppAgentClient>).value;

const post = computed(() => {
  if (!post_metadata.value?.record) return undefined;

  return decode((post_metadata.value.record.entry as any).Present.entry) as Post;
});

const postHash = computed(() => {
  return decodeHashFromBase64(route.params.postHashString as string);
});

const postContent = computed(() => {
  if(!post.value?.content) return undefined;
  return DOMPurify.sanitize(marked(post.value?.content));
});

const myPost = computed(() => {
  if(!post_metadata.value?.record) return false;
  return isEqual(post_metadata.value.record.signed_action.hashed.content.author, client.myPubKey);
});

const authorHash = computed(() => {
  if (!post_metadata.value?.record) return undefined;
  return post_metadata.value.record.signed_action.hashed.content.author;
});

const dateRelative = computed((): undefined | string => {
  if(!post_metadata.value?.record) return undefined;
  return dayjs(post_metadata.value.record.signed_action.hashed.content.timestamp/1000).fromNow();
});

const fetchPost = async (): Promise<{ upvotes: number, downvotes: number, record: Record }> => {
  const post_metadata = await client.callZome({
    cell_id: [props.dnaHash, client.myPubKey],
    zome_name: 'posts',
    fn_name: 'get_post_metadata',
    payload: postHash.value,
  });

  return post_metadata;
};

const deletePost = async() => {
  try {
    await client.callZome({
      cell_id: [props.dnaHash, client.myPubKey],
      cap_secret: null,
      zome_name: 'posts',
      fn_name: 'delete_post',
      payload: postHash.value,
    });
    emit('post-deleted', postHash.value);
    router.push(`/herds/${route.params.listingHashString}`);
  } catch (e: any) {
    toast.error(`Error deleting the post: ${e.data.data}`);
  }
};

const { data: post_metadata, run: runFetchPost } = useRequest(fetchPost, {
  pollingInterval: 1000,
  onError: (e: any) => {
    toast.error(`Error fetching the post: ${e.data.data}`);
  }
});
</script>

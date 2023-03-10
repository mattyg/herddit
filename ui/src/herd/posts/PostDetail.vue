<template>
  <div class="w-full flex justify-center">
    <div class="w-full md:max-w-screen-xl">
      <div v-if="!loading">
        <div v-if="editing && record">
          <EditPost 
            :dna-hash="dnaHash" 
            :post-hash="postHash" 
            :current-record="record" 
            @updated="() => { editing = false; fetchPost(); }" 
            @cancelled="() => {editing = false;}"
          />
        </div>
        <div
          v-else-if="record && postContent && authorHash"
          class="flex flex-row justify-center items-start space-x-4"
        >
          <PostVotes 
            class="mr-2"
            :votes="upvotes - downvotes" 
            :dna-hash="dnaHash" 
            :post-hash="postHash"
            @upvote="fetchPost"
            @downvote="fetchPost"
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

<script lang="ts">
import { defineComponent, inject, ComputedRef, PropType } from 'vue';
import { decode } from '@msgpack/msgpack';
import { AppAgentClient, Record, AppInfo, encodeHashToBase64, decodeHashFromBase64 } from '@holochain/client';
import { Post } from './types';
import PostVotes from './PostVotes.vue';
import CommentsForPost from './CommentsForPost.vue';
import AgentProfile from '../profiles/AgentProfile.vue';
import EditPost from './EditPost.vue';
import {marked} from 'marked';
import dayjs from 'dayjs';
import { isEqual } from 'lodash';
import { toast } from 'vue3-toastify';
import BaseEditDeleteButtons from '../../components/BaseEditDeleteButtons.vue';
// Override function
const renderer = {
  link(href: string) {
    return href;
  }
};

marked.use({ renderer });

export default defineComponent({
  components: {
    PostVotes,
    CommentsForPost,
    EditPost,
    AgentProfile,
    BaseEditDeleteButtons,
  },
  props: {
    dnaHash: {
      // @ts-ignore
      type: Object as PropType<Uint8Array>,
      required: true
    }
  },
  emits: ['post-deleted'],
  setup() {
    const client = (inject('client') as ComputedRef<AppAgentClient>).value;
    return {
      client
    };
  },
  data(): { record: Record | undefined; upvotes: number; downvotes: number; my_vote?: number; loading: boolean; editing: boolean; appInfo?: AppInfo} {
    return {
      record: undefined,
      upvotes: 0,
      downvotes: 0,
      my_vote: undefined,
      loading: true,
      editing: false,
      appInfo: undefined,
    }
  },
  computed: {
    post() {
      if (!this.record) return undefined;
      return decode((this.record.entry as any).Present.entry) as Post;
    },
    postHash() {
      return decodeHashFromBase64(this.$route.params.postHashString as string);
    },
    postContent() {
      if(!this.post?.content) return undefined;

      return marked(this.post?.content);
    },
    myPost() {
      if(!this.record || !this.appInfo) return false;
      return isEqual(this.record.signed_action.hashed.content.author, this.client.myPubKey);
    },
    authorHash() {
      if (!this.record) return undefined;

      return this.record.signed_action.hashed.content.author;
    },
    authorHashString() {
      if(!this.authorHash) return;
      
      return encodeHashToBase64(this.authorHash);
    },
    dateRelative() {
      if(!this.record?.signed_action.hashed.content.timestamp) return;

      return dayjs(this.record.signed_action.hashed.content.timestamp/1000).fromNow();
    },
  },
  async mounted() {
    await this.fetchPost();
    this.appInfo = await this.client.appInfo();
  },
  methods: {
    async fetchPost() {
      try {
        const post_metadata = await this.client.callZome({
          cell_id: [this.dnaHash, this.client.myPubKey],
          zome_name: 'posts',
          fn_name: 'get_post_metadata',
          payload: this.postHash,
        });

        console.log('post_metadata', post_metadata)
        this.upvotes = post_metadata.upvotes;
        this.downvotes = post_metadata.downvotes;
        this.record = post_metadata.record;
      } catch (e: any) {
         toast.error(`Error fetching the post: ${e.data.data}`);
      }

      this.loading = false;
    },
    async deletePost() {
      try {
        await this.client.callZome({
          cell_id: [this.dnaHash, this.client.myPubKey],
          cap_secret: null,
          zome_name: 'posts',
          fn_name: 'delete_post',
          payload: this.postHash,
        });
        this.$emit('post-deleted', this.postHash);
        this.$router.push(`/herds/${this.$route.params.listingHashString}`);
      } catch (e: any) {
        toast.error(`Error deleting the post: ${e.data.data}`);
      }
    },
  }
})
</script>

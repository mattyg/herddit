<template>
  <div v-if="record">
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

<script lang="ts">
import { defineComponent, inject, ComputedRef, PropType } from 'vue';
import { decode } from '@msgpack/msgpack';
import { AppAgentClient, Record, encodeHashToBase64 } from '@holochain/client';
import { Post } from './types';
import PostVotes from './PostVotes.vue';
import AgentProfile from '../profiles/AgentProfile.vue';
import BaseContentHidden from '../../components/BaseContentHidden.vue';
import dayjs from 'dayjs';
import relativeTime from 'dayjs/plugin/relativeTime';
dayjs.extend(relativeTime);

export default defineComponent({
  components: {
    PostVotes,
    AgentProfile,
    BaseContentHidden,
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
  setup() {
    const client = (inject('client') as ComputedRef<AppAgentClient>).value;
    return {
      client
    };
  },
  data(): { record: Record | undefined; loading: boolean; editing: boolean; votesCount: number; showIfVoteNegative: boolean; } {
    return {
      record: undefined,
      loading: true,
      editing: false,
      votesCount: 0,
      showIfVoteNegative: false,
    }
  },
  computed: {
    post() {
      if (!this.record) return undefined;
      return decode((this.record.entry as any).Present.entry) as Post;
    },
    isDeleted() {
      if (!this.record) return undefined;

      return this.record.signed_action.hashed.content.type === 'Delete';
    },
    authorHash() {
      if (!this.record) return undefined;

      return this.record.signed_action.hashed.content.author;
    },
    dateRelative() {
      if(!this.record?.signed_action.hashed.content.timestamp) return;

      return dayjs(this.record.signed_action.hashed.content.timestamp/1000).fromNow();
    },
    postHashString() {      
      return encodeHashToBase64(this.postHash);
    },
    dnaHashString() {
      return encodeHashToBase64(this.dnaHash);
    }
  },
  watch: {
    dnaHash() {
      this.fetchPost();
    },
    postHash() {
      this.fetchPost();
    }
  },
  async mounted() {
    await this.fetchPost();
  },
  methods: {
    async fetchPost() {
      const post_metadata = await this.client.callZome({
        cell_id: [this.dnaHash, this.client.myPubKey],
        cap_secret: null,
        zome_name: 'posts',
        fn_name: 'get_post_metadata',
        payload: this.postHash,
      });

      this.record = post_metadata.record;
      this.votesCount = post_metadata.upvotes - post_metadata.downvotes;

      this.loading = false;
    },
  },
})
</script>

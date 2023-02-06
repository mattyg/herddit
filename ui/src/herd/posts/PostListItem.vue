<template>
  <div v-if="!loading">
    <div v-if="record" class="flex flex-row flex-start items-center">
      <PostVotes 
        :votes="votesCount" 
        :dnaHash="dnaHash" 
        :postHash="postHash"
        @upvote="fetchPost()"
        @downvote="fetchPost()"
        class="mr-8"
        size="sm"
      />
      <RouterLink :to="`${$route.fullPath}/posts/${postHashString}`">
        <div class="w-full flex flex-col bg-neutral-1 hover:bg-neutral-2">
          <div class="w-full text-3xl mb-2">{{ post?.title }}</div>
          <div class="text-md text-gray-400 font-bold">Submitted {{dateRelative}} by {{authorHash}}</div>
        </div>
      </RouterLink>
    </div>

    
    <span v-else>The requested post was not found.</span>
  </div>

  <div v-else style="display: flex; flex: 1; align-items: center; justify-content: center">
    <mwc-circular-progress indeterminate></mwc-circular-progress>
  </div>

  <mwc-snackbar ref="delete-error" leading>
  </mwc-snackbar>
</template>

<script lang="ts">
import { defineComponent, inject, ComputedRef, PropType } from 'vue';
import { decode } from '@msgpack/msgpack';
import { AppAgentClient, Record, AgentPubKey, EntryHash, ActionHash, encodeHashToBase64, decodeHashFromBase64 } from '@holochain/client';
import { Post } from './types';
import '@material/mwc-circular-progress';
import '@material/mwc-icon-button';
import '@material/mwc-snackbar';
import { Snackbar } from '@material/mwc-snackbar';
import PostVotes from './PostVotes.vue';
import dayjs from 'dayjs';
import relativeTime from 'dayjs/plugin/relativeTime';
dayjs.extend(relativeTime);

export default defineComponent({
  components: {
    PostVotes,
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
  data(): { record: Record | undefined; loading: boolean; editing: boolean; votesCount: number} {
    return {
      record: undefined,
      loading: true,
      editing: false,
      votesCount: 0,
    }
  },
  computed: {
    post() {
      if (!this.record) return undefined;
      return decode((this.record.entry as any).Present.entry) as Post;
    },
    authorHash() {
      if (!this.record) return undefined;

      return encodeHashToBase64(this.record.signed_action.hashed.content.author);
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
      this.loading = true;
      this.record = undefined;

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
  setup() {
    const client = (inject('client') as ComputedRef<AppAgentClient>).value;
    return {
      client
    };
  },
})
</script>

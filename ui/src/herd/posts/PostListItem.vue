<template>
  <div v-if="!loading">
    <RouterLink v-if="record" :to="`/herds/${dnaHashString}/posts/${postHashString}`">
      <div class="w-full flex flex-col bg-neutral-1 hover:bg-neutral-2">
        <div class="w-full text-2xl">{{ post?.title }}</div>
        <div class="text-xs">Submitted {{dateRelative}} by {{authorHash}}</div>
      </div>
    </RouterLink>
    
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
import { AppAgentClient, Record, AgentPubKey, EntryHash, ActionHash } from '@holochain/client';
import { Post } from './types';
import '@material/mwc-circular-progress';
import '@material/mwc-icon-button';
import '@material/mwc-snackbar';
import { Snackbar } from '@material/mwc-snackbar';
import { deserializeHash, serializeHash } from '@holochain-open-dev/utils';
import dayjs from 'dayjs';
import relativeTime from 'dayjs/plugin/relativeTime';
dayjs.extend(relativeTime);

export default defineComponent({
  components: {
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
  data(): { record: Record | undefined; loading: boolean; editing: boolean; } {
    return {
      record: undefined,
      loading: true,
      editing: false,
    }
  },
  computed: {
    post() {
      if (!this.record) return undefined;
      return decode((this.record.entry as any).Present.entry) as Post;
    },
    authorHash() {
      if (!this.record) return undefined;

      return serializeHash(this.record.signed_action.hashed.content.author);
    },
    dateRelative() {
      if(!this.record?.signed_action.hashed.content.timestamp) return;

      return dayjs(this.record.signed_action.hashed.content.timestamp/1000).fromNow();
    },
    postHashString() {      
      return serializeHash(this.postHash);
    },
    dnaHashString() {
      return serializeHash(this.dnaHash);
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

      this.record = await this.client.callZome({
        cell_id: [this.dnaHash, this.client.myPubKey],
        cap_secret: null,
        zome_name: 'posts',
        fn_name: 'get_post',
        payload: this.postHash,
      });

      this.loading = false;
    }
  },
  setup() {
    const client = (inject('client') as ComputedRef<AppAgentClient>).value;
    return {
      client
    };
  },
})
</script>

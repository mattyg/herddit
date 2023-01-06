<template>
  <div v-if="!loading && listing" class="inline-block">
    <RouterLink :to="`/herds/${listingHashString}`" :title="listing.description" class="no-underline hover:underline cursor-pointer">{{ listing.title }}</RouterLink>      
  </div>
</template>

<script lang="ts">
import { defineComponent, inject, ComputedRef, PropType } from 'vue';
import { decode } from '@msgpack/msgpack';
import { AppAgentClient, Record, AgentPubKey, EntryHash, ActionHash } from '@holochain/client';
import { Listing } from './types';
import '@material/mwc-circular-progress';
import '@material/mwc-icon-button';
import '@material/mwc-snackbar';
import { Snackbar } from '@material/mwc-snackbar';
import EditListing from './EditListing.vue';
import { deserializeHash, serializeHash } from '@holochain-open-dev/utils';

export default defineComponent({
  components: {
    EditListing
  },
  props: {
    listingHash: {
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
    listing() {
      if (!this.record) return undefined;
      return decode((this.record.entry as any).Present.entry) as Listing;
    },
    listingHashString() {
      if (!this.listing) return undefined;

      return serializeHash(this.listingHash);
    }
  },
  async mounted() {
    await this.fetchListing();
  },
  methods: {
    async fetchListing() {
      this.loading = true;
      this.record = undefined;

      this.record = await this.client.callZome({
        cap_secret: null,
        role_name: 'herd',
        zome_name: 'directory',
        fn_name: 'get_listing',
        payload: this.listingHash,
      });
      
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
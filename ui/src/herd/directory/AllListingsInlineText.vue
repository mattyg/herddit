<template>
  <div v-if="!loading" class="flex flow-row">
    <div v-if="hashes && hashes.length > 0" class="space-x-2">
      <ListingLink 
        v-for="hash in hashes" 
        :listingHash="hash"
        @listing-deleted="fetchListing()" />
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, inject, ComputedRef, PropType } from 'vue';
import { decode } from '@msgpack/msgpack';
import { AppAgentClient, Record, AgentPubKey, EntryHash, ActionHash } from '@holochain/client';
import '@material/mwc-circular-progress';
import ListingLink from './ListingLink.vue';

export default defineComponent({
  components: {
    ListingLink
  },
  props: {
    dnaHash: {
      type: Object as PropType<Uint8Array> | undefined,
      required: false
    }
  },
  data(): { hashes: Array<ActionHash> | undefined; loading: boolean; error: any } {
    return {
      hashes: undefined,
      loading: true,
      error: undefined
    }
  },
  async mounted() {
    await this.fetchListing();
  },
  methods: {
    async fetchListing() {
      this.loading = true;

      const cellArgs = this.dnaHash ? {
        cell_id: [this.dnaHash, this.client.myPubKey]
      } : {
        role_name: 'herd'
      };

      try {
        this.hashes = await this.client.callZome({
          ...cellArgs,
          zome_name: 'directory',
          fn_name: 'get_all_listings',
          payload: undefined
        });
      } catch (e) {
        this.error = e;
      }
      this.loading = false;
    }
  },
  setup() {
    const client = (inject('client') as ComputedRef<AppAgentClient>).value;
    return {
      client,
    };
  },
})
</script>

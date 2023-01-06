<template>
  <div v-if="loading" style="display: flex; flex: 1; align-items: center; justify-content: center">
    <mwc-circular-progress indeterminate></mwc-circular-progress>
  </div>

  <div v-else style="display: flex; flex-direction: column">
    <span v-if="error">Error fetching the listings: {{error.data.data}}.</span>
    <div v-else-if="hashes && hashes.length > 0">
      <ListingDetail 
        v-for="hash in hashes" 
        :listing-hash="hash"
        @listing-deleted="fetchListing()"
        style="margin-bottom: 8px">
      </ListingDetail>
    </div>
    <span v-else>No listings found.</span>
  </div>

</template>

<script lang="ts">
import { defineComponent, inject, ComputedRef } from 'vue';
import { decode } from '@msgpack/msgpack';
import { AppAgentClient, Record, AgentPubKey, EntryHash, ActionHash } from '@holochain/client';
import '@material/mwc-circular-progress';
import ListingDetail from './ListingDetail.vue';

export default defineComponent({
  components: {
    ListingDetail
  },
  props: { 
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
      try {
        this.hashes = await this.client.callZome({
          cap_secret: null,
          role_name: 'herd',
          zome_name: 'directory',
          fn_name: 'get_all_listings',
          payload: null,
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

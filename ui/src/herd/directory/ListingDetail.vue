<template>
  <div v-if="!loading">
    <div v-if="editing">
      <EditListing
        :original-listing-hash="listingHash"
        :current-record="record!"
        @listing-updated="editing = false; fetchListing();"
        @edit-canceled="editing = false"
        style="display: flex; flex: 1;"
      ></EditListing>
    </div>
    <div v-else-if="record" style="display: flex; flex-direction: column">
      <div style="display: flex; flex-direction: row">
        <span style="font-size: 18px; flex: 1;">Listing</span>

        <mwc-icon-button style="margin-left: 8px" icon="edit" @click="editing = true"></mwc-icon-button>
        <mwc-icon-button style="margin-left: 8px" icon="delete" @click="deleteListing()"></mwc-icon-button>
      </div>

      <div style="display: flex; flex-direction: row; margin-bottom: 16px;">
	<span><strong>Description</strong></span>
 	<span style="white-space: pre-line">{{  listing?.description }} </span>
      </div>

    </div>
    
    <span v-else>The requested listing was not found.</span>
  </div>

  <div v-else style="display: flex; flex: 1; align-items: center; justify-content: center">
    <mwc-circular-progress indeterminate></mwc-circular-progress>
  </div>

  <mwc-snackbar ref="delete-error" leading>
  </mwc-snackbar>
</template>

<script lang="ts">
import { defineComponent, inject, ComputedRef } from 'vue';
import { decode } from '@msgpack/msgpack';
import { AppAgentClient, Record, AgentPubKey, EntryHash, ActionHash } from '@holochain/client';
import { Listing } from './types';
import '@material/mwc-circular-progress';
import '@material/mwc-icon-button';
import '@material/mwc-snackbar';
import { Snackbar } from '@material/mwc-snackbar';
import EditListing from './EditListing.vue';

export default defineComponent({
  components: {
    EditListing
  },
  props: {
    listingHash: {
      type: Object,
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
    async deleteListing() {
      try {
        await this.client.callZome({
          cap_secret: null,
          role_name: 'herd',
          zome_name: 'directory',
          fn_name: 'delete_listing',
          payload: this.listingHash,
        });
        this.$emit('listing-deleted', this.listingHash);
        this.fetchListing();
      } catch (e: any) {
        const errorSnackbar = this.$refs['delete-error'] as Snackbar;
        errorSnackbar.labelText = `Error deleting the listing: ${e.data.data}`;
        errorSnackbar.show();
      }
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

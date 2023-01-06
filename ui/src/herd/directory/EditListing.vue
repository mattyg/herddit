<template>
  <mwc-snackbar ref="update-error"></mwc-snackbar>

  <div style="display: flex; flex-direction: column">
    <span style="font-size: 18px">Edit Listing</span>
      <div style="margin-bottom: 16px">
      <mwc-textarea outlined label="Description" :value="description" @input="description = $event.target.value" required></mwc-textarea>
      </div>



    <div style="display: flex; flex-direction: row">
      <mwc-button
        outlined
        label="Cancel"
        @click="$emit('edit-canceled')"
        style="flex: 1; margin-right: 16px;"
      ></mwc-button>
      <mwc-button 
        raised
        label="Save"
        :disabled="!isListingValid"
        @click="updateListing"
        style="flex: 1;"
      ></mwc-button>
    </div>
  </div>
</template>
<script lang="ts">
import { defineComponent, inject, ComputedRef } from 'vue';
import { AppAgentClient, Record, AgentPubKey, EntryHash, ActionHash } from '@holochain/client';
import { Listing } from './types';
import '@material/mwc-button';
import '@material/mwc-snackbar';
import { decode } from '@msgpack/msgpack';
import { Snackbar } from '@material/mwc-snackbar';
import '@material/mwc-textarea';

export default defineComponent({
  data(): {
    description: string;
  } {
    const currentListing = decode((this.currentRecord.entry as any).Present.entry) as Listing;
    return { 
      description: currentListing.description,
    }
  },
  props: {
    originalListingHash: {
      type: null,
      required: true,
    },
    currentRecord: {
      type: Object,
      required: true
    }
  },
  computed: {
    currentListing() {
      return decode((this.currentRecord.entry as any).Present.entry) as Listing;
    },
    isListingValid() {
      return true && this.description !== undefined;
    },
  },
  methods: {
    async updateListing() {

      const listing: Listing = { 
        title: this.title!,
        description: this.description!,
        dna: this.currentListing.dna,
      };

      try {
        const updateRecord: Record = await this.client.callZome({
          cap_secret: null,
          role_name: 'herd',
          zome_name: 'directory',
          fn_name: 'update_listing',
          payload: {
            original_listing_hash: this.originalListingHash,
            previous_listing_hash: this.currentRecord.signed_action.hashed.hash,
            updated_listing: listing
          }
        });
        this.$emit('listing-updated', updateRecord.signed_action.hashed.hash);
      } catch (e: any) {
        const errorSnackbar = this.$refs['update-error'] as Snackbar;
        errorSnackbar.labelText = `Error updating the listing: ${e.data.data}`;
        errorSnackbar.show();
      }
    },
  },
  emits: ['listing-updated', 'edit-canceled'],
  setup() {
    const client = (inject('client') as ComputedRef<AppAgentClient>).value;
    return {
      client,
    };
  },
})
</script>

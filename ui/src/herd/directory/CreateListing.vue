<template>
  <mwc-snackbar ref="create-error"></mwc-snackbar>

  <div style="display: flex; flex-direction: column">
    <span style="font-size: 18px">Create Listing</span>
  
    <div style="margin-bottom: 16px">
      <mwc-textarea outlined label="Title" @input="title = $event.target.value" required></mwc-textarea>
    </div>

    <div style="margin-bottom: 16px">
      <mwc-textarea outlined label="Description" @input="description = $event.target.value" required></mwc-textarea>
    </div>
  
    <mwc-button 
      raised
      label="Create Listing"
      :disabled="!isListingValid"
      @click="createListing"
    ></mwc-button>
  </div>
</template>
<script lang="ts">
import { defineComponent, inject, ComputedRef } from 'vue';
import { AppAgentClient, Record, AgentPubKey, EntryHash, ActionHash } from '@holochain/client';
import { Listing } from './types';
import '@material/mwc-button';
import '@material/mwc-icon-button';
import '@material/mwc-snackbar';
import { Snackbar } from '@material/mwc-snackbar';

import '@material/mwc-textarea';
export default defineComponent({
  data(): {
    title: string | undefined;
    description: string | undefined;
  } {
    return { 
      title: undefined,
      description: undefined,
    }
  },
  props: {
    dna: {
      type: null,
      required: true
    },
  },
  computed: {
    isListingValid() {
      return true && this.description !== undefined
    },
  },
  methods: {
    async createListing() {
      const listing: Listing = { 
        title: this.title!,
        description: this.description!,
        dna: this.dna!,
      };

      try {
        const record: Record = await this.client.callZome({
          cap_secret: null,
          role_name: 'herd',
          zome_name: 'directory',
          fn_name: 'create_listing',
          payload: listing,
        });
        this.$emit('listing-created', record.signed_action.hashed.hash);
      } catch (e: any) {
        const errorSnackbar = this.$refs['create-error'] as Snackbar;
        errorSnackbar.labelText = `Error creating the listing: ${e.data.data}`;
        errorSnackbar.show();
      }
    },
  },
  emits: ['listing-created'],
  setup() {
    const client = (inject('client') as ComputedRef<AppAgentClient>).value;
    return {
      client,
    };
  },
})
</script>

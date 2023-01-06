<template>
  <mwc-snackbar ref="create-error"></mwc-snackbar>

  <div class="w-full flex justify-center">
    <div class="w-full md:max-w-screen-lg mx-4">
      <span class="text-2xl mb-8">Create Herd</span>
    
      <div class="mb-4">
        <mwc-textfield class="w-full" outlined label="Title" @input="title = $event.target.value" required></mwc-textfield>
      </div>
    
      <mwc-button 
        raised
        label="Create Herd"
        :disabled="!isHerdValid"
        @click="createHerd"
      ></mwc-button>
    </div>
  </div>
</template>
<script lang="ts">
import { defineComponent, inject, ComputedRef } from 'vue';
import { AppAgentClient, Record, AgentPubKey, EntryHash, ActionHash, InstalledCell } from '@holochain/client';
import { Listing } from '../directory/types';
import '@material/mwc-button';
import '@material/mwc-icon-button';
import '@material/mwc-snackbar';
import { Snackbar } from '@material/mwc-snackbar';
import { generateSlug } from "random-word-slugs";

import '@material/mwc-textarea';
import { deserializeHash, serializeHash, timestampToMillis } from '@holochain-open-dev/utils';
export default defineComponent({
  data(): {
    title: string | undefined;
    description: string | undefined;
  } {
    return { 
      title: undefined,
      description: '',
    }
  },
  props: {
  },
  computed: {
    isHerdValid() {
      return true && this.title !== undefined
    },
  },
  methods: {
    async createHerd() {
      try {
        const timestamp = Math.floor(new Date() / 1000);
        const network_seed = generateSlug(5);

        const cloneCell: InstalledCell = await this.client.createCloneCell({
          role_name: 'herd',
          modifiers: {
            network_seed,
            properties: {
              title: this.title!
            },
          },
        });
        console.log('Created clone cell', cloneCell);

        const listing: Listing = { 
          title: this.title!,
          description: this.description!,
          network_seed,
          dna: cloneCell.cell_id[0]
        };

        const record: Record = await this.client.callZome({
          cap_secret: null,
          role_name: 'herd',
          zome_name: 'directory',
          fn_name: 'create_listing',
          payload: listing,
        });

        this.$emit('listing-created', record.signed_action.hashed.hash);
        this.$router.push(`/herds/${serializeHash(record.signed_action.hashed.hash)}`);
      } catch (e: any) {
        const errorSnackbar = this.$refs['create-error'] as Snackbar;
        errorSnackbar.labelText = `Error creating the herd: ${e.data.data}`;
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

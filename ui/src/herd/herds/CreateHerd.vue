<template>
  <div class="w-full flex justify-center my-12">
    <div class="w-full md:max-w-screen-lg mx-4">
      <div class="text-2xl mb-4">Gather New Herd</div>
      <div class="mb-8">
        <mwc-textfield class="w-full" outlined label="Title" @input="title = $event.target.value" required></mwc-textfield>
      </div>
    
      <button 
        class="btn btn-primary"
        :disabled="!isHerdValid || creatingHerd"
        @click="createHerd"
      >Gather New Herd</button>
    </div>
  </div>
</template>
<script lang="ts">
import { defineComponent, inject, ComputedRef, PropType } from 'vue';
import { AppAgentClient, Record, AgentPubKey, EntryHash, ActionHash, InstalledCell, encodeHashToBase64, ClonedCell } from '@holochain/client';
import { Listing } from '../directory/types';
import '@material/mwc-button';
import '@material/mwc-icon-button';
import '@material/mwc-snackbar';
import { generateSlug } from "random-word-slugs";
import '@material/mwc-textarea';
import { toast } from 'vue3-toastify';
export default defineComponent({
  data(): {
    title: string | undefined;
    description: string | undefined;
    creatingHerd: boolean;
  } {
    return { 
      title: undefined,
      description: '',
      creatingHerd: false,
    }
  },
  computed: {
    isHerdValid() {
      return true && this.title !== undefined
    },
  },
  methods: {
    async createHerd() {
      this.creatingHerd = true;
      try {
        const network_seed = generateSlug(5);

        const cloneCell: ClonedCell = await this.client.createCloneCell({
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
        toast.success(`Cloned cell for herd "${this.title}"`);
        this.$router.push(`/herds/${encodeHashToBase64(record.signed_action.hashed.hash)}`);
      } catch (e: any) {
        toast.error(`Error creating the herd: ${e.data.data}`);
      }

      this.creatingHerd = false;
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

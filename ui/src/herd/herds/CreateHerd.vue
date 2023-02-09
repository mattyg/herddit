<template>
  <div class="w-full flex justify-center my-12">
    <article class="prose w-full md:max-w-screen-lg mx-4">
      <h1>Gather a Herd</h1>
      <mwc-textfield class="w-full mb-8" outlined label="Title" @input="title = $event.target.value" required></mwc-textfield>
      <mwc-textarea class="w-full mb-8" :rows="10" outlined label="Description" @input="description = $event.target.value"></mwc-textarea>
      <div class="flex flex-row justify-start items-center space-x-4 mb-8 cursor-pointer" @click="public = !public">
        <mwc-checkbox class="w-full w-8" :checked="public"></mwc-checkbox>
        <div class="flex-1">Announce at The Watering Hole</div>
      </div>

      <div>
      <button 
        class="btn btn-primary"
        :disabled="!isHerdValid || creatingHerd"
        @click="createHerd"
      >Gather a Herd</button>
    </div>
  </article>
  </div>
  <HerdPasswordModal :visible="showPasswordModal" :text="password" />
</template>
<script lang="ts">
import { defineComponent, inject, ComputedRef, PropType, ref } from 'vue';
import { AppAgentClient, Record, AgentPubKey, EntryHash, ActionHash, InstalledCell, encodeHashToBase64, ClonedCell, randomByteArray } from '@holochain/client';
import { toast } from 'vue3-toastify';
import HerdPasswordModal from './HerdPasswordModal.vue';
import { title } from 'process';
import { Listing } from '../directory/types';

export default defineComponent({
  components: {
    HerdPasswordModal
  },
  data(): {
    title: string | undefined;
    description: string | undefined;
    public: boolean;
    creatingHerd: boolean;
    password: string | undefined;
    showPasswordModal: boolean;
  } {
    return { 
      title: undefined,
      description: '',
      public: true,
      creatingHerd: false,
      password: undefined,
      showPasswordModal: false,
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
        // Generate random network seed
        const entropy = await randomByteArray(128)
        const network_seed = encodeHashToBase64(entropy);

        // Prepare herd cloned cell
        const cloneCellParams = {
          role_name: 'herd',
          modifiers: {
            network_seed: network_seed,
            properties: {
              title: this.title!
            },
          },
        };

        // Clone herd cell
        const cloneCell: ClonedCell = await this.client.createCloneCell(cloneCellParams);
        toast.success(`Cloned cell for herd "${this.title}"`);

        // Publish Listing about cell
        const listing: Listing = { 
            title: this.title!,
            description: this.description!,
            network_seed,
            dna: cloneCell.cell_id[0]
        };

        if(this.public) {   
          await this.createListing(listing);
        } else {
          await this.createPrivateListing(listing);
        }
      } catch (e: any) {
        toast.error(`Error creating the herd: ${e.data.data}`);
      }

      this.creatingHerd = false;
    },

  async createListing(listing: Listing) {
    // Publish listing to public directory 
    try {
      const listing_ah: ActionHash = await this.client.callZome({
          role_name: 'herd',
          zome_name: 'directory',
          fn_name: 'create_listing',
          payload: listing,
        });

      this.$router.push(`/herds/${encodeHashToBase64(listing_ah)}`);
    } catch (e: any) {
      console.log('error', e);
      toast.error('Error creating private listing', e.data.data);
    }
  },
  async createPrivateListing(listing: Listing) {
    // Publish listing to private entry
    try {
      const listing_ah: ActionHash = await this.client.callZome({
        role_name: 'herd',
        zome_name: 'directory',
        fn_name: 'create_private_listing_idempotent',
        payload: listing,
      });
      console.log('created private listing', listing_ah)
    } catch (e: any) {
      console.log('error', e);
      toast.error('Error creating private listing', e.data.data);
    }
    // Encode listing into string to use a secret herd password
    try {
      const listing_babble = await this.client.callZome({
        role_name: 'herd',
        zome_name: 'directory',
        fn_name: 'listing_to_bubble_babble',
        payload: listing,
      });
      console.log('words', listing_babble);
      
      this.password = listing_babble;
      this.showPasswordModal = true;
    } catch (e: any) {
      console.log('error', e);
      toast.error('Error converting data to mnemonic', e);
    }
  },
  },
  emits: ['listing-created'],
  setup() {
    const client = (inject('client') as ComputedRef<AppAgentClient>).value;
    const checkboxModal = ref(null);

    return {
      checkboxModal,
      client,
    };
  },
})
</script>

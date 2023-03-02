<template>
  <div class="w-full flex justify-center my-12">
    <article class="prose w-full  md:max-w-screen-lg mx-4">
      <h1>Gather a Herd</h1>

      <div class="form-control my-4 font-bold">
        <label class="label">
          <span class="label-text">Herd Name</span>
        </label>
        <input
          v-model="title"
          type="text"
          class="input w-full input-lg input-bordered"
        >
        <label class="label">
          <span class="label-text">
            Letters, numbers and underscores only
          </span>
        </label>
      </div>
      <div class="form-control my-4 flex flex-row justify-start items-center space-x-4">
        <label class="label cursor-pointer">
          <input
            v-model="publish"
            type="checkbox"
            class="checkbox checkbox-lg"
          >
        </label>
        <span class="label-text text-2xl">Share at the Watering Hole</span> 
      </div>
        
      <div class="form-control my-12">
        <button 
          class="btn btn-primary"
          :disabled="!isHerdValid || creatingHerd"
          @click="createHerd"
        >
          Gather a Herd
        </button>
      </div>
    </article>
  </div>
  <HerdPasswordModal
    :visible="showPasswordModal"
    :text="password"
  />
</template>
<script lang="ts">
import { defineComponent, inject, ComputedRef, ref } from 'vue';
import { AppAgentClient, ActionHash, encodeHashToBase64, ClonedCell, randomByteArray } from '@holochain/client';
import { toast } from 'vue3-toastify';
import HerdPasswordModal from './HerdPasswordModal.vue';
import { Listing } from '../directory/types';

export default defineComponent({
  components: {
    HerdPasswordModal
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
  data(): {
    title: string;
    description: string;
    publish: boolean;
    creatingHerd: boolean;
    password: string | undefined;
    showPasswordModal: boolean;
  } {
    return { 
      title: '',
      description: '',
      publish: true,
      creatingHerd: false,
      password: undefined,
      showPasswordModal: false,
    }
  },
  computed: {
    isHerdValid() {
      return this.title.length > 0
    },
  },
  watch: {
    title(val) {
      this.title = val.replace(/\W/g, "");
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

        // Clone herd cell
        const cloneCell: ClonedCell = await this.client.createCloneCell({
          role_name: 'herd',
          name: this.title,
          modifiers: {
            network_seed: network_seed,
            properties: {
              title: this.title
            },
          },
        });

        // Publish Listing about cell
        const listing: Listing = { 
            title: this.title,
            description: this.description,
            network_seed,
            dna: cloneCell.cell_id[0]
        };

        if(this.publish) {   
          await this.createListing(listing);
        } else {
          await this.createPrivateListing(listing);
        }

        toast.success(`Cloned cell for herd "${this.title}"`);
      } catch (e: any) {
        toast.error(`Error creating the herd: ${e.data.data}`);
      }

      this.creatingHerd = false;
    },

  async createListing(listing: Listing) {
    // Publish listing to public directory 
    try {
      const listing_ah: ActionHash = await this.client.callZome({
          role_name: 'directory',
          zome_name: 'directory',
          fn_name: 'create_listing',
          payload: listing,
        });

      this.$router.push(`/herds/${encodeHashToBase64(listing_ah)}`);
    } catch (e: any) {
      console.log('error', e);
      toast.error('Error creating listing', e.data.data);
    }
  },
  async createPrivateListing(listing: Listing) {
    // Publish listing to private entry
    try {
      await this.client.callZome({
        role_name: 'directory',
        zome_name: 'directory',
        fn_name: 'create_private_listing_idempotent',
        payload: listing,
      });
    } catch (e: any) {
      console.log('error', e);
      toast.error('Error creating private listing', e.data.data);
    }
    // Encode listing into string to use a secret herd password
    try {
      const listing_babble = await this.client.callZome({
        role_name: 'directory',
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
})
</script>

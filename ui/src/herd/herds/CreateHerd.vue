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
            v-model="isPublicHerd"
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
<script lang="ts" setup>
import { defineEmits, inject, computed, ref, ComputedRef, watch } from 'vue';
import { AppAgentClient, ActionHash, encodeHashToBase64, ClonedCell, randomByteArray } from '@holochain/client';
import { toast } from 'vue3-toastify';
import HerdPasswordModal from './HerdPasswordModal.vue';
import { Listing } from '../directory/types';
import { useRouter } from 'vue-router';

defineEmits(['listing-created']);
const router = useRouter();

const client = (inject('client') as ComputedRef<AppAgentClient>).value;

const title = ref('');
const description = ref('');
const isPublicHerd = ref(true);
const creatingHerd = ref(false);
const password = ref();
const showPasswordModal = ref(false);

const isHerdValid = computed(() => title.value.length > 0);

watch(title, (newTitle) => {
  console.log('watch called');
  title.value = newTitle.replace(/\W/g, "");
});

const createHerd = async () => {
  creatingHerd.value = true;
  try {
    // Generate random network seed
    const entropy = await randomByteArray(128)
    const network_seed = encodeHashToBase64(entropy);

    // Prepare herd cloned cell

    // Clone herd cell
    const cloneCell: ClonedCell = await client.createCloneCell({
      role_name: 'herd',
      modifiers: {
        network_seed: network_seed,
        properties: {
          title: title.value
        },
      },
    });

    // Publish Listing about cell
    const listing: Listing = { 
        title: title.value,
        description: description.value,
        network_seed,
        dna: cloneCell.cell_id[0]
    };

    if(isPublicHerd.value) {   
      await createListing(listing);
    } else {
      await createPrivateListing(listing);
    }

    toast.success(`Cloned cell for herd "${title.value}"`);
  } catch (e: any) {
    toast.error(`Error creating the herd: ${e.data.data}`);
  }

  creatingHerd.value = false;
};

const createListing = async (listing: Listing) => {
  // Publish listing to public directory 
  try {
    const listing_ah: ActionHash = await client.callZome({
        role_name: 'directory',
        zome_name: 'directory',
        fn_name: 'create_listing',
        payload: listing,
      });

    router.push(`/herds/${encodeHashToBase64(listing_ah)}`);
  } catch (e: any) {
    console.log('error', e);
    toast.error('Error creating listing', e.data.data);
  }
};

const createPrivateListing = async (listing: Listing) => {
  // Publish listing to private entry
  try {
    await client.callZome({
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
    const listing_babble = await client.callZome({
      role_name: 'directory',
      zome_name: 'directory',
      fn_name: 'listing_to_bubble_babble',
      payload: listing,
    });
    
    password.value = listing_babble;
    showPasswordModal.value = true;
  } catch (e: any) {
    console.log('error', e);
    toast.error('Error converting data to mnemonic', e);
  }
};
</script>

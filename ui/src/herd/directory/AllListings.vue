<template>
  <div
    v-if="loading"
    class="h-screen flex flex-col flex-1 justify-center items-center bg-base-100 base-neutral-content"
  >
    <BaseSpinner>Finding Herds...</BaseSpinner>
  </div>
  <div
    v-else
    class="flex justify-center w-full flex-wrap"
  >
    <div
      v-if="installedListingHashes.length == 0 && notInstalledListingHashes.length === 0 && showEmptyMessage"
      class="flex flex-col justify-center items-center space-y-8"
    >
      <div class="text-2xl my-16">
        All seems quiet at the watering hole...
      </div>

      <RouterLink
        :to="`/herds/create`"
        class="btn btn-primary btn-xl"
      >
        Gather a Herd
      </RouterLink>
    </div>

    <div
      v-if="installedListingHashes.length > 0"
      class="mx-16 my-8 w-full"
    >
      <h2 class="text-4xl font-medium text-center my-8 pb-2 border-solid border-b-2 border-accent">
        My Herds
      </h2>

      <div
        class="w-full flex flex-wrap justify-center items-center"
      >
        <ListingLink 
          v-for="hash in installedListingHashes" 
          :key="encodeHashToBase64(hash)"
          :listing-hash="hash"
          class="inline-block px-4 my-4 mx-2 truncate flex justify-center"
        />
      </div>
    </div>

    <div
      v-if="notInstalledListingHashes.length > 0"
      class="mx-16 my-8 w-full"
    >
      <h2 class="text-4xl font-medium text-center my-8 pb-2 border-solid border-b-2 border-accent">
        Neighbor Herds
      </h2>

      <div
        class="w-full flex flex-wrap justify-center items-center"
      >
        <ListingLink 
          v-for="hash in notInstalledListingHashes" 
          :key="encodeHashToBase64(hash)"
          :listing-hash="hash"
          class="inline-block px-4 my-4 mx-2 truncate flex justify-center"
        />
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { inject, ComputedRef, PropType, defineProps, ref, watch } from 'vue';
import { AppAgentClient, ActionHash, encodeHashToBase64 } from '@holochain/client';
import ListingLink from './ListingLink.vue';
import BaseSpinner from '../../components/BaseSpinner.vue';
import { toast } from 'vue3-toastify';

const props = defineProps({
  dnaHash: {
      type: Object as PropType<Uint8Array> | undefined,
      default: undefined,
    },
    showEmptyMessage: {
      type: Boolean,
      default: false,
    },
    showPrivate: {
      type: Boolean,
      default: true,
    }
});

const installedListingHashes = ref<Array<ActionHash>>([]);
const notInstalledListingHashes = ref<Array<ActionHash>>([]);
const loading = ref<boolean>(true);

const client = (inject('client') as ComputedRef<AppAgentClient>).value;

const fetchListings = async () =>  {
  console.log('fetching listings');
  const cellArgs = props.dnaHash ? {
    cell_id: [props.dnaHash, client.myPubKey]
  } : {
    role_name: 'directory'
  };

  try {
    const input = {
      include_public: true,
      include_private: props.showPrivate,
    };

    // @ts-ignore
    const allListings = await client.callZome({
      ...cellArgs,
      zome_name: 'directory',
      fn_name: 'get_listings',
      payload: input
    });

    // Compare listings to installed herd cells
    const appInfo = await client.appInfo();
    
    // @ts-ignore
    const installedHerdDnaHashes = appInfo.cell_info.herd.filter(cell => cell.cloned && cell.cloned.enabled).map((cell) => cell.cloned.cell_id[0]);
    const installedHerdDnaHashStrings = installedHerdDnaHashes.map(h => encodeHashToBase64(h));
    
    const installedListings = allListings.filter(([listingDnaHash, ]: [Uint8Array, ]) => installedHerdDnaHashStrings.includes(encodeHashToBase64(listingDnaHash)));
    const notInstalledListings = allListings.filter(([listingDnaHash, ]: [Uint8Array, ]) => !installedHerdDnaHashStrings.includes(encodeHashToBase64(listingDnaHash)));

    installedListingHashes.value = installedListings.map((l: [Uint8Array, Uint8Array]) => l[1]);
    notInstalledListingHashes.value = notInstalledListings.map((l: [Uint8Array, Uint8Array]) => l[1]);
  } catch (e: any) {
    toast.error('Failed to get listings', e.data.data)
  }
  loading.value = false;
};

fetchListings();

watch(props, () => {
  fetchListings();
});
</script>

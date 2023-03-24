<template>
  <div
    v-if="!allListings && !appInfo"
    class="h-screen flex flex-col flex-1 justify-center items-center bg-base-100 base-neutral-content"
  >
    <BaseSpinner>Finding Herds...</BaseSpinner>
  </div>
  <div
    v-else
    class="flex justify-center w-full flex-wrap"
  >
    <div
      v-if="installedListingHashes.length === 0 && notInstalledListingHashes.length === 0 && showEmptyMessage"
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
          v-for="(hash,i) in installedListingHashes" 
          :key="i"
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
          v-for="(hash,i) in notInstalledListingHashes" 
          :key="i"
          :listing-hash="hash"
          class="inline-block px-4 my-4 mx-2 truncate flex justify-center"
        />
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { inject, ComputedRef, defineProps, watch, computed } from 'vue';
import { AppAgentClient, encodeHashToBase64 } from '@holochain/client';
import ListingLink from './ListingLink.vue';
import BaseSpinner from '../../components/BaseSpinner.vue';
import { useRequest } from 'vue-request';
import { toast } from 'vue3-toastify';

const props = defineProps<{
  dnaHash: Uint8Array,
  showEmptyMessage?: boolean,
  showPrivate?: boolean,
}>();

const client = (inject('client') as ComputedRef<AppAgentClient>).value;

const fetchListings = async () =>  {
  const cellArgs = props.dnaHash ? {
    cell_id: [props.dnaHash, client.myPubKey]
  } : {
    role_name: 'directory'
  };

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
    
  return allListings;
};

const fetchAppInfo = async () => {
  const res = await client.appInfo();

  return res;
}

const installedHerdDnaHashStrings = computed((): Array<string> => {
  if(!appInfo.value) return [];

  // @ts-ignore
  const hashStrings = appInfo.value.cell_info.herd.filter(cell => cell.cloned && cell.cloned.enabled).map((cell) => encodeHashToBase64(cell.cloned.cell_id[0]));
  return hashStrings;
});

const installedListingHashes = computed((): Array<Uint8Array> => {
  if(!allListings.value || !appInfo.value) return [];
  
  const hashes: Uint8Array[] = allListings.value
    .filter(([listingDnaHash, ]: [Uint8Array, ]) => installedHerdDnaHashStrings.value.includes(encodeHashToBase64(listingDnaHash)))
    .map(([, actionHash]: [Uint8Array, Uint8Array]) => actionHash);
  return hashes;
});

const notInstalledListingHashes = computed((): Array<Uint8Array> => {
  if(!allListings.value || !appInfo.value) return [];
  
  const hashes: Uint8Array[]  = allListings.value
    .filter(([listingDnaHash, ]: [Uint8Array, ]) => !installedHerdDnaHashStrings.value.includes(encodeHashToBase64(listingDnaHash)))
    .map(([, actionHash]: [Uint8Array, Uint8Array]) => actionHash);
  return hashes;
});

const { data: allListings, run: runFetchListings } = useRequest(fetchListings, {
  pollingInterval: 1000,
  onError: (e: any) => {
    toast.error(`Failed to fetch listings ${e.data.data}`)
  }
});
const { data: appInfo } = useRequest(fetchAppInfo, {
  pollingInterval: 1000,
  onError: (e: any) => {
    toast.error(`Failed to fetch appInfo ${e.data.data}`)
  }
});

watch(props, () => {
  runFetchListings();
});
</script>

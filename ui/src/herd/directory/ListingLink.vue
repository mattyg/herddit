<template>
  <RouterLink
    v-if="listing"
    :to="`/herds/${listingHashString}`"
  >
    <div class="flex flex-row items-center space-x-1">
      <div
        v-if="isPrivate"
        class="text-3xl mr-2"
        title="This herd is not published to the watering hole"
      >
        <svg
          class="h-8 w-8"
          xmlns="http://www.w3.org/2000/svg"
          width="256"
          height="256"
          viewBox="0 0 256 256"
        ><path
          fill="currentColor"
          d="M247.3 131.3c-.4.9-10.5 23.3-33.3 43.8a8.6 8.6 0 0 1-5.4 2a8 8 0 0 1-5.9-2.6L101.4 63.1a8.1 8.1 0 0 1 4.6-13.3a132.4 132.4 0 0 1 22-1.8c34.9 0 66.6 13.3 91.7 38.3c18.8 18.9 27.3 37.7 27.6 38.5a8.2 8.2 0 0 1 0 6.5Zm-33.4 79.3a7.9 7.9 0 0 1-.5 11.3a8.2 8.2 0 0 1-5.4 2.1a8 8 0 0 1-5.9-2.6l-22-24.2A128.6 128.6 0 0 1 128 208c-34.9 0-66.6-13.3-91.7-38.3C17.5 150.8 9 132 8.7 131.3a8.2 8.2 0 0 1 0-6.5c.7-1.6 16.3-36 52.6-58.3L42.1 45.4a8 8 0 0 1 11.8-10.8Zm-68.2-51.3l-47.2-51.9a36 36 0 0 0 47.2 51.9Z"
        /></svg>
      </div>
      <span
        class="text-3xl no-underline hover:underline cursor-pointer"
        :title="listing.title"
      >{{ listing.title }}</span>      
    </div>
  </RouterLink>
</template>

<script lang="ts" setup>
import { inject, ComputedRef, defineProps, computed } from 'vue';
import { decode } from '@msgpack/msgpack';
import { AppAgentClient, Record, encodeHashToBase64 } from '@holochain/client';
import { Listing } from './types';
import { useRequest } from 'vue-request';
import { toast } from 'vue3-toastify';

const props = defineProps<{
  listingHash: Uint8Array
}>();

const client = (inject('client') as ComputedRef<AppAgentClient>).value;

const listing = computed(() => {
  if (!record.value) return undefined;
  
  return decode((record.value.entry as any).Present.entry) as Listing;
});

const listingHashString = computed(() => {
  return encodeHashToBase64(props.listingHash);
});

const isPrivate = computed(() => {
  if(!record.value) return;
  
  return Object.keys(record.value.signed_action.hashed.content.entry_type.App.visibility).includes('Private');
})

const fetchListing = async (): Promise<Record> => {
  const res =  await client.callZome({
      cap_secret: null,
      role_name: 'directory',
      zome_name: 'directory',
      fn_name: 'get_listing',
      payload: props.listingHash,
    });
 
  return res;
};

const { data: record } = useRequest(fetchListing, {
  onError: (e: any) => {
    toast.error(`Failed to fetch listing ${e.data.data}`)
  }
});
</script>

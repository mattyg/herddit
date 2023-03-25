<template>
  <div
    v-if="loading"
    class="h-screen flex flex-col flex-1 justify-center items-center space-y-4 bg-base-10 text-base-content"
  >
    <BaseSpinner>
      Tracking down the herd...
    </BaseSpinner>
  </div>

  <div
    v-if="!loading"
    class="w-full h-full"
  >
    <div class="h-16 sticky top-0 w-full flex flex-row justify-between items-center shadow-md space-x-4 px-8 bg-neutral text-neutral-content z-30">
      <div class="flex flex-row justify-start items-center space-x-2">
        <div
          v-if="isPrivate"
          class="text-3xl mr-2"
          title="This herd is not published to the Watering Hole"
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            width="42"
            height="42"
            viewBox="0 0 256 256"
          ><path
            fill="currentColor"
            d="M247.3 131.3c-.4.9-10.5 23.3-33.3 43.8a8.6 8.6 0 0 1-5.4 2a8 8 0 0 1-5.9-2.6L101.4 63.1a8.1 8.1 0 0 1 4.6-13.3a132.4 132.4 0 0 1 22-1.8c34.9 0 66.6 13.3 91.7 38.3c18.8 18.9 27.3 37.7 27.6 38.5a8.2 8.2 0 0 1 0 6.5Zm-33.4 79.3a7.9 7.9 0 0 1-.5 11.3a8.2 8.2 0 0 1-5.4 2.1a8 8 0 0 1-5.9-2.6l-22-24.2A128.6 128.6 0 0 1 128 208c-34.9 0-66.6-13.3-91.7-38.3C17.5 150.8 9 132 8.7 131.3a8.2 8.2 0 0 1 0-6.5c.7-1.6 16.3-36 52.6-58.3L42.1 45.4a8 8 0 0 1 11.8-10.8Zm-68.2-51.3l-47.2-51.9a36 36 0 0 0 47.2 51.9Z"
          /></svg>
        </div>
        <RouterLink
          :to="`/herds/${$route.params.listingHashString}`"
          class="text-3xl"
        >
          h/{{ herdInfo?.title }}
        </RouterLink>
      </div>
      <div class="flex-row justify-between items-center space-x-12">
        <div
          class="btn btn-secondary btn-xs"
          @click="leaveHerd()"
        >
          Leave the Herd
        </div>
        <RouterLink
          :to="`/herds/${$route.params.listingHashString}/posts/create`"
          class="btn btn-primary btn-sm"
        >
          Call to {{ listing?.title }}
        </RouterLink>
      </div>
    </div>

    <div
      v-if="listing && cellInstalled"
      class="w-full h-full flex justify-center"
    >
      <RouterView
        :dna-hash="listing.dna"
        class="w-full md:max-w-screen-lg my-16 z-10"
      />
    </div>
  </div>
</template>

<script lang="ts" setup>
import { AppAgentClient, CellId, decodeHashFromBase64, ClonedCell } from '@holochain/client';
import { decode } from '@msgpack/msgpack';
import { ComputedRef, inject, ref, computed, onMounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { Listing } from '../directory/types';
import { isEqual } from 'lodash';
import { toast } from 'vue3-toastify';
import BaseSpinner from '../../components/BaseSpinner.vue';

const client = (inject('client') as ComputedRef<AppAgentClient>).value;
const route = useRoute();
const router = useRouter();

const record = ref();
const listing = ref();
const herdInfo = ref();
const loading = ref(true);
const cellInstalled = ref(false);

const isPrivate = computed(() => {
  if(route.params.password) return true;
  if(!record.value) return false;
            
  return Object.keys(record.value.signed_action.hashed.content.entry_type.App.visibility).includes('Private');
});

onMounted(async () => {
  await saveListing();
  const cell_id = await installHerdCell();
  await fetchHerdInfo(cell_id);

  loading.value = false;    
})

const saveListing = async () => {
  if(route.params.password) {
    listing.value = await decodePasswordToListing(route.params.password as string);
  } else if(route.params.listingHashString) {
    record.value = await fetchListing(decodeHashFromBase64(route.params.listingHashString as string));
    listing.value = decode((record.value.entry as any).Present.entry) as Listing;
  } else {
    toast.error('Listing password or action hash must be provided');
    router.push('/');
  }
};

const decodePasswordToListing = async (password: string) => {
  try {
    // Deserialize Listing from Bubble Babble string
    const res: Listing = await client.callZome({
        role_name: 'directory',
        zome_name: 'directory',
        fn_name: 'bubble_babble_to_listing',
        payload: password,
    });

    // Save PrivateListing to source chain
    await client.callZome({
        cap_secret: null,
        role_name: 'directory',
        zome_name: 'directory',
        fn_name: 'create_private_listing_idempotent',
        payload: res,
    });

    return res;
  } catch (e: any) {
      toast.error('Invalid Secret Herd-Word', e);
      router.push('/');
  }
};

const fetchListing = async (listingActionHash: Uint8Array) => {  
  const res = await client.callZome({
    role_name: 'directory',
    zome_name: 'directory',
    fn_name: 'get_listing',
    payload: listingActionHash,
  });

  return res;
};

const installHerdCell = async () => {  
    if(!listing.value) return;

    const appInfo = await client.appInfo();
    const cellInfo = appInfo.cell_info.herd.find((cell) => {
      //@ts-ignore
      if(!cell.cloned) return false;

      //@ts-ignore
      return isEqual(cell.cloned.cell_id[0], listing.value.dna);
    });

    // eslint-disable-next-line @typescript-eslint/ban-ts-comment
    // @ts-ignore
    const clonedCell = cellInfo?.cloned;

    if(clonedCell && !clonedCell.enabled) {
        // If cell is disabled, enable it again
        try {
            await client.enableCloneCell({
                clone_cell_id: [listing.value.dna, client.myPubKey]
            });

            cellInstalled.value = true;
            return clonedCell.cell_id;
        }
        catch(e: any) {
            toast.error(`Error enabling cloned cell ${e.data.data}`)
            router.push('/');
        }
    } else if(!clonedCell) {
        // If cell not found, install it
        try {
          const cloneCell: ClonedCell = await client.createCloneCell({
              role_name: 'herd',
              modifiers: {
                  network_seed: listing.value.network_seed,
                  properties: {
                      title: listing.value.title,
                  },
              }
          });

          cellInstalled.value = true;
          return cloneCell.cell_id;
        }
        catch(e: any) {
          toast.error(`Error installing cloned cell ${e.data.data}`)
          router.push('/');
        }
    } else {
        cellInstalled.value = true;
        return clonedCell.cell_id;
    }
};

const fetchHerdInfo = async (cell_id: CellId) => {
  try {
    herdInfo.value = await client.callZome({
      cell_id,
      zome_name: 'herd',
      fn_name: 'get_info',
      payload: null,
    });
  } catch (e: any) {
    toast.error(`Error fetching the herd cell info: ${e.data.data}`);
  }
};

const leaveHerd = async () => {
  if(!listing.value) return;

  try {
    await client.disableCloneCell({
        clone_cell_id: [listing.value.dna, client.myPubKey]
    });
    toast.success(`Disabled cloned cell for herd ${listing.value.title}`);
  } catch (e: any) {
    toast.error(`Error disabling the herd cell: ${e.data.data}`);
  }

  router.push('/');
};
</script>
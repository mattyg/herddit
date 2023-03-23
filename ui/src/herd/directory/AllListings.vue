<template>
  <div
    v-if="!loading"
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
      <h2 class="text-4xl font-bold text-center my-8 pb-2 border-solid border-b-2 border-accent">
        Your Herds
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
      <h2 class="text-4xl font-bold text-center my-8 pb-2 border-solid border-b-2 border-accent">
        Neighbor-Herds
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

<script lang="ts">
import { defineComponent, inject, ComputedRef, PropType } from 'vue';
import { AppAgentClient, ActionHash, encodeHashToBase64 } from '@holochain/client';
import ListingLink from './ListingLink.vue';
import { toast } from 'vue3-toastify';

export default defineComponent({
  components: {
    ListingLink,
  },
  props: {
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
  },
  setup() {
    const client = (inject('client') as ComputedRef<AppAgentClient>).value;
    return {
      client,
    };
  },
  data(): { hashes: Array<ActionHash> | undefined; installedListingHashes:  Array<ActionHash>; notInstalledListingHashes: Array<ActionHash>; loading: boolean; error?: Error } {
    return {
      hashes: undefined,
      installedListingHashes: [],
      notInstalledListingHashes: [],
      loading: true,
    }
  },
  watch: {
    showPrivate() {
      this.fetchListings();
    }
  },
  async mounted() {
    await this.fetchListings();
  },
  methods: {
    async fetchListings() {
      const cellArgs = this.dnaHash ? {
        cell_id: [this.dnaHash, this.client.myPubKey]
      } : {
        role_name: 'directory'
      };

      try {
        const input = {
          include_public: true,
          include_private: this.showPrivate,
        };

        console.log('input', input);
        // @ts-ignore
        const allListings = await this.client.callZome({
          ...cellArgs,
          zome_name: 'directory',
          fn_name: 'get_listings',
          payload: input
        });

        // Compare listings to installed herd cells
        const appInfo = await this.client.appInfo();
        
        // @ts-ignore
        const installedHerdDnaHashes = appInfo.cell_info.herd.filter(cell => cell.cloned && cell.cloned.enabled).map((cell) => cell.cloned.cell_id[0]);
        const installedHerdDnaHashStrings = installedHerdDnaHashes.map(h => encodeHashToBase64(h));
        
        const installedListings = allListings.filter(([listingDnaHash, ]: [Uint8Array, ]) => installedHerdDnaHashStrings.includes(encodeHashToBase64(listingDnaHash)));
        const notInstalledListings = allListings.filter(([listingDnaHash, ]: [Uint8Array, ]) => !installedHerdDnaHashStrings.includes(encodeHashToBase64(listingDnaHash)));

        this.installedListingHashes = installedListings.map((l: [Uint8Array, Uint8Array]) => l[1]);
        this.notInstalledListingHashes = notInstalledListings.map((l: [Uint8Array, Uint8Array]) => l[1]);
      } catch (e: any) {
        toast.error('Failed to get listings', e.data.data)
      }
      this.loading = false;
    },
    encodeHashToBase64(val: Uint8Array) {
      return encodeHashToBase64(val);
    }
  },
})
</script>

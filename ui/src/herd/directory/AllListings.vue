<template>
  <div
    v-if="loading"
    class="h-screen flex flex-col flex-1 justify-center items-center bg-base-100 text-base-content"
  >
    <BaseSpinner>Heading to water...</BaseSpinner>
  </div>
  
  <div
    v-else
    class="flex justify-center w-full"
  >
    <div
      v-if="hashes && hashes.length > 0"
      class="w-full flex flex-wrap justify-start items-center mx-16"
    >
      <ListingLink 
        v-for="hash in hashes" 
        :key="encodeHashToBase64(hash)"
        :listing-hash="hash"
        class="px-8 my-4 inline-block w-1/4 truncate"
      />
    </div>
    <div
      v-else-if="showEmptyMessage"
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
  </div>
</template>

<script lang="ts">
import { defineComponent, inject, ComputedRef, PropType } from 'vue';
import { AppAgentClient, ActionHash, encodeHashToBase64 } from '@holochain/client';
import ListingLink from './ListingLink.vue';
import BaseSpinner from '../../components/BaseSpinner.vue';
import { toast } from 'vue3-toastify';

export default defineComponent({
  components: {
    ListingLink,
    BaseSpinner,
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
  data(): { hashes: Array<ActionHash> | undefined; loading: boolean; error?: Error } {
    return {
      hashes: undefined,
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
        this.hashes = await this.client.callZome({
          ...cellArgs,
          zome_name: 'directory',
          fn_name: 'get_listings',
          payload: input
        });
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

<template>
  <div
    v-if="!loading"
    class="flex justify-center"
  >
    <div>
      <div class="flex justify-center items-center space-x-4 my-8">
        <div class="text-gray-400 font-bold">
          Private Herds
        </div>
        <mwc-switch
          class="text-gray-400 font-bold"
          :selected="showPrivate"
          @click="showPrivate = $event.target.selected"
        />
      </div>
        
      <div
        v-if="hashes && hashes.length > 0"
        clas="flex flex-wrap justify-center items-center"
      >
        <ListingLink 
          v-for="hash in hashes" 
          :key="encodeHashToBase64(hash)"
          :listing-hash="hash"
          class="mx-8 my-4 inline-block"
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
  </div>
</template>

<script lang="ts">
import { defineComponent, inject, ComputedRef, PropType } from 'vue';
import { AppAgentClient, ActionHash, encodeHashToBase64 } from '@holochain/client';
import ListingLink from './ListingLink.vue';
import { toast } from 'vue3-toastify';

export default defineComponent({
  components: {
    ListingLink
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
  },
  setup() {
    const client = (inject('client') as ComputedRef<AppAgentClient>).value;
    return {
      client,
    };
  },
  data(): { hashes: Array<ActionHash> | undefined; loading: boolean; error?: Error; showPrivate: boolean; } {
    return {
      hashes: undefined,
      loading: true,
      showPrivate: true,
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

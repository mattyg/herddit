<template>
  <div v-if="!loading" class="flex justify-center">
    <div>
      <div class="flex justify-center items-center space-x-4 my-8">
        <div class="text-gray-400 font-bold">Private Herds</div>
        <mwc-switch class="text-gray-400 font-bold" :selected="showPrivate" @click="showPrivate = $event.target.selected"></mwc-switch>
      </div>
        
      <div v-if="hashes && hashes.length > 0"  clas="flex flex-wrap justify-center items-center">
          <ListingLink 
            v-for="hash in hashes" 
            :listingHash="hash"
            class="mx-8 my-4 inline-block"
          />
      </div>
      <div v-else-if="showEmptyMessage" class="flex flex-col justify-center items-center space-y-8">
        <div class="text-2xl my-16">All seems quiet at the watering hole...</div>

        <RouterLink :to="`/herds/create`" class="btn btn-primary btn-xl">Gather a Herd</RouterLink>
      </div>
    </div>
</div>
</template>

<script lang="ts">
import { defineComponent, inject, ComputedRef, PropType } from 'vue';
import { decode } from '@msgpack/msgpack';
import { AppAgentClient, Record, AgentPubKey, EntryHash, ActionHash } from '@holochain/client';
import ListingLink from './ListingLink.vue';

export default defineComponent({
  components: {
    ListingLink
  },
  props: {
    dnaHash: {
      type: Object as PropType<Uint8Array> | undefined,
      required: false
    },
    showEmptyMessage: {
      default: false,
    },
  },
  data(): { hashes: Array<ActionHash> | undefined; loading: boolean; error: any; showPrivate: boolean; } {
    return {
      hashes: undefined,
      loading: true,
      error: undefined,
      showPrivate: true,
    }
  },
  watch: {
    showPrivate(newVal) {
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
      } catch (e) {
        this.error = e;
      }
      this.loading = false;
    }
  },
  setup() {
    const client = (inject('client') as ComputedRef<AppAgentClient>).value;
    return {
      client,
    };
  },
})
</script>

<template>
  <RouterLink :to="`/herds/${listingHashString}`" v-if="!loading && listing" :class="{'bg-neutral-200 rounded-full py-1 px-3': isPrivate}">
    <div class="flex flex-row items-center space-x-1">
      <mwc-icon class="text-gray-400 text-lg" v-if="isPrivate">visibility_off</mwc-icon>
      <span class="text-lg no-underline hover:underline cursor-pointer">{{ listing.title }}</span>      
    </div>
  </RouterLink>
</template>

<script lang="ts">
import { defineComponent, inject, ComputedRef, PropType } from 'vue';
import { decode } from '@msgpack/msgpack';
import { AppAgentClient, Record, AgentPubKey, EntryHash, ActionHash, encodeHashToBase64, decodeHashFromBase64 } from '@holochain/client';
import { Listing } from './types';
import '@material/mwc-circular-progress';
import '@material/mwc-icon-button';
import '@material/mwc-icon';
import '@material/mwc-snackbar';
import { Snackbar } from '@material/mwc-snackbar';
import { RouterLink } from 'vue-router';
import { toast } from 'vue3-toastify';

export default defineComponent({
  props: {
    listingHash: {
      type: Object as PropType<Uint8Array>,
      required: true
    }
  },
  data(): { record: Record | undefined; loading: boolean; editing: boolean; } {
    return {
      record: undefined,
      loading: true,
      editing: false,
    }
  },
  computed: {
    listing() {
      if (!this.record) return undefined;
      return decode((this.record.entry as any).Present.entry) as Listing;
    },
    listingHashString() {
      if (!this.listing) return undefined;

      return encodeHashToBase64(this.listingHash);
    },
    isPrivate() {
      if(!this.record) return;
      // @ts-ignore
      return Object.keys(this.record?.signed_action.hashed.content.entry_type.App.visibility).includes('Private');
    }
  },
  async mounted() {
    await this.fetchListing();
  },
  methods: {
    async fetchListing() {
      this.loading = true;
      this.record = undefined;

      try {

      this.record = await this.client.callZome({
        cap_secret: null,
        role_name: 'herd',
        zome_name: 'directory',
        fn_name: 'get_listing',
        payload: this.listingHash,
      });
      
      } catch(e:any) {
        toast.error('Error getting listing: ', e.data.data)
      }
      this.loading = false;
    },
  },
  setup() {
    const client = (inject('client') as ComputedRef<AppAgentClient>).value;
    return {
      client
    };
  },
})
</script>

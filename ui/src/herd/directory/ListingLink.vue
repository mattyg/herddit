<template>
  <RouterLink
    v-if="!loading && listing"
    :to="`/herds/${listingHashString}`"
  >
    <div class="flex flex-row items-center space-x-1">
      <mwc-icon
        v-if="isPrivate"
        class="text-tertiary-content-200 text-lg"
      >
        visibility_off
      </mwc-icon>
      <span class="text-lg no-underline hover:underline cursor-pointer">{{ listing.title }}</span>      
    </div>
  </RouterLink>
</template>

<script lang="ts">
import { defineComponent, inject, ComputedRef, PropType } from 'vue';
import { decode } from '@msgpack/msgpack';
import { AppAgentClient, Record, encodeHashToBase64 } from '@holochain/client';
import { Listing } from './types';
import { toast } from 'vue3-toastify';

export default defineComponent({
  props: {
    listingHash: {
      type: Object as PropType<Uint8Array>,
      required: true
    }
  },
  setup() {
    const client = (inject('client') as ComputedRef<AppAgentClient>).value;
    return {
      client
    };
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
        role_name: 'directory',
        zome_name: 'directory',
        fn_name: 'get_listing',
        payload: this.listingHash,
      });
      
      } catch(e: any) {
        toast.error('Error getting listing: ', e.data.data)
      }
      this.loading = false;
    },
  },
})
</script>

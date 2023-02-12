<template>
  <div
    v-if="profile"
    class="inline-block"
    :class="{'opacity-60': muted}"
  >
    <div 
      class="flex flex-row items-center" 
      :class="{'space-x-3': size === 'lg', 'space-x-2': size === 'md' || size === 'sm'}"
    >
      <img 
        class="rounded-full" 
        :class="{'h-9': size === 'lg', 'h-7': size === 'md', 'h-5': size === 'sm'}"
        :src="profile.fields.avatar" 
      >
      <div 
        :class="{'text-3xl font-bold': size === 'lg', 'text-2xl font-bold': size === 'md', 'text-lg': size === 'sm'}"
      >
        {{ profile.nickname }}
      </div>
      <slot />
    </div>
  </div>
</template>

<script lang="ts">
import { Profile, ProfilesStore } from '@holochain-open-dev/profiles';
import { AppAgentClient } from '@holochain/client';
import { ComputedRef, defineComponent, inject, PropType } from 'vue'
import { toast } from 'vue3-toastify';

export default defineComponent({
  props: {
    agentPubKey: {
      type: Object as PropType<Uint8Array>,
      required: true
    },
    size: {
      type: String,
      default: 'lg',
    },
    muted: {
      type: Boolean,
      default: false
    }
  },
  setup() {
    const client = (inject('client') as ComputedRef<AppAgentClient>).value;
    const profilesStore = (inject('profilesStore') as ComputedRef<ProfilesStore>).value;
    return {
      client,
      profilesStore
    };
  },
  data() : { profile?: Profile } {
    return {
      profile: undefined,
    }
  },
  async mounted() {
    try {
      this.profile = await this.profilesStore.client.getAgentProfile(this.agentPubKey);
    } catch(e: any) {
      toast.error('Error getting agent profile', e);
    }
  },
})
</script>

<style scoped>

</style>
<template>
  <div
    v-if="profile"
    class="relative"
    @mouseenter="() => { detailsVisible = true; }"
    @mouseleave="() => { detailsVisible = false; }"
  >
    <div 
      class="flex flex-row items-center" 
      :class="{'space-x-3': size === 'lg', 'space-x-2': size === 'md' || size === 'sm', 'opacity-60': muted}"
    >
      <div 
        class="h-9 w-9 rounded-full bg-cover bg-center"
        :style="`background-image: url(${profile.fields.avatar})`"
        :class="{'h-9 w-9': size === 'lg', 'h-7 w-7': size === 'md', 'h-5 w-5': size === 'sm'}"
      />
      <div 
        :class="{'text-3xl font-bold': size === 'lg', 'text-2xl font-bold': size === 'md', 'text-lg': size === 'sm'}"
      >
        {{ profile.nickname }}
      </div>
    </div>

    <div
      v-if="hoverForDetails"
      v-show="detailsVisible"
      class="absolute z-30 bg-neutral text-neutral-content p-4 rounded-md flex flex-col justify-center min-w-64 max-w-92 break-words"
    >
      <profile-detail
        style="color: hsl(var(--nc)); --sl-input-help-text-color: hsl(var(--nc)); --sl-input-label-color: hsl(var(--nc));"
        :agentPubKey="agentPubKey"
      />
      <RouterLink
        :to="`/agents/${agentPubKeyString}`"
        class="btn btn-ghost btn-sm mt-4 f"
      >
        More
      </RouterLink>
    </div>
  </div>
</template>

<script lang="ts">
import { Profile, ProfilesStore } from '@holochain-open-dev/profiles';
import { AppAgentClient, encodeHashToBase64 } from '@holochain/client';
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
    },
    hoverForDetails: {
      type: Boolean,
      default: true
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
  data() : { profile?: Profile; detailsVisible: boolean } {
    return {
      profile: undefined,
      detailsVisible: false,
    }
  },
  computed: {
    agentPubKeyString() {
      return encodeHashToBase64(this.agentPubKey);
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
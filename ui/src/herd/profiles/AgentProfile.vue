<template>
  <div
    v-if="profile"
    class="relative"
    @mouseenter="() => { showDetails = true; }"
    @mouseleave="() => { showDetails = false; }"
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
      v-show="showDetails"
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

<script lang="ts" setup>
import { Profile, ProfilesStore } from '@holochain-open-dev/profiles';
import { encodeHashToBase64 } from '@holochain/client';
import { ref, ComputedRef, inject, computed, watch } from 'vue'
import { useRequest } from 'vue-request';
import { toast } from 'vue3-toastify';

const props = withDefaults(defineProps<{
  agentPubKey: Uint8Array,
  size?: string,
  muted?: boolean,
  hoverForDetails?: boolean
}>(),{
  size: 'lg',
  muted: false,
  hoverForDetails: true
});

const profilesStore = (inject('profilesStore') as ComputedRef<ProfilesStore>).value;
const showDetails = ref(false);

const agentPubKeyString = computed(() => {
  return encodeHashToBase64(props.agentPubKey);
});

const getAgentProfile = async (): Promise<undefined | Profile> => {
  const res = await profilesStore.client.getAgentProfile(props.agentPubKey);
  return res;
};

const { data: profile, run: runGetAgentProfile } = useRequest(getAgentProfile, {
  onError: (e: any) => {
    toast.error(`Error getting agent profile ${e.data.data}`);
  }
});

watch(props, runGetAgentProfile);
</script>
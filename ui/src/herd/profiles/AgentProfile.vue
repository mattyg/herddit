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
      class="absolute z-30 bg-neutral text-neutral-content p-4 rounded-md flex flex-col justify-center break-words max-w-md"
    >
      <profile-detail
        class="min-w-[15rem]"
        style="color: hsl(var(--nc)); --sl-input-help-text-color: hsl(var(--nc)); --sl-input-label-color: hsl(var(--nc));"
        :agentPubKey="agentPubKey"
      />
      <RouterLink
        v-if="isMyAgent"
        to="/my-agent"
        class="btn btn-ghost btn-sm mt-4 f"
      >
        More
      </RouterLink>
      <RouterLink
        v-else
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
import { AppAgentWebsocket, encodeHashToBase64 } from '@holochain/client';
import { isEqual } from 'lodash';
import { ref, ComputedRef, inject, computed, onUnmounted } from 'vue'

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
const client = (inject('client') as ComputedRef<AppAgentWebsocket>).value;
const profilesStore = (inject('profilesStore') as ComputedRef<ProfilesStore>).value;

const showDetails = ref(false);
const profile = ref<Profile>();

const agentPubKeyString = computed(() => {
  return encodeHashToBase64(props.agentPubKey);
});

const isMyAgent = computed(() => {
  return isEqual(props.agentPubKey, client.myPubKey);
});

const unsubscribe = profilesStore.profiles.get(props.agentPubKey).subscribe((res) => {
  if(res.status === 'complete') {
    profile.value = res.value;
  }
});
onUnmounted(unsubscribe);
</script>
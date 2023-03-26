<template>
  <div
    class="w-full min-h-screen flex justify-center bg-base-100"
    style="color: hsl(var(--bc)); --sl-input-help-text-color: hsl(var(--bc)); --sl-input-label-color: hsl(var(--bc));"
  >
    <div
      v-if="isMyAgent"
      class="w-full h-full flex justify-center items-center max-w-screen-md"
    >
      <my-profile
        class="p-8 bg-base-200 text-base-content "
        style="color: hsl(var(--bc)) !important; --sl-input-help-text-color: hsl(var(--bc)); --sl-input-label-color: hsl(var(--bc));"
      />
    </div>
    <div
      v-else
      class="w-full h-full flex justify-center items-center max-w-screen-md"
    >
      <profile-detail
        :agentPubKey="agentPubKey"
        class="p-8 bg-neutral text-neutral-content "
        style="color: hsl(var(--bc)) !important; --sl-input-help-text-color: hsl(var(--bc)); --sl-input-label-color: hsl(var(--bc));"
      />
    </div>
  </div>
</template>

<script lang="ts" setup>
import { ComputedRef, inject, computed } from 'vue'
import { useRoute } from 'vue-router';
import { isEqual} from 'lodash';
import { AppAgentClient, decodeHashFromBase64 } from '@holochain/client';

const route = useRoute();
const client = (inject('client') as ComputedRef<AppAgentClient>).value;

const agentPubKey = computed(() => {
  if(!route.params.agentPubKeyString) return;
  return decodeHashFromBase64(route.params.agentPubKeyString as string);
});

const isMyAgent = computed(() => {
  if(!agentPubKey.value) return;
  return isEqual(client.myPubKey, agentPubKey.value);
});
</script>

<template>
  <div
    class="w-full min-h-screen flex justify-center bg-base-100"
    style="color: hsl(var(--bc)); --sl-input-help-text-color: hsl(var(--bc)); --sl-input-label-color: hsl(var(--bc));"
  >
    <div
      class="w-full h-full flex justify-center items-center max-w-md"
    >
      <profile-detail
        :agentPubKey="agentPubKey"
        class="p-8 bg-base-200 text-base-content min-w-[15rem]"
        style="color: hsl(var(--bc)) !important; --sl-input-help-text-color: hsl(var(--bc)); --sl-input-label-color: hsl(var(--bc));"
      />
    </div>
  </div>
</template>

<script lang="ts" setup>
import { computed } from 'vue'
import { useRoute } from 'vue-router';
import { decodeHashFromBase64 } from '@holochain/client';

const route = useRoute();

const agentPubKey = computed(() => {
  return decodeHashFromBase64(route.params.agentPubKeyString as string);
});
</script>

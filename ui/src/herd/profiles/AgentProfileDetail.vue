<template>
  <div class="w-full min-h-screen flex justify-center">
    <div
      v-if="isMyAgent"
      class="w-full h-full flex justify-center items-center max-w-screen-md"
    >
      <my-profile />
    </div>
    <div
      v-else
      class="w-full h-full flex justify-center items-center max-w-screen-md"
    >
      <profile-detail :agentPubKey="agentPubKey" />
    </div>
  </div>
</template>

<script lang="ts">
import { ComputedRef, defineComponent, inject } from 'vue'
import { isEqual} from 'lodash';
import { ProfilesStore } from '@holochain-open-dev/profiles';
import { AppAgentClient, decodeHashFromBase64 } from '@holochain/client';

export default defineComponent({
  setup() {
    const client = (inject('client') as ComputedRef<AppAgentClient>).value;
    const profilesStore = (inject('profilesStore') as ComputedRef<ProfilesStore>).value;
    return {
      client,
      profilesStore
    };
  },
  computed: {
    isMyAgent() {
      if(!this.agentPubKey) return;

      return isEqual(this.client.myPubKey, this.agentPubKey);
    },
    agentPubKey() {
      if(!this.$route.params.agentPubKeyString) return;

      return decodeHashFromBase64(this.$route.params.agentPubKeyString as string);
    }
  },
  mounted() {
    console.log('agentPubKeyString', this.$route.params.agentPubKeyString)
  },
})
</script>

<style scoped>

</style>

<template>
  <div>
    <profile-detail :agent-pub-key="$route.params.agentPubKey" />

    <div v-if="isAgent">
      <update-profile />
    </div>
  </div>
</template>

<script lang="ts">
import { ComputedRef, defineComponent, inject } from 'vue'
import { isEqual} from 'lodash';
import { ProfilesStore } from '@holochain-open-dev/profiles';
import { AppAgentClient } from '@holochain/client';

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
    isAgent() {
      return isEqual(this.client.myPubKey, this.$route.params.agentPubKey);
    }
  },
  mounted() {
    console.log('agent pub key', this.$route.params.agentPubKey);

  },
})
</script>

<style scoped>

</style>

<template>
  <div>
    WORKING
    <profile-detail :agentPubKey="$route.params.agentPubKey"></profile-detail>

      <div v-if="isAgent">
        <update-profile></update-profile>
      </div>
  </div>
</template>

<script lang="ts">
import { ComputedRef, defineComponent, inject, PropType } from 'vue'
import { isEqual} from 'lodash';
import { ProfilesStore, ProfileDetail, UpdateProfile } from '@holochain-open-dev/profiles';
import { AppAgentClient } from '@holochain/client';

export default defineComponent({
  mounted() {
    console.log('agent pub key', this.$route.params.agentPubKey);

  },
  computed: {
    isAgent() {
      return isEqual(this.client.myPubKey, this.$route.params.agentPubKey);
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
})
</script>

<style scoped>

</style>

<template>
  <div class="navbar h-13 bg-neutral-200 text-neutral-content-200">
    <div class="flex-1">
      <RouterLink to="/" class="mx-4 btn btn-ghost normal-case text-3xl">
        herddit
      </RouterLink>
    </div>
    <div>
      <div v-if="profile === undefined" class="mx-4 text-2xl font-bold ">
        find your herd
      </div>
      <ul v-else class="menu menu-horizontal px-1 flex items-center">
        <RouterLink :to="`/herds/create`" class="btn btn-ghost btn-sm">Gather a Herd</RouterLink>

        <li tabIndex={0}>
          <div class="flex flex-row justify-center">
            <AgentProfile :agentPubKey="client.myPubKey" size="lg" />
            <svg class="fill-current" xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24"><path d="M7.41,8.58L12,13.17L16.59,8.58L18,10L12,16L6,10L7.41,8.58Z"/></svg>
          </div>
          <ul class="p-2 bg-base-100 z-40 w-full bg-base-200 text-base-content">

            <!-- TODO waiting on fix to profiles components-->
            <!-- <li><RouterLink :to="`/agents/${client.myPubKey}`">Edit Profile</RouterLink></li> -->
            
            <li><label htmlFor="join-herd-modal">Join Secret Herd</label></li>
          </ul>
        </li>
      </ul>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, computed, ComputedRef, inject, PropType } from 'vue';
import { AppWebsocket, ActionHash, AppAgentClient, AppAgentWebsocket, decodeHashFromBase64 } from '@holochain/client';
import { ProfilesStore, ProfilesClient, Profile } from "@holochain-open-dev/profiles";
import AgentProfile from '../herd/profiles/AgentProfile.vue';
import { RouterLink } from 'vue-router';

export default defineComponent({
  components: {
    AgentProfile
  },
  props: {
    profile: {
      type: Object as PropType<Profile>,
      required: false,
    }
  },
  setup() {
    const client = (inject('client') as ComputedRef<AppAgentClient>).value;
    return {
      client,
    };
  },
});
</script>

<style scoped>

</style>

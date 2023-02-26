<template>
  <div class="navbar h-13">
    <div class="flex-1">
      <RouterLink
        to="/"
        class="mx-4 btn btn-ghost normal-case text-3xl"
      >
        herddit
      </RouterLink>
    </div>
    <div>
      <div
        v-if="!profile"
        class="mx-4 text-2xl font-bold "
      >
        find your herd
      </div>
      <ul
        v-else
        class="menu menu-horizontal px-1 flex items-center"
      >
        <BaseThemeSelect class="mx-4" />

        <RouterLink
          :to="`/herds/create`"
          class="btn btn-tertiary btn-sm"
        >
          Gather a Herd
        </RouterLink>

        <li tabIndex="{0}">
          <div class="flex flex-row justify-center">
            <AgentProfile
              :agentPubKey="client.myPubKey"
              :hoverForDetails="false"
              size="lg"
            />
            <svg
              class="fill-current"
              xmlns="http://www.w3.org/2000/svg"
              width="20"
              height="20"
              viewBox="0 0 24 24"
            ><path d="M7.41,8.58L12,13.17L16.59,8.58L18,10L12,16L6,10L7.41,8.58Z" /></svg>
          </div>
          <ul class="p-2 z-40 w-full bg-base-200 text-base-content-200">
            <li>
              <RouterLink :to="`/agents/${myPubKeyBase64}`">
                Edit Profile
              </RouterLink>
            </li>
            
            <li><label htmlFor="join-herd-modal">Join Secret Herd</label></li>
          </ul>
        </li>
      </ul>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, ComputedRef, inject, PropType } from 'vue';
import { AppAgentClient, encodeHashToBase64 } from '@holochain/client';
import { Profile, ProfilesStore } from "@holochain-open-dev/profiles";
import AgentProfile from '../herd/profiles/AgentProfile.vue';
import BaseThemeSelect from './BaseThemeSelect.vue';

export default defineComponent({
  components: {
    AgentProfile,
    BaseThemeSelect
  },
  props: {
    profile: {
      type: Object as PropType<Profile> | undefined,
      default: undefined,
      required: false,
    }
  },
  setup() {
    const client = (inject('client') as ComputedRef<AppAgentClient>).value;
    const profilesStore = (inject('profilesStore') as ComputedRef<ProfilesStore>).value;
    return {
      client,
      profilesStore,
    };
  },
  computed: {
    myPubKeyBase64() {
      if(!this.client?.myPubKey) return;
      
      return encodeHashToBase64(this.client.myPubKey);
    },
  }
});
</script>

<style scoped>

</style>

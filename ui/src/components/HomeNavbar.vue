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
          <ul class="z-40 bg-base-200 text-base-content shadow-md">
            <li>
              <BaseThemeSelect>
                <svg
                  xmlns="http://www.w3.org/2000/svg"
                  width="32"
                  height="32"
                  viewBox="0 0 256 256"
                ><path
                  fill="currentColor"
                  d="M228.2 153L209 100.3a16 16 0 0 0-20.5-9.5l-59.9 21.8l11-62.8a16 16 0 0 0-2.6-12a16.2 16.2 0 0 0-10.3-6.6l-55.2-9.7a16.1 16.1 0 0 0-18.5 13L28.8 171.7a45 45 0 0 0 7.7 33.9a43.4 43.4 0 0 0 28.7 17.9a51.6 51.6 0 0 0 6.7.5H212a16 16 0 0 0 16-16v-43.7a15.7 15.7 0 0 0 .2-11.3ZM72 192a12 12 0 1 1 12-12a12 12 0 0 1-12 12Zm140 16h-88.3l88.3-32.1Zm-98.5-13.3a41 41 0 0 0 1.8-7.1l10.1-56.8l68.6-25l19.1 52.6Z"
                /></svg>
                Theme
              </BaseThemeSelect>
            </li>
            <li>
              <RouterLink :to="`/agents/${myPubKeyBase64}`">
                <svg
                  xmlns="http://www.w3.org/2000/svg"
                  width="24"
                  height="24"
                  viewBox="0 0 24 24"
                ><path
                  fill="currentColor"
                  d="m12.9 6.858l4.242 4.243L7.242 21H3v-4.243l9.9-9.9zm1.414-1.414l2.121-2.122a1 1 0 0 1 1.414 0l2.829 2.829a1 1 0 0 1 0 1.414l-2.122 2.121l-4.242-4.242z"
                /></svg>
                Edit Profile
              </RouterLink>
            </li>
            
            <li>
              <label htmlFor="join-herd-modal">Join Secret Herd</label>
            </li>
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

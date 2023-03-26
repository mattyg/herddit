<template>
  <div class="navbar h-13 z-20">
    <div class="flex-1">
      <RouterLink
        to="/"
      >
        <span class="px-4 btn btn-ghost normal-case text-3xl">
          herddit
        </span>
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
        class="menu menu-horizontal px-1 flex items-center space-x-4"
      >
        <RouterLink
          :to="`/herds/create`"
          class="btn btn-tertiary btn-sm"
        >
          Gather a Herd
        </RouterLink>

        <BaseThemeSelect class="px-4" />

        <div class="dropdown dropdown-hover">
          <label
            tabindex="0"
            class="normal-case"
          >
            <div class="flex flex-row items-center justify-center space-x-2">
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
          </label>
          <ul
            tabindex="0"
            class="dropdown-content menu z-40 bg-base-200 text-base-content shadow-md"
          >
            <li>
              <RouterLink :to="`/my-agent`">
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
          </ul>
        </div>
      </ul>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { ComputedRef, inject, computed, ref } from 'vue';
import { AppAgentClient, encodeHashToBase64 } from '@holochain/client';
import { ProfilesStore, Profile } from "@holochain-open-dev/profiles";
import AgentProfile from '../herd/profiles/AgentProfile.vue';
import BaseThemeSelect from './BaseThemeSelect.vue';

const client = (inject('client') as ComputedRef<AppAgentClient>).value;
const profilesStore = (inject('profilesStore') as ComputedRef<ProfilesStore>).value;
const profile = ref<Profile>();

profilesStore.myProfile.subscribe(res => {
  if(res.status === 'complete') profile.value = res.value;
})

const myPubKeyBase64 = computed(() => {
  if(!client.myPubKey) return;
  
  return encodeHashToBase64(client.myPubKey);
})
</script>

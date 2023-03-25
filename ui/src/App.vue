<template>
  <div
    v-if="loading"
    class="h-screen flex flex-col flex-1 justify-center items-center bg-neutral text-neutral-content"
  >
    <BaseSpinner>Heading to water...</BaseSpinner>
  </div>

  <div
    v-else
    class="w-full"
  >
    <profiles-context :store="profilesStore">
      <HomeNavbar
        class="bg-neutral text-neutral-content"
      />

      <profile-prompt
        :class="{'min-h-screen w-full flex justify-center items-center': !profile, 
                 'min-h-screen w-full': profile}"
      >
        <RouterView />
      </profile-prompt>
      
      <footer class="footer p-10 bg-accent text-accent-content">
        <div>
          <div class="text-4xl font-bold">
            herddit
          </div> 
          <div class="text-lg">
            find your herd
          </div> 
        </div>
      </footer>
    </profiles-context>
  </div>
</template>
<script lang="ts" setup>
import { provide, ref, toRaw } from 'vue';
import { AppAgentWebsocket } from '@holochain/client';
import HomeNavbar from './components/HomeNavbar.vue';
import { ProfilesStore, ProfilesClient, Profile } from "@holochain-open-dev/profiles";
import { toast } from 'vue3-toastify';
import BaseSpinner from './components/BaseSpinner.vue';
import { useThemeStore } from './stores/theme';

// Apply active theme
const themeStore = useThemeStore();
themeStore.apply(); 

const client = ref<AppAgentWebsocket>();
const profilesStore = ref();
const loading = ref(true);
const profile = ref<undefined | Profile>();

provide('client', client);
provide('profilesStore', profilesStore);

const setupConductor = async () => {
  try {
    // Setup conductor websocket
    client.value = await AppAgentWebsocket.connect('', 'herddit', 60000);

    // Setup profiles store
    const profilesClient = new ProfilesClient(toRaw(client.value), 'directory', 'profiles');
    profilesStore.value = new ProfilesStore(profilesClient, {
      avatarMode: "avatar-required",
        // Custom app level profile fields
      additionalFields: [
        {
          name: "location",
          label: "Location",
          required: false, 
        },
        {
          name: "bio",
          label: "Bio",
          required: false,
        },
        {
          name: "website",
          label: "Website",
          required: false,
        }
      ], 
    });

    profilesStore.value.myProfile.subscribe((data: any) => {
      if (data.status === 'complete') profile.value = data.value;
    });

    loading.value = false;
  } catch(e) {
    toast.error(`Error setting up conductor websocket ${e}`);
  }
};

setupConductor();
</script>

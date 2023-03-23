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
        :profile="profile"
        class="bg-neutral text-neutral-content"
      />

      <profile-prompt
        :class="{'min-h-screen w-full flex justify-center items-center': !profile, 
                 'min-h-screen w-full': profile}"
        @profile-created="createProfile"
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
<script lang="ts">
import { defineComponent, computed } from 'vue';
import { AppAgentClient, AppAgentWebsocket } from '@holochain/client';
import HomeNavbar from './components/HomeNavbar.vue';
import { ProfilesStore, ProfilesClient, Profile } from "@holochain-open-dev/profiles";
import { toast } from 'vue3-toastify';
import BaseSpinner from './components/BaseSpinner.vue';
import { useThemeStore } from './stores/theme';

export default defineComponent({
  components: {
    HomeNavbar,
    BaseSpinner,
  },
  provide() {
    return {
      client: computed(() => this.client),
      profilesStore: computed(() => this.profilesStore)
    };
  },
  setup() {
    const themeStore = useThemeStore();

    return { themeStore };
  },
  data(): {
    client?: AppAgentClient;
    profilesStore?: ProfilesStore;
    loading: boolean;
    herd_password: string;
    joinHerdModalVisible: boolean;
    profile?: Profile
  } {
    return {
      client: undefined,
      profilesStore: undefined,
      loading: true,
      herd_password: "",
      joinHerdModalVisible: false,
      profile: undefined,
    };
  },
  async mounted() {   
    // Apply active theme
    this.themeStore.apply(); 

    try {
      // Setup conductor websocket
      const client = await AppAgentWebsocket.connect('', 'herddit', 60000);

      // Setup profiles store
      const profilesClient = new ProfilesClient(client, 'directory', 'profiles');
      this.profilesStore = new ProfilesStore(profilesClient, {
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
      this.client = client;

      await this.setProfile();

      this.profilesStore.myProfile.subscribe((data) => {
        console.log('profile store', data);
        if (data.status === 'complete') {
          this.profile = data.value;
        }
      });
      this.loading = false;
    } catch (e: any) {
      toast.error("Error setting up conductor websocket", e)
    }

  },
  methods: {
    async createProfile() {
      try {
        await this.setProfile();
      } catch(e: any) {
        toast.error('Error creating profile', e);
      }
    },
    async setProfile() {
      if(!this.client) return;

      const profile = await this.profilesStore?.client.getAgentProfile(this.client.myPubKey);
      if(profile) {
        this.profile = profile;
      }
    }
  },
});
</script>

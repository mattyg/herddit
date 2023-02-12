<template>
  <div
    v-if="loading"
    class="h-screen flex justify-center items-center"
  >
    <span class="h-16 w-16 block rounded-full border-t-4 border-white-300 animate-spin z-40" />
  </div>

  <div
    v-else
    class="w-full"
  >
    <profiles-context :store="profilesStore">
      <HomeNavbar :profile="profile" />

      <div
        v-if="!profile"
        class="min-h-screen  w-full flex justify-center items-center"
      >
        <create-profile @profile-created="createProfile" />
      </div>
      <div
        v-else
        class="min-h-screen w-full"
      >
        <RouterView />
      </div>

      <footer class="footer p-10 bg-neutral text-neutral-content">
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
  
  <input
    id="join-herd-modal"
    v-model="joinHerdModalVisible"
    type="checkbox"
    className="modal-toggle"
  >
  <label
    htmlFor="join-herd-modal"
    className="modal cursor-pointer"
  >
    <label
      className="modal-box relative"
      htmlFor=""
    >
      <div class="prose">
        <h3>Enter Secret Herd-Word:</h3>
        <mwc-textarea
          v-model="herd_password"
          class="w-full h-32"
          outlined
        />
        <div class="modal-action">
          <button
            class="btn btn-primary bn-sm"
            @click="joinPrivateHerd"
          >Join Secret Herd</button>
        </div>
      </div>
    </label>
  </label>
</template>
<script lang="ts">
import { defineComponent, computed } from 'vue';
import { AppAgentClient, AppAgentWebsocket } from '@holochain/client';
import HomeNavbar from './components/HomeNavbar.vue';
import { ProfilesStore, ProfilesClient, Profile } from "@holochain-open-dev/profiles";
import { toast } from 'vue3-toastify';

export default defineComponent({
  components: {
    HomeNavbar
  },
  provide() {
    return {
      client: computed(() => this.client),
      profilesStore: computed(() => this.profilesStore)
    };
  },
  data(): {
    client?: AppAgentClient;
    profilesStore?: ProfilesStore;
    loading: boolean;
    herd_name: string;
    theme: string;
    herd_password: string;
    joinHerdModalVisible: boolean;
    profile?: Profile
  } {
    return {
      client: undefined,
      profilesStore: undefined,
      loading: true,
      herd_name: 'SupaHerd',
      theme: 'dark',
      herd_password: "",
      joinHerdModalVisible: false,
      profile: undefined,
    };
  },
  async mounted() {    
    try {
      // Setup conductor websocket
      this.client = await AppAgentWebsocket.connect('', 'herddit', 12000);

      // Setup profiles store
      const profilesClient = new ProfilesClient(this.client, 'directory', 'profiles');
      this.profilesStore = new ProfilesStore(profilesClient, {
        avatarMode: "avatar-required",
        additionalFields: ["Bio", "Location", "Website"],
      });
      
      await this.setProfile();

      this.profilesStore.myProfile.subscribe((data) => {
        console.log('profile store', data);
        if (data.status === 'complete') {
          this.profile = data.value;
        }
      });
      } catch (e: any) {
        toast.error("Error setting up conductor websocket")
      }

    this.loading = false;
  },
  methods: {
    joinPrivateHerd() {
      this.$router.push(`/herds/private/${this.herd_password}`);
      this.herd_password = "";
      this.joinHerdModalVisible = false;
    },
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

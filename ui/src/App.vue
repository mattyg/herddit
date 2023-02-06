<template>
    <div v-if="loading" class="h-screen flex justify-center items-center">
      <span class="h-16 w-16 block rounded-full border-t-4 border-white-300 animate-spin z-[10]"></span>
    </div>
    <div v-else class="h-screen">
      <HomeNavbar />

      <RouterView></RouterView>
    </div>

  <input type="checkbox" id="join-herd-modal" v-model="joinHerdModalVisible" className="modal-toggle" />
  <label htmlFor="join-herd-modal" className="modal cursor-pointer">
    <label className="modal-box relative" htmlFor="">
      <h3 class="text-xl">Enter Secret Herd-Word:</h3>
      <mwc-textarea class="w-full h-32 my-4" v-model="herd_password" outlined></mwc-textarea>
      <div class="modal-action">
        <button class="btn btn-primary bn-sm" @click="joinPrivateHerd">Join Secret Herd</button>
      </div>
    </label>
  </label>
</template>
<script lang="ts">
import { defineComponent, computed } from 'vue';
import { AppWebsocket, ActionHash, AppAgentClient, AppAgentWebsocket, decodeHashFromBase64 } from '@holochain/client';
import '@material/mwc-circular-progress';
import HomeNavbar from './components/HomeNavbar.vue';

export default defineComponent({
  components: {
    HomeNavbar,
  },
  data(): {
    client: AppAgentClient | undefined;
    loading: boolean;
    herd_name: string;
    theme: string;
    herd_password: string;
    joinHerdModalVisible: boolean;
  } {
    return {
      client: undefined,
      loading: true,
      herd_name: 'SupaHerd',
      theme: 'dark',
      herd_password: "",
      joinHerdModalVisible: false,
    };
  },
  async mounted() {    
    // eslint-disable-next-line @typescript-eslint/ban-ts-comment
    // @ts-ignore
    const TIMEOUT = 12000
    // default timeout is set to 12000
    this.client = await AppAgentWebsocket.connect('', 'herddit', 12000);

    this.loading = false;
  },
  methods: {
    joinPrivateHerd() {
      this.$router.push(`/herds/private/${this.herd_password}`);
      this.herd_password = "";
      this.joinHerdModalVisible = false;
    }
  },
  provide() {
    return {
      client: computed(() => this.client),
    };
  },
});
</script>

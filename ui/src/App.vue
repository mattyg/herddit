<template>
    <div v-if="loading" class="h-screen flex justify-center items-center">
      <span class="h-16 w-16 block rounded-full border-t-4 border-white-300 animate-spin z-[10]"></span>
    </div>
    <div v-else class="h-screen">
      <div class="navbar bg-base-300">
        <div class="flex-1">
          <RouterLink to="/" class="mx-4 btn btn-ghost normal-case text-xl">
            herddit
          </RouterLink>
        </div>
        <div>
          <ul class="menu menu-horizontal px-1 flex items-center">
            <RouterLink :to="`/herds/create`" class="btn btn-primary btn-sm">Create Herd</RouterLink>

            <li tabIndex={0}>
              <RouterLink to="/account">
                My Account
                <svg class="fill-current" xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24"><path d="M7.41,8.58L12,13.17L16.59,8.58L18,10L12,16L6,10L7.41,8.58Z"/></svg>
              </RouterLink>
              <ul class="p-2 bg-base-100">
                <li><a>Submenu 1</a></li>
                <li><a>Submenu 2</a></li>
              </ul>
            </li>
          </ul>
        </div>
      </div>

      <RouterView></RouterView>
    </div>
</template>
<script lang="ts">
import { defineComponent, computed } from 'vue';
import { AppWebsocket, ActionHash, AppAgentClient, AppAgentWebsocket } from '@holochain/client';
import '@material/mwc-circular-progress';
import { themeChange} from 'theme-change'

export default defineComponent({
  components: {
  },
  data(): {
    client: AppAgentClient | undefined;
    loading: boolean;
    herd_name: string;
    theme: string;
  } {
    return {
      client: undefined,
      loading: true,
      herd_name: 'SupaHerd',
      theme: 'dark',
    };
  },
  async mounted() {
    themeChange(true);
    
    // eslint-disable-next-line @typescript-eslint/ban-ts-comment
    // @ts-ignore
    const TIMEOUT = 12000
    // default timeout is set to 12000
    const appWs = await AppWebsocket.connect('', 12000);
    this.client = await AppAgentWebsocket.connect(appWs, 'herddit');

    this.loading = false;
  },
  provide() {
    return {
      client: computed(() => this.client),
    };
  },
});
</script>

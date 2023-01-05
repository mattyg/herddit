<template>
    <div v-if="loading" class="h-screen flex justify-center items-center">
      <span class="h-16 w-16 block rounded-full border-t-4 border-white-300 animate-spin z-[10]"></span>
    </div>
    <div v-else class="h-screen">
      <div className="w-full h-4 bg-base-400 text-xs">
        SuperHerd, MetaHerd, holofriends, SuperHerd, MetaHerd, holofriends, SuperHerd, MetaHerd, holofriends, SuperHerd, MetaHerd, holofriends
      </div>
      <div className="navbar bg-base-300">
        <div className="flex-1">
          <RouterLink to="/" className="mx-4 btn btn-ghost normal-case text-xl">herddit</RouterLink>
        </div>
        <div>
          <ul className="menu menu-horizontal px-1 flex items-center">
            <li><RouterLink to="/posts/create" class="btn btn-primary btn-sm">Create Herd</RouterLink></li>
            <li><RouterLink to="neighbors">Neighbors</RouterLink></li>


            <li tabIndex={0}>
              <RouterLink to="/account">
                My Account
                <svg className="fill-current" xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24"><path d="M7.41,8.58L12,13.17L16.59,8.58L18,10L12,16L6,10L7.41,8.58Z"/></svg>
              </RouterLink>
              <ul className="p-2 bg-base-100">
                <li><a>Submenu 1</a></li>
                <li><a>Submenu 2</a></li>
              </ul>
            </li>
            <li>
              <div className="form-control">
                <label className="label cursor-pointer">
                  <button data-toggle-theme="dark,light" className="toggle"></button>
                </label>
              </div>
            </li>
          </ul>
        </div>
      </div>
      
      <div class="w-full flex flex-row justify-between items-center border-b-2 mb-14 space-x-4">
        <div class="text-3xl mx-8 my-4 border-b-2">h/{{herd_name}}</div>
        <div class="flex-row justify-between items-center space-x-2">
          <RouterLink to="/posts/create" class="btn btn-primary btn-sm">Post</RouterLink>
          <RouterLink to="/posts/create" class="btn btn-accent btn-sm">Follow</RouterLink>
          <RouterLink to="/posts/create" class="btn btn-secondary btn-sm">Fork</RouterLink>
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

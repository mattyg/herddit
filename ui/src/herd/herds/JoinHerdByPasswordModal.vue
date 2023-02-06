<template>
  <div class="modal" :class="{'modal-open': visible}">
    <div class="modal-box">
      <h3 class="text-xl"><mwc-icon>warning</mwc-icon>Below is your secret Herd Password:</h3>

      <mwc-textarea class="w-full h-32 my-4" outlined :value="password" @input="password = $event.target.value" @focus="copy"></mwc-textarea>
      <p class="text-lg mb-4">Only share it with people allowed in the herd</p>
      <div class="modal-action">
        <button class="btn btn-primary bn-sm" htmlFor="herd-password-modal" @click="join()">Go to Herd</button>
      </div>

    </div>
  </div>
</template>

<script lang="ts">
import { AppAgentClient } from '@holochain/client';
import { join } from 'path';
import { PropType, defineComponent, ComputedRef, inject } from 'vue';
import { toast } from 'vue3-toastify';


export default defineComponent({
  props: {
    visible: {
      default: false
    },
  },
  data(): {
    password: string;
  } {
    return { 
      password: ""
    }
  },
  methods: {
    copy() {
      console.log('copy to clipboard')
    },
    async join() {
      try {
        const listing = await this.client.callZome({
          role_name: 'herd',
          zome_name: 'directory',
          fn_name: 'bubble_babble_to_listing',
          payload: this.password,
        });
      } catch (e: any) {
          console.log('error', e);
          toast.error('Error converting data to mnemonic', e);
      }
    },
    async decodeListing() {
      
    },
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
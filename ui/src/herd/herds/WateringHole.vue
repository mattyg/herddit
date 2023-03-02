<template>
  <div
    v-if="loading"
    class="h-screen flex flex-col flex-1 justify-center items-center space-y-4"
  >
    <BaseSpinner>Heading to water...</BaseSpinner>
  </div>

  <div
    v-else
    class="w-full"
  >
    <div class="h-16 sticky top-0 w-full flex flex-row justify-between items-center shadow-md space-x-4 px-8 bg-neutral text-neutral-content z-30">
      <div class="text-3xl my-4">
        The Watering Hole
      </div>
      <div>
        <div class="flex justify-center items-center space-x-4 my-8">
          <div class="text-neutrl-content font-bold">
            Show Private Herds
          </div>
          <input
            v-model="showPrivate"
            type="checkbox"
            class="toggle toggle-md"
            :checked="showPrivate"
          >
        </div>
      </div>
    </div>
        
    <AllListings
      :show-empty-message="true"
      class="my-12"
    />
  </div>
</template>

<script lang="ts">
import { AppAgentClient } from '@holochain/client';
import { ComputedRef, defineComponent, inject } from 'vue'
import AllListings from '../directory/AllListings.vue';

export default defineComponent({
    components: {
        AllListings
    },
    setup() {
        const client = (inject('client') as ComputedRef<AppAgentClient>).value;
        return {
          client,
        };
    },
    data() {
        return {
            loading: true,
        };
    },
    async mounted() {
        await this.client.appInfo();
        this.loading = false;
    },
})
</script>

<style scoped>

</style>
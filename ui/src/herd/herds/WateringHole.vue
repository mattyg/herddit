<template>
  <div
    v-if="!loading"
    class="w-full h-full bg-base-100 text-base-content"
  >
    <div class="h-16 sticky top-0 w-full flex flex-row justify-between items-center shadow-md space-x-4 px-8 z-30 bg-neutral text-neutral-content">
      <div class="text-3xl my-4">
        The Watering Hole
      </div>
      <div>
        <div class="flex justify-center items-center space-x-4 my-8">        
          <div class="btn btn-ghost btn-sm">
            <label htmlFor="join-herd-modal">Find Secret Herd</label>
          </div>
          <div
            class="flex flex-row space-x-2 justify-start items-center cursor-pointer btn btn-ghost btn-sm"
            @click="showPrivate = !showPrivate"
          >
            <div>Show Secret Herds</div>
            <input
              v-model="showPrivate"
              type="checkbox"
              class="toggle toggle-md"
            >
          </div>
        </div>
      </div>
    </div>

    <h1 class="text-5xl font-bold text-center my-12">
      Find Your Herd
    </h1>
    <AllListings
      :show-empty-message="true"
      :show-private="showPrivate"
      class="my-12 z-0"
    />
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
      <div class="prose form-control">
        <h3>Enter Secret Herd-Word:</h3>
        <textarea
          v-model="herd_password"
          class="textarea textarea-bordered textarea-sm w-full h-32"
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
import { AppAgentClient } from '@holochain/client';
import { ComputedRef, defineComponent, inject } from 'vue'
import AllListings from '../directory/AllListings.vue';
import BaseSpinner from '../../components/BaseSpinner.vue';

export default defineComponent({
    components: {
        AllListings,
        BaseSpinner,
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
            showPrivate: true,
            herd_password: "",
            joinHerdModalVisible: false,
        };
    },
    async mounted() {
        await this.client.appInfo();
        this.loading = false;
    },
    methods: {
      joinPrivateHerd() {
        this.$router.push(`/herds/private/${this.herd_password}`);
        this.herd_password = "";
        this.joinHerdModalVisible = false;
      },
    }
})
</script>

<style scoped>

</style>
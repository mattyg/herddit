<template>
  <div
    v-if="loading"
    class="h-screen flex flex-col flex-1 justify-center items-center bg-neutral text-neutral-content"
  >
    <BaseSpinner>Heading to water...</BaseSpinner>
  </div>
  <div
    v-else
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

    <AllListings
      :show-empty-message="true"
      :show-private="showPrivate"
    />
  </div>
  <input
    id="join-herd-modal"
    v-model="showJoinHerdModal"
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

<script lang="ts" setup>
import { AppAgentClient } from '@holochain/client';
import { ComputedRef, ref, inject, onMounted } from 'vue'
import AllListings from '../directory/AllListings.vue';
import BaseSpinner from '../../components/BaseSpinner.vue';
import { useRouter } from 'vue-router';

const client = (inject('client') as ComputedRef<AppAgentClient>).value;
const router = useRouter();

const loading = ref(true);
const showPrivate = ref(true);
const herd_password = ref("");
const showJoinHerdModal = ref(false);

onMounted(async () => {
    await client.appInfo();
    loading.value = false;
});

const joinPrivateHerd = () => {
  router.push(`/herds/private/${herd_password.value}`);
  herd_password.value = "";
  showJoinHerdModal.value = false;
};
</script>

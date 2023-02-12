<template>
  <div
    class="modal"
    :class="{'modal-open': visible}"
  >
    <div class="modal-box">
      <h3 class="text-xl">
        Below is your secret Herd-Word:
      </h3>
      <mwc-textarea
        class="w-full h-32 my-4"
        outlined
        readonly
        :value="text"
      />
      <p class="text-lg mb-4">
        Only share it with your stampede!
      </p>
      <div class="modal-action">
        <button
          class="btn btn-primary bn-sm"
          htmlFor="herd-password-modal"
          @click="() => { $router.push(`/herds/private/${text}`) }"
        >
          Go to Herd
        </button>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';

export default defineComponent({
  props: {
    visible: {
      type: Boolean,
      default: false
    },
    text: {
      type: String,
      default: "",
      required: true,
    }
  }
});
</script>

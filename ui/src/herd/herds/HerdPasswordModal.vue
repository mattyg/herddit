<template>
  <div
    class="modal"
    :class="{'modal-open': visible}"
  >
    <div class="modal-box">
      <h3 class="text-xl">
        This is your secret Herd-Word:
      </h3>
      <textarea
        class="textarea textarea-bordered w-full h-32 my-4"
        :value="text"
        disabled
      />
      <div class="text-lg mb-4 flex justify-center items-center space-x-4">
        <svg
          xmlns="http://www.w3.org/2000/svg"
          width="24"
          height="24"
          viewBox="0 0 24 24"
        ><path
          fill="currentColor"
          d="M1 21L12 2l11 19Zm11-3q.425 0 .713-.288Q13 17.425 13 17t-.287-.712Q12.425 16 12 16t-.712.288Q11 16.575 11 17t.288.712Q11.575 18 12 18Zm-1-3h2v-5h-2Z"
        /></svg>
        <div>Only share it with your stampede!</div>
        <svg
          xmlns="http://www.w3.org/2000/svg"
          width="24"
          height="24"
          viewBox="0 0 24 24"
        ><path
          fill="currentColor"
          d="M1 21L12 2l11 19Zm11-3q.425 0 .713-.288Q13 17.425 13 17t-.287-.712Q12.425 16 12 16t-.712.288Q11 16.575 11 17t.288.712Q11.575 18 12 18Zm-1-3h2v-5h-2Z"
        /></svg>
      </div>
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

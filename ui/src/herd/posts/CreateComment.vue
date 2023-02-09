<template>
  <div class="w-full" v-bind="$attrs">
    <mwc-textarea class="w-full mb-2" outlined contenteditable label="Respond to Call" @input="content = $event.target.value" :value="content"></mwc-textarea>
  
    <button
      v-if="content?.length > 0 "
      class="btn btn-primary btn-sm"
      :disabled="!isCommentValid || submitting"
      @click="createComment"
    >Respond</button>
  </div>
</template>
<script lang="ts">
import { defineComponent, inject, ComputedRef, PropType } from 'vue';
import { AppAgentClient, Record, AgentPubKey, EntryHash, ActionHash } from '@holochain/client';
import { Comment } from './types';
import { toast } from 'vue3-toastify';

export default defineComponent({
  data(): {
    content: string;
    submitting: boolean;
  } {
    return { 
      content: "",
      submitting: false,
    }
  },
  props: {
    dnaHash: {
      type: Object as PropType<Uint8Array>,
      required: true
    },
    postHash: {
      type: Object as PropType<Uint8Array>,
      required: true
    },
  },
  computed: {
    isCommentValid() {
      return true && this.content.length > 0;
    },
  },
  methods: {
    async createComment() {
      this.submitting = true;
      
      const comment: Comment = { 
        content: this.content!,
        post_ah: this.postHash!,
      };

      try {
        const record: Record = await this.client.callZome({
          cell_id: [this.dnaHash, this.client.myPubKey],
          zome_name: 'posts',
          fn_name: 'create_comment',
          payload: comment,
        });
        this.$emit('created', record.signed_action.hashed.hash);
        this.content = "";
      } catch (e: any) {
        toast.error(`Error creating the comment: ${e.data.data}`);
      }

      this.submitting = false;
    },
  },
  emits: ['created'],
  setup() {
    const client = (inject('client') as ComputedRef<AppAgentClient>).value;
    return {
      client,
    };
  },
})
</script>

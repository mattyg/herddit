<template>
  <mwc-snackbar ref="create-error"></mwc-snackbar>

  <div style="display: flex; flex-direction: column">
    <span style="font-size: 18px">Create Comment</span>
  
    <div style="margin-bottom: 16px">
      <mwc-textarea outlined label="Content" @input="content = $event.target.value" required :value="content"></mwc-textarea>
    </div>

  
    <mwc-button 
      raised
      label="Create Comment"
      :disabled="!isCommentValid || submitting"
      @click="createComment"
    ></mwc-button>
  </div>
</template>
<script lang="ts">
import { defineComponent, inject, ComputedRef, PropType } from 'vue';
import { AppAgentClient, Record, AgentPubKey, EntryHash, ActionHash } from '@holochain/client';
import { Comment } from './types';
import '@material/mwc-button';
import '@material/mwc-icon-button';
import '@material/mwc-snackbar';
import { Snackbar } from '@material/mwc-snackbar';
import '@material/mwc-textarea';
import { error } from 'console';
import { create } from 'domain';

export default defineComponent({
  data(): {
    content: string | undefined;
    submitting: boolean;
  } {
    return { 
      content: undefined,
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
      return true && this.content !== undefined
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
        const errorSnackbar = this.$refs['create-error'] as Snackbar;
        errorSnackbar.labelText = `Error creating the comment: ${e.data.data}`;
        errorSnackbar.show();
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
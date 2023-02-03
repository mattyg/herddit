<template>
  <mwc-snackbar ref="update-error"></mwc-snackbar>

  <div style="display: flex; flex-direction: column">
    <span style="font-size: 18px">Edit Comment</span>
      <div style="margin-bottom: 16px">
      <mwc-textarea outlined label="Content" :value="content" @input="content = $event.target.value" required></mwc-textarea>
      </div>



    <div style="display: flex; flex-direction: row">
      <mwc-button
        outlined
        label="Cancel"
        @click="$emit('edit-canceled')"
        style="flex: 1; margin-right: 16px;"
      ></mwc-button>
      <mwc-button 
        raised
        label="Save"
        :disabled="!isCommentValid"
        @click="updateComment"
        style="flex: 1;"
      ></mwc-button>
    </div>
  </div>
</template>
<script lang="ts">
import { defineComponent, inject, ComputedRef } from 'vue';
import { AppAgentClient, Record, AgentPubKey, EntryHash, ActionHash } from '@holochain/client';
import { Comment } from './types';
import '@material/mwc-button';
import '@material/mwc-snackbar';
import { decode } from '@msgpack/msgpack';
import { Snackbar } from '@material/mwc-snackbar';
import '@material/mwc-textarea';

export default defineComponent({
  data(): {
    content: string;
  } {
    const currentComment = decode((this.currentRecord.entry as any).Present.entry) as Comment;
    return { 
      content: currentComment.content,
    }
  },
  props: {
    originalCommentHash: {
      type: null,
      required: true,
    },
    currentRecord: {
      type: Object,
      required: true
    }
  },
  computed: {
    currentComment() {
      return decode((this.currentRecord.entry as any).Present.entry) as Comment;
    },
    isCommentValid() {
      return true && this.content !== undefined;
    },
  },
  methods: {
    async updateComment() {

      const comment: Comment = { 
        content: this.content!,
        post_ah: this.currentComment.post_ah,
      };

      try {
        const updateRecord: Record = await this.client.callZome({
          cap_secret: null,
          role_name: 'herd',
          zome_name: 'posts',
          fn_name: 'update_comment',
          payload: {
            original_comment_hash: this.originalCommentHash,
            previous_comment_hash: this.currentRecord.signed_action.hashed.hash,
            updated_comment: comment
          }
        });
        this.$emit('comment-updated', updateRecord.signed_action.hashed.hash);
      } catch (e: any) {
        const errorSnackbar = this.$refs['update-error'] as Snackbar;
        errorSnackbar.labelText = `Error updating the comment: ${e.data.data}`;
        errorSnackbar.show();
      }
    },
  },
  emits: ['comment-updated', 'edit-canceled'],
  setup() {
    const client = (inject('client') as ComputedRef<AppAgentClient>).value;
    return {
      client,
    };
  },
})
</script>

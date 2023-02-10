<template>
  <mwc-snackbar ref="update-error"></mwc-snackbar>

  <div class="flex flex-col w-full">
    <div class="mb-4">Edit Response</div>
    <mwc-textarea class="w-full mb-4" outlined label="Content" :value="content" @input="content = $event.target.value" required></mwc-textarea>

    <div class="flex flex-row justify-end items-center space-x-4">
      <button
        @click="$emit('cancelled')"
        class="btn btn-ghost btn-sm"
      >Cancel</button>
      <button 
        :disabled="!isCommentValid"
        @click="updateComment"
        class="btn btn-primary btn-sm"
      >Save</button>
    </div>
  </div>
</template>
<script lang="ts">
import { defineComponent, inject, ComputedRef, PropType } from 'vue';
import { AppAgentClient, Record, AgentPubKey, EntryHash, ActionHash } from '@holochain/client';
import { Comment } from './types';
import { decode } from '@msgpack/msgpack';
import { error } from 'console';
import { update } from 'lodash';
import { toast } from 'vue3-toastify';

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
    dnaHash: {
      type: Object as PropType<Uint8Array>,
      required: true,
    },
    originalCommentHash: {
      type: Object as PropType<Uint8Array>,
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
      return this.content;
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
          cell_id: [this.dnaHash, this.client.myPubKey],
          zome_name: 'posts',
          fn_name: 'update_comment',
          payload: {
            original_comment_hash: this.originalCommentHash,
            previous_comment_hash: this.currentRecord.signed_action.hashed.hash,
            updated_comment: comment
          }
        });
        this.$emit('updated', updateRecord.signed_action.hashed.hash);
      } catch (e: any) {
        toast.error(`Error updating the comment: ${e.data.data}`);
      }
    },
  },
  emits: ['updated', 'cancelled'],
  setup() {
    const client = (inject('client') as ComputedRef<AppAgentClient>).value;
    return {
      client,
    };
  },
})
</script>

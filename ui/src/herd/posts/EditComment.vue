<template>
  <div class="flex flex-col w-full">
    <div class="mb-4">
      Edit Response
    </div>
    <textarea
      v-model="content"
      class="w-full mb-4 textarea textarea-bordered"
      outlined
      label="Content"
    />

    <div class="flex flex-row justify-end items-center space-x-4">
      <button
        class="btn btn-ghost btn-sm"
        @click="$emit('cancelled')"
      >
        Cancel
      </button>
      <button 
        :disabled="!isCommentValid"
        class="btn btn-primary btn-sm"
        @click="updateComment"
      >
        Save
      </button>
    </div>
  </div>
</template>
<script lang="ts">
import { defineComponent, inject, ComputedRef, PropType } from 'vue';
import { AppAgentClient, Record } from '@holochain/client';
import { Comment } from './types';
import { decode } from '@msgpack/msgpack';
import { toast } from 'vue3-toastify';

export default defineComponent({
  props: {
    dnaHash: {
      type: Object as PropType<Uint8Array>,
      required: true,
    },
    originalActionHash: {
      type: Object as PropType<Uint8Array>,
      required: true,
    },
    currentRecord: {
      type: Object,
      required: true
    }
  },
  emits: ['updated', 'cancelled'],
  setup() {
    const client = (inject('client') as ComputedRef<AppAgentClient>).value;
    return {
      client,
    };
  },
  data(): {
    content: string;
  } {
    const currentComment = decode((this.currentRecord.entry as any).Present.entry) as Comment;
    return { 
      content: currentComment.content,
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
        content: this.content,
        post_ah: this.currentComment.post_ah,
      };

      try {
        const updateRecord: Record = await this.client.callZome({
          cell_id: [this.dnaHash, this.client.myPubKey],
          zome_name: 'posts',
          fn_name: 'update_comment',
          payload: {
            original_comment_hash: this.originalActionHash,
            updated_comment: comment
          }
        });
        this.$emit('updated', updateRecord.signed_action.hashed.hash);
      } catch (e: any) {
        toast.error(`Error updating the comment: ${e.data.data}`);
      }
    },
  },
})
</script>

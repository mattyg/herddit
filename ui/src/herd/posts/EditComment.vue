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
<script lang="ts" setup>
import { inject, computed, ComputedRef, ref } from 'vue';
import { AppAgentClient, Record } from '@holochain/client';
import { Comment } from './types';
import { decode } from '@msgpack/msgpack';
import { toast } from 'vue3-toastify';

const props = defineProps<{
  dnaHash: Uint8Array,
  originalActionHash: Uint8Array,
  currentRecord: Record
}>();
const emit = defineEmits(['updated', 'cancelled']);
const client = (inject('client') as ComputedRef<AppAgentClient>).value;

const comment = computed(() => {
  return decode((props.currentRecord.entry as any).Present.entry) as Comment;
});
const isCommentValid = computed(() => content.value);
const content = ref<string>(comment.value.content);

const updateComment = async () => {

  const newComment: Comment = { 
    content: content.value,
    post_ah: comment.value.post_ah,
  };

  try {
    const updateRecord: Record = await client.callZome({
      cell_id: [props.dnaHash, client.myPubKey],
      zome_name: 'posts',
      fn_name: 'update_comment',
      payload: {
        original_comment_hash: props.originalActionHash,
        updated_comment: newComment
      }
    });
    emit('updated', updateRecord.signed_action.hashed.hash);
  } catch (e: any) {
    toast.error(`Error updating the comment: ${e.data.data}`);
  }
};
</script>

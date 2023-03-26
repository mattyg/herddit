<template>
  <div
    class="w-full"
    v-bind="$attrs"
  >
    <div class="form-control">
      <label class="label text-sm text-gray-400 font-bold">
        Respond to Call
      </label>
      <textarea
        v-model="content"
        class="textarea textarea-primary w-full mb-2"
      />
    </div>

  
    <button
      v-if="content?.length > 0 "
      class="btn btn-primary btn-sm"
      :disabled="!isCommentValid || submitting"
      @click="createComment"
    >
      Respond
    </button>
  </div>
</template>
<script lang="ts" setup>
import { ref, computed, defineEmits, inject, ComputedRef, PropType } from 'vue';
import { AppAgentClient, Record } from '@holochain/client';
import { Comment } from './types';
import { toast } from 'vue3-toastify';

const props = defineProps<{
  dnaHash: Uint8Array,
  postHash: Uint8Array,
}>();
const emit = defineEmits(['created']);
const client = (inject('client') as ComputedRef<AppAgentClient>).value;

const content = ref("");
const submitting = ref(false);

const isCommentValid = computed(() => {
  return true && content.value.length > 0;
});

const createComment = async () => {
  submitting.value = true;
  
  const comment: Comment = { 
    content: content.value,
    post_ah: props.postHash,
  };

  try {
    const record: Record = await client.callZome({
      cell_id: [props.dnaHash, client.myPubKey],
      zome_name: 'posts',
      fn_name: 'create_comment',
      payload: comment,
    });

    // Upvote my comment
    await client.callZome({
      cell_id: [props.dnaHash, client.myPubKey],
      zome_name: 'posts',
      fn_name: 'upvote_comment',
      payload: record.signed_action.hashed.hash,
    });

    emit('created', record.signed_action.hashed.hash);
    content.value = "";
  } catch (e: any) {
    toast.error(`Error creating the comment: ${e.data.data}`);
  }

  submitting.value = false;
};
</script>

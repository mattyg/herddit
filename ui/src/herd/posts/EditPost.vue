<template>
  <div class="w-full flex justify-center">
    <div class="prose w-full md:max-w-screen-lg mx-4">
      <h1>Edit Call</h1>
      <div class="form-control w-full">
        <label class="label">
          <span class="label-text font-bold">Title</span>
        </label>
        <input
          v-model="title"
          type="text"
          class="input input-md input-primary mb-4"
        >
      </div>
      <div class="form-control w-full">
        <label class="label">
          <span class="label-text font-bold">Message</span>
        </label>
        <textarea
          ref="contentTextarea"
          v-model="content"
          rows="10"
          class="w-full textarea textarea-primary"
          label="Content"
        />
        <label class="label">
          <span>Use markdown for rich text</span>
        </label>
      </div>
      <div class="flex flex-row justify-end items-center space-x-4 mt-8">
        <button
          class="btn btn-ghost btn-sm"
          @click="$emit('cancelled')"
        >
          Cancel
        </button>
        <button 
          :disabled="!isPostValid"
          class="btn btn-primary btn-sm"
          @click="updatePost"
        >
          Edit Call
        </button>
      </div>
    </div>
  </div>
</template>
<script lang="ts" setup>
import { ref, inject, ComputedRef, computed } from 'vue';
import { AppAgentClient, Record } from '@holochain/client';
import { Post } from './types';
import { decode } from '@msgpack/msgpack';
import '@vaadin/date-time-picker/theme/material/vaadin-date-time-picker.js';
import { toast } from 'vue3-toastify';

const props = defineProps<{
  dnaHash: Uint8Array,
  postHash: Uint8Array,
  currentRecord: Record
}>();
const emit = defineEmits(['updated', 'cancelled']);
const client = (inject('client') as ComputedRef<AppAgentClient>).value;

const post = computed(() => decode((props.currentRecord.entry as any).Present.entry) as Post);
const title = ref(post.value.title);
const content = ref(post.value.content);

const isPostValid =  computed(() => {
  return title.value && content.value;
});

const updatePost = async () => {
  if(!isPostValid.value) return;

  const newPost: Post = { 
    title: title.value,
    content: content.value,
  };

  try {
    const updateRecord: Record = await client.callZome({
      cell_id: [props.dnaHash, client.myPubKey],
      zome_name: 'posts',
      fn_name: 'update_post',
      payload: {
        original_post_hash: props.postHash,
        previous_post_hash: props.currentRecord.signed_action.hashed.hash,
        updated_post: newPost
      }
    });
    emit('updated', updateRecord.signed_action.hashed.hash);
  } catch (e: any) {
    toast.error(`Error updating the post: ${e.data.data}`);
  }
};
</script>

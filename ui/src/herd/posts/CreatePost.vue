<template>
  <div class="w-full flex justify-center">
    <div class="prose w-full md:max-w-screen-lg mx-4">
      <h1>Call to Herd</h1>
      <div class="form-control w-full">
        <label class="label">
          <span class="label-text font-bold">Title</span>
        </label>
        <input
          v-model="title"
          type="text"
          class="input mb-4 input-md input-primary"
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
        <label class="label text-sm">Use markdown for rich text</label>
      </div>
      <div class="form-control my-12">
        <button
          class="btn btn-primary"
          :disabled="!isPostValid"
          @click="createPost"
        >
          Call to Herd
        </button>
      </div>
    </div>
  </div>
</template>
<script lang="ts" setup>
import { inject, computed, ComputedRef, ref } from 'vue';
import { AppAgentClient, Record, encodeHashToBase64 } from '@holochain/client';
import { Post } from './types';
import { toast } from 'vue3-toastify';
import { useRoute, useRouter} from 'vue-router';

const props = defineProps<{
  dnaHash: Uint8Array
}>();
const emit = defineEmits(['post-created']);
const client = (inject('client') as ComputedRef<AppAgentClient>).value;
const router = useRouter();
const route = useRoute();

const title = ref("");
const content = ref("");

const isPostValid = computed(() => {
  return title.value && content.value;
});

const createPost = async () => {
  if(!isPostValid.value) return;
  
  const post: Post = { 
    title: title.value,
    content: content.value,
  };

  try {
    const record: Record = await client.callZome({
      cell_id: [props.dnaHash, client.myPubKey],
      zome_name: 'posts',
      fn_name: 'create_post',
      payload: post,
    });
  
    // Upvote my post
    await client.callZome({
      cell_id: [props.dnaHash, client.myPubKey],
      zome_name: 'posts',
      fn_name: 'upvote_post',
      payload: record.signed_action.hashed.hash,
    });

    emit('post-created', record.signed_action.hashed.hash);
    router.push(`/herds/${route.params.listingHashString}/posts/${encodeHashToBase64(record.signed_action.hashed.hash)}`);
  } catch (e: any) {
    toast.error(`Error creating the Post: ${e.data.data}`);
  }
};
</script>

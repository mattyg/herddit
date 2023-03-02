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
<script lang="ts">
import { defineComponent, inject, ComputedRef, PropType } from 'vue';
import { AppAgentClient, Record, encodeHashToBase64 } from '@holochain/client';
import { Post } from './types';
import { toast } from 'vue3-toastify';
export default defineComponent({
  
  props: {
    dnaHash: {
      type: Object as PropType<Uint8Array>,
      required: true
    },
  },
  emits: ['post-created'],
  setup() {
    const client = (inject('client') as ComputedRef<AppAgentClient>).value;
    return {
      client,
    };
  },
  data(): {
    title: string;
    content: string;
    record: Record | undefined;
    loading: boolean;
  } {
    return { 
      title: "",
      content: "",
      record: undefined,
      loading: false,
    }
  },
  computed: {
    isPostValid() {
      return this.title && this.content;
    },
  },
  methods: {
    async createPost() {
      if(!this.isPostValid) return;
      
      const post: Post = { 
        title: this.title,
        content: this.content,
      };

      try {
        const record: Record = await this.client.callZome({
          cell_id: [this.dnaHash, this.client.myPubKey],
          zome_name: 'posts',
          fn_name: 'create_post',
          payload: post,
        });
      
        // Upvote my post
        await this.client.callZome({
          cell_id: [this.dnaHash, this.client.myPubKey],
          zome_name: 'posts',
          fn_name: 'upvote_post',
          payload: record.signed_action.hashed.hash,
        });

        this.$emit('post-created', record.signed_action.hashed.hash);
        this.$router.push(`/herds/${this.$route.params.listingHashString}/posts/${encodeHashToBase64(record.signed_action.hashed.hash)}`);
      } catch (e: any) {
        toast.error(`Error creating the Post: ${e.data.data}`);

      }
    },
  },
})
</script>

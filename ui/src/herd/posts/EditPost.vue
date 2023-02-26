<template>
  <div class="w-full flex justify-center">
    <div class="prose w-full md:max-w-screen-lg mx-4">
      <h1>Edit Call</h1>
      <div class="form-control w-full">
        <label class="label">
          <span class="label-text">Title</span>
        </label>
        <input
          v-model="title"
          type="text"
          class="input input-md input-primary mb-4"
        >
      </div>
      <div class="form-control w-full">
        <label class="label">
          <span class="label-text">Message</span>
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
<script lang="ts">
import { defineComponent, inject, ComputedRef, PropType } from 'vue';
import { AppAgentClient, Record, encodeHashToBase64 } from '@holochain/client';
import { Post } from './types';
import { decode } from '@msgpack/msgpack';
import '@vaadin/date-time-picker/theme/material/vaadin-date-time-picker.js';
import { toast } from 'vue3-toastify';

export default defineComponent({
  props: {
    dnaHash: {
      type: Object as PropType<Uint8Array>,
      required: true,
    },
    postHash: {
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
    title: string;
    content: string;
  } {
    const currentPost = decode((this.currentRecord.entry as any).Present.entry) as Post;
    return { 
      title: currentPost.title,
      content: currentPost.content
    };
  },
  computed: {
    currentPost() {
      return decode((this.currentRecord.entry as any).Present.entry) as Post;
    },
    isPostValid() {
      return this.title && this.content;
    },
    postHashString() {
      if(!this.postHash) return undefined;

      return encodeHashToBase64(this.postHash);
    }
  },
  methods: {
    async updatePost() {
      if(!this.isPostValid) return;

      const post: Post = { 
        title: this.title,
        content: this.content,
      };

      try {
        const updateRecord: Record = await this.client.callZome({
          cell_id: [this.dnaHash, this.client.myPubKey],
          zome_name: 'posts',
          fn_name: 'update_post',
          payload: {
            original_post_hash: this.postHash,
            previous_post_hash: this.currentRecord.signed_action.hashed.hash,
            updated_post: post
          }
        });
        this.$emit('updated', updateRecord.signed_action.hashed.hash);
      } catch (e: any) {
        toast.error(`Error updating the post: ${e.data.data}`);
      }
    },
  },
})
</script>

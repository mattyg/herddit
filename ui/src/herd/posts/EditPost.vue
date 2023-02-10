<template>
<div class="w-full flex justify-center">
    <div class="prose w-full md:max-w-screen-lg mx-4">
      <h1>Edit Call</h1>
      <mwc-textfield class="w-full mb-4" outlined label="Title" :value="title" @input="title = $event.target.value" required></mwc-textfield>
      <mwc-textarea ref="contentTextarea"  rows="10" class="w-full" outlined label="Content" :value="content"  @input="content = $event.target.value" required></mwc-textarea>
      <span>Use markdown for rich text</span>
      
      <div class="flex flex-row justify-end items-center space-x-4 mt-8">
        <button
          @click="$emit('cancelled')"
          class="btn btn-ghost btn-sm"
        >Cancel</button>
        <button 
        :disabled="!isPostValid"
        @click="updatePost"
          class="btn btn-primary btn-sm"
        >Edit Call</button>
      </div>
  </div>
</div>
</template>
<script lang="ts">
import { defineComponent, inject, ComputedRef, PropType } from 'vue';
import { AppAgentClient, Record, AgentPubKey, EntryHash, ActionHash, decodeHashFromBase64, encodeHashToBase64} from '@holochain/client';
import { Post } from './types';
import { decode } from '@msgpack/msgpack';
import { Snackbar } from '@material/mwc-snackbar';
import '@vaadin/date-time-picker/theme/material/vaadin-date-time-picker.js';
import { error } from 'console';
import { update } from 'lodash';
import { title } from 'process';
import { toast } from 'vue3-toastify';

export default defineComponent({
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

      const post: Post = { 
        title: this.title!,
        content: this.content!,
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
  emits: ['updated', 'cancelled'],
  setup() {
    const client = (inject('client') as ComputedRef<AppAgentClient>).value;
    return {
      client,
    };
  },
})
</script>

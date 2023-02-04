<template>
  <mwc-snackbar ref="update-error"></mwc-snackbar>

  <div class="flex h-full justify-center item-center">
    <div class="w-full md:max-w-md bg-white-200">
      <div class="text-2xl mb-8">Update Call to Herd</div>
    
      <div class="mb-4">
        <mwc-textfield class="w-full" outlined label="Title" @input="title = $event.target.value" :value="title" required></mwc-textfield>
      </div>

      <div  class="mb-4">
        <mwc-textarea class="w-full" outlined label="Content" @input="content = $event.target.value" :value="content" required></mwc-textarea>
      </div>
    

    <div class="flex flex-row justify-end items-center space-x-4">
      <button
        @click="$emit('cancelled')"
        class="btn btn-ghost btn-sm"
      >Cancel</button>
      <button 
      :disabled="!isPostValid"
      @click="updatePost"
        class="btn btn-primary btn-sm"
      >Call to Herd</button>
    </div>
  
  </div>

  </div>
</template>
<script lang="ts">
import { defineComponent, inject, ComputedRef, PropType } from 'vue';
import { AppAgentClient, Record, AgentPubKey, EntryHash, ActionHash, decodeHashFromBase64, encodeHashToBase64} from '@holochain/client';
import { Post } from './types';
import '@material/mwc-button';
import '@material/mwc-snackbar';
import { decode } from '@msgpack/msgpack';
import { Snackbar } from '@material/mwc-snackbar';
import '@vaadin/date-time-picker/theme/material/vaadin-date-time-picker.js';
import '@material/mwc-textfield';
import '@material/mwc-textarea';
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
      return true && this.title !== undefined && this.content !== undefined;
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

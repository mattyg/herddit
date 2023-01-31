<template>
  <mwc-snackbar ref="create-error"></mwc-snackbar>

  <div class="flex h-full justify-center item-center">
    <div class="w-full md:max-w-md bg-white-200">
      <div class="text-2xl mb-8">Create Post</div>
    
      <div class="mb-4">
        <mwc-textfield class="w-full" outlined label="Title" @input="title = $event.target.value" required></mwc-textfield>
      </div>

      <div  class="mb-4">
        <mwc-textarea class="w-full" outlined label="Content" @input="content = $event.target.value" required></mwc-textarea>
      </div>
    
      <button class="btn bn-primary"
        :disabled="!isPostValid"
        @click="createPost"
      >Create Post</button>
    </div>
  </div>
</template>
<script lang="ts">
import { defineComponent, inject, ComputedRef, PropType } from 'vue';
import { AppAgentClient, Record, AgentPubKey, EntryHash, ActionHash, encodeHashToBase64 } from '@holochain/client';
import { Post } from './types';
import '@material/mwc-button';
import '@material/mwc-icon-button';
import '@material/mwc-snackbar';
import { Snackbar } from '@material/mwc-snackbar';
import '@material/mwc-textfield';
import '@vaadin/date-time-picker/theme/material/vaadin-date-time-picker.js';
import '@material/mwc-textarea';
import { decode } from '@msgpack/msgpack';
export default defineComponent({
  
  props: {
    dnaHash: {
      type: Object as PropType<Uint8Array>,
      required: true
    },
  },
  data(): {
    title: string | undefined;
    content: string | undefined;
    record: Record | undefined;
    loading: boolean;
  } {
    return { 
      title: undefined,
      content: undefined,
      record: undefined,
      loading: false,
    }
  },
  computed: {
    isPostValid() {
      return true && this.title !== undefined && this.content !== undefined;
    },
  },
  methods: {
    async createPost() {
      const post: Post = { 
        title: this.title!,
        content: this.content!,
      };

      try {
        const record: Record = await this.client.callZome({
          cell_id: [this.dnaHash, this.client.myPubKey],
          cap_secret: null,
          zome_name: 'posts',
          fn_name: 'create_post',
          payload: post,
        });
      
        this.$emit('post-created', record.signed_action.hashed.hash);
        this.$router.push(`/herds/${this.$route.params.listingHashString}/posts/${encodeHashToBase64(record.signed_action.hashed.hash)}`);
      } catch (e: any) {
        const errorSnackbar = this.$refs['create-error'] as Snackbar;
        errorSnackbar.labelText = `Error creating the post: ${e.data.data}`;
        errorSnackbar.show();
      }
    },
  },
  emits: ['post-created'],
  setup() {
    const client = (inject('client') as ComputedRef<AppAgentClient>).value;
    return {
      client,
    };
  },
})
</script>

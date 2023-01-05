<template>
  <mwc-snackbar ref="create-error"></mwc-snackbar>

  <div class="flex h-full justify-center item-center">
    <div class="w-full md:max-w-md bg-white-200">
      <span class="text-xl mb-4">Create Post</span>
    
      <div style="margin-bottom: 16px" class="w-full">
        <mwc-textfield outlined label="Title" @input="title = $event.target.value" required></mwc-textfield>
      </div>

      <div style="margin-bottom: 16px" class="w-full">
        <mwc-textarea outlined label="Content" @input="content = $event.target.value" required></mwc-textarea>
      </div>
    
      <button class="btn bn-primary"
        :disabled="!isPostValid"
        @click="createPost"
      >Create Post</button>
    </div>
  </div>
</template>
<script lang="ts">
import { defineComponent, inject, ComputedRef } from 'vue';
import { AppAgentClient, Record, AgentPubKey, EntryHash, ActionHash } from '@holochain/client';
import { Post } from './types';
import '@material/mwc-button';
import '@material/mwc-icon-button';
import '@material/mwc-snackbar';
import { Snackbar } from '@material/mwc-snackbar';
import '@material/mwc-textfield';
import '@vaadin/date-time-picker/theme/material/vaadin-date-time-picker.js';
import { serializeHash } from '@holochain-open-dev/utils';
import '@material/mwc-textarea';
export default defineComponent({
  data(): {
    title: string | undefined;
    content: string | undefined;
  } {
    return { 
      title: undefined,
      content: undefined,
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
          cap_secret: null,
          role_name: 'posts',
          zome_name: 'posts',
          fn_name: 'create_post',
          payload: post,
        });
        this.$emit('post-created', record.signed_action.hashed.hash);
        this.$router.push(`/posts/${serializeHash(record.signed_action.hashed.hash)}`);
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

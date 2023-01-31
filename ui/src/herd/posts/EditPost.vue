<template>
  <mwc-snackbar ref="update-error"></mwc-snackbar>

  <div style="display: flex; flex-direction: column">
    <span style="font-size: 18px">Edit Post</span>
      <div style="margin-bottom: 16px">
      <mwc-textfield outlined label="Title" :value="title" @input="title = $event.target.value" required></mwc-textfield>
      </div>

      <div style="margin-bottom: 16px">
      <mwc-textarea outlined label="Content" :value="content" @input="content = $event.target.value" required></mwc-textarea>
      </div>


    <div style="display: flex; flex-direction: row">
      <mwc-button
        outlined
        label="Cancel"
        @click="$emit('edit-canceled')"
        style="flex: 1; margin-right: 16px;"
      ></mwc-button>
      <mwc-button 
        raised
        label="Save"
        :disabled="!isPostValid"
        @click="updatePost"
        style="flex: 1;"
      ></mwc-button>
    </div>
  </div>
</template>
<script lang="ts">
import { defineComponent, inject, ComputedRef } from 'vue';
import { AppAgentClient, Record, AgentPubKey, EntryHash, ActionHash, decodeHashFromBase64} from '@holochain/client';
import { Post } from './types';
import '@material/mwc-button';
import '@material/mwc-snackbar';
import { decode } from '@msgpack/msgpack';
import { Snackbar } from '@material/mwc-snackbar';
import '@vaadin/date-time-picker/theme/material/vaadin-date-time-picker.js';
import '@material/mwc-textfield';
import '@material/mwc-textarea';

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
    postHashString: {
      type: null,
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
    originalPostHash() {
      if(!this.postHashString) return undefined;

      return decodeHashFromBase64(this.postHashString);
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
          cap_secret: null,
          role_name: 'herd',
          zome_name: 'posts',
          fn_name: 'update_post',
          payload: {
            original_post_hash: this.originalPostHash,
            previous_post_hash: this.currentRecord.signed_action.hashed.hash,
            updated_post: post
          }
        });
        this.$emit('post-updated', updateRecord.signed_action.hashed.hash);
      } catch (e: any) {
        const errorSnackbar = this.$refs['update-error'] as Snackbar;
        errorSnackbar.labelText = `Error updating the post: ${e.data.data}`;
        errorSnackbar.show();
      }
    },
  },
  emits: ['post-updated', 'edit-canceled'],
  setup() {
    const client = (inject('client') as ComputedRef<AppAgentClient>).value;
    return {
      client,
    };
  },
})
</script>

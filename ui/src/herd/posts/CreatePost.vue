<template>
  <div class="w-full flex justify-center">
    <div class="prose w-full md:max-w-screen-lg mx-4">
      <h1>Call to Herd</h1>
      <mwc-textfield class="w-full mb-4" outlined label="Title" @input="title = $event.target.value" required></mwc-textfield>
      <mwc-textarea ref="contentTextarea"  class="w-full mb-8" outlined label="Content" @input="content = $event.target.value" required></mwc-textarea>
    <div class="mt-8">
      <button class="btn bn-primary"
        :disabled="!isPostValid"
        @click="createPost"
      >Call to Herd</button>
    </div>
    </div>
  </div>
</template>
<script lang="ts">
import { defineComponent, inject, ComputedRef, PropType } from 'vue';
import { AppAgentClient, Record, AgentPubKey, EntryHash, ActionHash, encodeHashToBase64 } from '@holochain/client';
import { Post } from './types';
import { toast } from 'vue3-toastify';
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
        toast.error(`Error creating the Post: ${e.data.data}`);

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

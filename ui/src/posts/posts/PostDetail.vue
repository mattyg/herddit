<template>
  <div class="w-full flex justify-center">
    <div class="w-full md:max-w-screen-lg">
      <a class="fixed top-48 left-8" @click="$router.back()">Back</a>
      <div v-if="!loading">
        <div v-if="record && postContent" class="flex flex-col justify-center items-center space-y-4 my-4">
          <div class="flex flex-col justify-start items-center space-y-4">
            <div class="w-full text-4xl">{{ post?.title }}</div>
            <div class="text-lg color-neutral">Submitted {{dateRelative}} by {{authorHashString}}</div>
          </div>
          <div class="flex flex-row justify-between items-center space-x-4">
            <div class="flex flex-row justify-center items-center space-x-2">
              <button v-if="myPost" class="btn btn-primary btn-xs" @click="editPost()">Edit</button>
              <button v-if="myPost" class="btn btn-error btn-xs" @click="deletePost()">Delete</button>
            </div>

          </div>

          <div class="w-full md:max-w-screen-md bg-base-200 p-8 shadow-sm prose md:prose-lg" v-html="postContent"></div>
          <button class="btn btn-primary btn-lg" @click="editPost()">Share to your Neighbors</button>
        </div>
        
        <span v-else>The requested post was not found.</span>
      </div>

      <div v-else style="display: flex; flex: 1; align-items: center; justify-content: center">
        <mwc-circular-progress indeterminate></mwc-circular-progress>
      </div>

      <mwc-snackbar ref="delete-error" leading>
      </mwc-snackbar>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, inject, ComputedRef } from 'vue';
import { decode } from '@msgpack/msgpack';
import { AppAgentClient, Record, AgentPubKey, EntryHash, ActionHash, AppInfo } from '@holochain/client';
import { Post } from './types';
import '@material/mwc-circular-progress';
import '@material/mwc-icon-button';
import '@material/mwc-snackbar';
import { Snackbar } from '@material/mwc-snackbar';
import PostListItem from './PostListItem.vue';
import { deserializeHash, serializeHash,  } from '@holochain-open-dev/utils';
import {marked} from 'marked';
import dayjs from 'dayjs';

export default defineComponent({
  components: {
    PostListItem
  },
  props: {
    postHashString: {
      type: String,
      required: true
    }
  },
  data(): { record: Record | undefined; loading: boolean; editing: boolean; appInfo?: AppInfo} {
    return {
      record: undefined,
      loading: true,
      editing: false,
      appInfo: undefined
    }
  },
  computed: {
    post() {
      if (!this.record) return undefined;
      return decode((this.record.entry as any).Present.entry) as Post;
    },
    postHash() {
      return deserializeHash(this.postHashString);
    },
    postContent() {
      if(!this.post?.content) return undefined;

      return marked(this.post?.content);
    },
    myPost() {
      if(!this.record || !this.appInfo) return false;

      console.log('author', this.record.signed_action.hashed.content.author);
      console.log('cell_id[1]',  (this.appInfo?.cell_info['posts'][0] as any).Provisioned?.cell_id[1]);
      return serializeHash(this.record.signed_action.hashed.content.author) === serializeHash((this.appInfo?.cell_info['posts'][0] as any).Provisioned?.cell_id[1])
    },
    authorHashString() {
      if (!this.record) return undefined;

      return serializeHash(this.record.signed_action.hashed.content.author);
    },
    dateRelative() {
      if(!this.record?.signed_action.hashed.content.timestamp) return;

      return dayjs(this.record.signed_action.hashed.content.timestamp/1000).fromNow();
    },
  },
  async mounted() {
    await this.fetchPost();
    this.appInfo = await this.client.appInfo();
  },
  methods: {
    async fetchPost() {
      this.loading = true;
      this.record = undefined;

      this.record = await this.client.callZome({
        cap_secret: null,
        role_name: 'posts',
        zome_name: 'posts',
        fn_name: 'get_post',
        payload: this.postHash,
      });

      this.loading = false;
    },
    async deletePost() {
      try {
        await this.client.callZome({
          cap_secret: null,
          role_name: 'posts',
          zome_name: 'posts',
          fn_name: 'delete_post',
          payload: this.postHash,
        });
        this.$emit('post-deleted', this.postHash);
        this.fetchPost();
      } catch (e: any) {
        const errorSnackbar = this.$refs['delete-error'] as Snackbar;
        errorSnackbar.labelText = `Error deleting the post: ${e.data.data}`;
        errorSnackbar.show();
      }
    },
    editPost() {
      this.$router.push(`/post/${this.postHashString}/edit`);
    }
  },
  setup() {
    const client = (inject('client') as ComputedRef<AppAgentClient>).value;
    return {
      client
    };
  },
})
</script>

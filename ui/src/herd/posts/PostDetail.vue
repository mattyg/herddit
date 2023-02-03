 <template>
  <div class="w-full flex justify-center">
    <div class="w-full md:max-w-screen-lg">
      <a class="fixed top-48 left-8 btn btn-ghost btn-xs" @click="$router.push(`/herds/${$route.params.listingHashString}`)">Back to Herd</a>
      <div v-if="!loading">
        <div  v-if="record && postContent"  class="flex flex-row justify-center items-start space-x-4">
          <div>
            <button class="text-2xl font-bold" @click="upvotePost">⇧</button>
            <div class="text-2xl font-bold">{{  upvotes - downvotes }}</div>
            <button class="text-2xl font-bold" @click="downvotePost">⇩</button>
          </div>
          <div class="flex flex-col justify-center items-center space-y-4 my-4">
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
        </div>
      </div>
        
        <span v-else>The requested post was not found.</span>
      </div>

      <div v-else style="display: flex; flex: 1; align-items: center; justify-content: center">
        <mwc-circular-progress indeterminate></mwc-circular-progress>
      </div>

      <mwc-snackbar ref="error" leading>
      </mwc-snackbar>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, inject, ComputedRef, PropType } from 'vue';
import { decode } from '@msgpack/msgpack';
import { AppAgentClient, Record, AgentPubKey, EntryHash, ActionHash, AppInfo, encodeHashToBase64, decodeHashFromBase64 } from '@holochain/client';
import { Post } from './types';
import '@material/mwc-circular-progress';
import '@material/mwc-icon-button';
import '@material/mwc-snackbar';
import { Snackbar } from '@material/mwc-snackbar';
import PostListItem from './PostListItem.vue';
import {marked} from 'marked';
import dayjs from 'dayjs';

export default defineComponent({
  components: {
    PostListItem
  },
  props: {
    dnaHash: {
      type: Object as PropType<Uint8Array>,
      required: true
    }
  },
  data(): { record: Record | undefined; upvotes: number; downvotes: number; myVote: number; loading: boolean; editing: boolean; appInfo?: AppInfo} {
    return {
      record: undefined,
      upvotes: 0,
      downvotes: 0,
      myVote: undefined,
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
      return decodeHashFromBase64(this.$route.params.postHashString as string);
    },
    postContent() {
      if(!this.post?.content) return undefined;

      return marked(this.post?.content);
    },
    myPost() {
      if(!this.record || !this.appInfo) return false;
      return this.record.signed_action.hashed.content.author === this.client.myPubKey;
    },
    authorHashString() {
      if (!this.record) return undefined;

      return encodeHashToBase64(this.record.signed_action.hashed.content.author);
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

      try {
        const post_metadata = await this.client.callZome({
          cell_id: [this.dnaHash, this.client.myPubKey],
          cap_secret: null,
          zome_name: 'posts',
          fn_name: 'get_post_metadata',
          payload: this.postHash,
        });

        console.log('post_metadata', post_metadata)
        this.upvotes = post_metadata.upvotes;
        this.downvotes = post_metadata.downvotes;
        this.record = post_metadata.record;
      } catch (e: any) {
        console.log('error', e);
        const errorSnackbar = this.$refs['error'] as Snackbar;
        errorSnackbar.labelText = `Error deleting the post: ${e.data.data}`;
        errorSnackbar.show();
      }

      this.loading = false;
    },
    async deletePost() {
      try {
        await this.client.callZome({
          cell_id: [this.dnaHash, this.client.myPubKey],
          cap_secret: null,
          zome_name: 'posts',
          fn_name: 'delete_post',
          payload: this.postHash,
        });
        this.$emit('post-deleted', this.postHash);
        this.fetchPost();
      } catch (e: any) {
        const errorSnackbar = this.$refs['error'] as Snackbar;
        errorSnackbar.labelText = `Error deleting the post: ${e.data.data}`;
        errorSnackbar.show();
      }
    },
    editPost() {
      this.$router.push(`/herds/${this.$route.params.listingHashString}/posts/${this.$route.params.postHashString}/edit`);
    },
    async upvotePost() {
      if(this.myVote === 1) return;

      try {
        await this.client.callZome({
          cell_id: [this.dnaHash, this.client.myPubKey],
          cap_secret: null,
          zome_name: 'posts',
          fn_name: 'upvote_post',
          payload: this.postHash,
        });
        this.$emit('post-upvoted', this.postHash);
        this.fetchPost();
      } catch (e: any) {
        const errorSnackbar = this.$refs['error'] as Snackbar;
        errorSnackbar.labelText = `Error upvoting the post: ${e.data.data}`;
        errorSnackbar.show();
      }
    },
    async downvotePost() {
      if(this.myVote === -1) return;

      try {
        await this.client.callZome({
          cell_id: [this.dnaHash, this.client.myPubKey],
          cap_secret: null,
          zome_name: 'posts',
          fn_name: 'downvote_post',
          payload: this.postHash,
        });
        this.$emit('post-downvoted', this.postHash);
        this.fetchPost();
      } catch (e: any) {
        const errorSnackbar = this.$refs['error'] as Snackbar;
        errorSnackbar.labelText = `Error downvoting the post: ${e.data.data}`;
        errorSnackbar.show();
      }
    },
  },
  setup() {
    const client = (inject('client') as ComputedRef<AppAgentClient>).value;
    return {
      client
    };
  },
})
</script>

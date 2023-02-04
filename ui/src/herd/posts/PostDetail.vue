 <template>
  <div class="w-full flex justify-center">
    <div class="w-full md:max-w-screen-xl">
      <div v-if="!loading">
        <div v-if="editing && record">
          <EditPost 
            :dnaHash="dnaHash" 
            :postHash="postHash" 
            :currentRecord="record" 
            @updated="() => { editing = false; fetchPost(); }" 
            @cancelled="() => {editing = false;}" />
        </div>
        <div  v-else-if="record && postContent"  class="flex flex-row justify-center items-start space-x-4">
          <PostVotes 
            :votes="votesCount" 
            :dnaHash="dnaHash" 
            :postHash="postHash"
            @upvote="() => { if(my_vote !== 1) { my_vote = 1; upvotes += 1; fetchPost();} }"
            @downvote="() => { if(my_vote !== -1) { my_vote = -1; upvotes -= 1; fetchPost();} }"
            @error="votingError" 
            class="mr-8"
          />

          <div class="my-4">
            <div class="flex flex-col justify-start items-center space-y-4">
              <div class="w-full text-4xl">{{ post?.title }}</div>
              <div class="text-lg color-neutral text-gray-400 font-bold">Submitted {{dateRelative}} by {{authorHashString}}</div>
            </div>
            <div class="flex flex-row justify-between items-center space-x-4">
              <div class="flex flex-row justify-center items-center space-x-2">
                <mwc-icon-button v-if="myPost" class="mx-2" icon="edit" @click="editing = true"></mwc-icon-button>
                <mwc-icon-button v-if="myPost" class="mx-2" icon="delete" @click="deletePost()"></mwc-icon-button>
              </div>
            </div>

            <div class="w-full md:max-w-screen-lg bg-base-200 p-8 shadow-sm prose md:prose-lg mb-8" v-html="postContent"></div>
            
            <div class="mt-16 px-8 w-full">
              <CommentsForPost :dnaHash="dnaHash" :postHash="postHash" />
            </div>
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
import PostVotes from './PostVotes.vue';
import CommentsForPost from './CommentsForPost.vue';
import EditPost from './EditPost.vue';
import {marked} from 'marked';
import dayjs from 'dayjs';
import { error } from 'console';
import { Listing } from '../directory/types';
import { isEqual } from 'lodash';
import { toast } from 'vue3-toastify';

export default defineComponent({
  components: {
    PostListItem,
    PostVotes,
    CommentsForPost,
    EditPost
  },
  props: {
    dnaHash: {
      type: Object as PropType<Uint8Array>,
      required: true
    }
  },
  data(): { record: Record | undefined; upvotes: number; downvotes: number; my_vote?: number; loading: boolean; editing: boolean; appInfo?: AppInfo} {
    return {
      record: undefined,
      upvotes: 0,
      downvotes: 0,
      my_vote: undefined,
      loading: true,
      editing: false,
      appInfo: undefined,
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
      return isEqual(this.record.signed_action.hashed.content.author, this.client.myPubKey);
    },
    authorHashString() {
      if (!this.record) return undefined;

      return encodeHashToBase64(this.record.signed_action.hashed.content.author);
    },
    dateRelative() {
      if(!this.record?.signed_action.hashed.content.timestamp) return;

      return dayjs(this.record.signed_action.hashed.content.timestamp/1000).fromNow();
    },
    votesCount() {
      return this.upvotes - this.downvotes;
    }
  },
  async mounted() {
    await this.fetchPost();
    this.appInfo = await this.client.appInfo();
  },
  methods: {
    async fetchPost() {
      console.log('fetching post');
      this.loading = true;
      this.record = undefined;

      try {
        const post_metadata = await this.client.callZome({
          cell_id: [this.dnaHash, this.client.myPubKey],
          zome_name: 'posts',
          fn_name: 'get_post_metadata',
          payload: this.postHash,
        });

        console.log('post_metadata', post_metadata)
        this.upvotes = post_metadata.upvotes;
        this.downvotes = post_metadata.downvotes;
        this.record = post_metadata.record;
      } catch (e: any) {
         toast.error(`Error fetching the post: ${e.data.data}`);
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
        this.$router.push(`/herds/${this.$route.params.listingHashString}`);
      } catch (e: any) {
        toast.error(`Error deleting the post: ${e.data.data}`);
      }
    },
    votingError(msg: string) {
      const errorSnackbar = this.$refs['error'] as Snackbar;
        errorSnackbar.labelText = `Error voting on post ${msg}`;
        errorSnackbar.show();
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

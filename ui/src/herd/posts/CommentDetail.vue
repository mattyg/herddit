<template>
  <div v-if="!loading && !isDeleted">
    <div v-if="editing">
      <EditComment
        :dnaHash="dnaHash"
        :original-comment-hash="commentHash"
        :current-record="record!"
        @updated="editing = false; fetchComment();"
        @cancelled="editing = false"
        class="flex flex-1"
      />
    </div>
    <div v-else-if="record" >
      <div v-if="upvotes - downvotes >= 0 || showIfVoteNegative" class="flex flex-row items-center">
        <CommentVotes 
            :votes="upvotes - downvotes" 
            :dnaHash="dnaHash" 
            :commentHash="commentHash"
            @upvote="fetchComment"
            @downvote="fetchComment"
        />
        <div class="py-2 px-4 flex-1">
          <div class="flex flex-row justify-start items-end mb-2">
            <div class="flex flex-1 pre-line prose-2xl" >{{  comment?.content }} </div>
            <div v-if="isMyComment">
              <mwc-icon-button class="text-gray-400" icon="edit" @click="editing = true"></mwc-icon-button>
              <mwc-icon-button class="text-gray-400" icon="delete" @click="deleteComment()"></mwc-icon-button>
            </div>
          </div>
          <div class="flex flex-row justify-between items-center">
            <div class="flex flex-row items-center space-x-4" v-if="authorPubKey">
              <AgentProfile :agentPubKey="authorPubKey" size="sm" :muted="true">
                <div class="badge badge-sm badge-primary" v-if="isPostAuthor">Author</div>
              </AgentProfile>
            </div>
            <div class="text-xs text-gray-500">
              <span v-if="isUpdated">
                edited
              </span>
            
              {{ dateRelative }}
            </div>
          </div>
        </div>   
      </div>

      <div v-else class="w-full flex flex-row justify-between items-center bg-base-200 px-8 py-4 space-x-8 text-gray-400 font-bold">
        <div>Response trampled by the herd</div>
        <button class="btn btn-ghost btn-xs" @click="showIfVoteNegative = true">Take a look</button>
      </div>
    </div>

  </div>
</template>

<script lang="ts">
import { defineComponent, inject, ComputedRef, PropType } from 'vue';
import { decode } from '@msgpack/msgpack';
import { AppAgentClient, Record, AgentPubKey, EntryHash, ActionHash, encodeHashToBase64 } from '@holochain/client';
import { Comment } from './types';
import EditComment from './EditComment.vue';
import AgentProfile from '../profiles/AgentProfile.vue';
import dayjs from 'dayjs';
import { toast } from 'vue3-toastify';
import { isEqual } from 'lodash';
import CommentVotes from './CommentVotes.vue';

export default defineComponent({
  components: {
    EditComment,
    AgentProfile,
    CommentVotes
  },
  props: {
    dnaHash: {
      type: Object as PropType<Uint8Array>,
      required: true
    },
    commentHash: {
      type: Object as PropType<Uint8Array>,
      required: true
    },
    postAuthorHash: {
      type: Object as PropType<Uint8Array>,
      required: true
    }
  },
  data(): { record?: Record; upvotes: number; downvotes: number; loading: boolean; editing: boolean; showIfVoteNegative: boolean;} {
    return {
      record: undefined,
      upvotes: 0,
      downvotes: 0,
      loading: true,
      editing: false,
      showIfVoteNegative: false
    }
  },
  computed: {
    isUpdated() {
      if (!this.record) return undefined;

      return this.record.signed_action.hashed.content.type === 'Update';
    },
    isDeleted() {
      if (!this.record) return undefined;

      return this.record.signed_action.hashed.content.type === 'Delete';
    },
    comment() {
      if (!this.record?.entry) return undefined;
      return decode((this.record.entry as any).Present.entry) as Comment;
    },
    authorPubKey() {
      if (!this.record) return undefined;

      return this.record.signed_action.hashed.content.author;
    },
    dateRelative() {
      if(!this.record?.signed_action.hashed.content.timestamp) return;

      return dayjs(this.record.signed_action.hashed.content.timestamp/1000).fromNow();
    },
    isPostAuthor() {
      if (!this.record) return undefined;

      return isEqual(this.authorPubKey, this.postAuthorHash);
    },
    isMyComment() {
      if (!this.commentHash) return false;

      return isEqual(this.authorPubKey, this.client.myPubKey);
    }
  },
  async mounted() {
    await this.fetchComment();
  },
  methods: {
    async fetchComment() {
      try {
        console.log('fetching comment with this comment hash', this.commentHash)
        const comment_metadata = await this.client.callZome({
          cell_id: [this.dnaHash, this.client.myPubKey],
          zome_name: 'posts',
          fn_name: 'get_comment_metadata',
          payload: this.commentHash,
        });

        this.record = comment_metadata.record;
        this.upvotes = comment_metadata.upvotes;
        this.downvotes = comment_metadata.downvotes;
      } catch(e: any) {
        toast.error(`Error fetching the comment: ${e.data.data}`);
      }

      this.loading = false;
    },
    async deleteComment() {
      try {
        await this.client.callZome({
          cell_id: [this.dnaHash, this.client.myPubKey],
          zome_name: 'posts',
          fn_name: 'delete_comment',
          payload: this.commentHash,
        });
        this.$emit('deleted', this.commentHash);
        this.record = undefined;
      } catch (e: any) {
        toast.error(`Error deleting the comment: ${e.data.data}`);

      }
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

<template>
  <div v-if="!loading && !isDeleted">
    <div v-if="editing && record">
      <EditComment
        :dna-hash="dnaHash"
        :original-comment-hash="commentHash"
        :current-record="record"
        class="flex flex-1"
        @updated="editing = false; fetchComment();"
        @cancelled="editing = false"
      />
    </div>
    <div v-else-if="record">
      <div
        v-if="upvotes - downvotes >= 0 || showIfVoteNegative"
        class="flex flex-row items-start"
      >
        <CommentVotes 
          class="mr-2"
          :votes="upvotes - downvotes" 
          :dna-hash="dnaHash" 
          :comment-hash="commentHash"
          @upvote="fetchComment"
          @downvote="fetchComment"
        />
        <div class="px-4 flex-1">
          <div class="flex flex-row justify-between items-start mb-2 min-h-12">
            <!-- eslint-disable vue/no-v-html -->
            <div
              class="prose-sm md:prose-md flex-1"
              v-html="commentContent"
            />
            <!-- eslint-enable vue/no-v-html -->
            <BaseEditDeleteButtons
              v-if="isMyComment"
              @edit="editing = true"
              @delete="deleteComment()"
            />
          </div>
          <div class="flex flex-row justify-between items-center">
            <div
              v-if="authorPubKey"
              class="flex flex-row items-center space-x-4"
            >
              <AgentProfile
                :agent-pub-key="authorPubKey"
                size="sm"
                :muted="true"
              >
                <div
                  v-if="isPostAuthor"
                  class="badge badge-sm badge-primary"
                >
                  Author
                </div>
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

      <BaseContentHidden
        v-else
        @show="showIfVoteNegative = true"
      >
        Response trampled by the herd
      </BaseContentHidden>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, inject, ComputedRef, PropType } from 'vue';
import { decode } from '@msgpack/msgpack';
import { AppAgentClient, Record } from '@holochain/client';
import { Comment } from './types';
import EditComment from './EditComment.vue';
import AgentProfile from '../profiles/AgentProfile.vue';
import dayjs from 'dayjs';
import { toast } from 'vue3-toastify';
import { isEqual } from 'lodash';
import CommentVotes from './CommentVotes.vue';
import { marked } from 'marked';
import BaseEditDeleteButtons from '../../components/BaseEditDeleteButtons.vue';
import BaseContentHidden from '../../components/BaseContentHidden.vue';
import DOMPurify from 'dompurify';

export default defineComponent({
  components: {
    EditComment,
    AgentProfile,
    CommentVotes,
    BaseEditDeleteButtons,
    BaseContentHidden,
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
  emits: ['deleted'],
  setup() {
    const client = (inject('client') as ComputedRef<AppAgentClient>).value;
    return {
      client
    };
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
    commentContent() {
      if(!this.comment?.content) return undefined;

      return DOMPurify.sanitize(marked(this.comment?.content));
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
  watch: {
    commentHash() {
      this.fetchComment();
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
})
</script>

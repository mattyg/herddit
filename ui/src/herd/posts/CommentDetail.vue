<template>
  <div v-if="!isDeleted || showIfDeleted">
    <div v-if="editing">
      <EditComment
        :dna-hash="dnaHash"
        :original-action-hash="originalActionHash"
        :current-record="comment_metadata.record"
        class="flex flex-1"
        @updated="editing = false; runFetchComment();"
        @cancelled="editing = false"
      />
    </div>
    <div v-else-if="comment_metadata">
      <div
        v-if="comment_metadata.upvotes - comment_metadata.downvotes >= 0 || showIfVoteNegative"
        class="flex flex-row items-start"
      >
        <CommentVotes 
          class="mr-2"
          :votes="comment_metadata.upvotes - comment_metadata.downvotes" 
          :dna-hash="dnaHash" 
          :original-action-hash="originalActionHash"
          @upvote="runFetchComment"
          @downvote="runFetchComment"
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
        :allowPeeking="true"
        @show="showIfVoteNegative = true"
      >
        Response trampled by the herd
      </BaseContentHidden>
    </div>
  </div>
  <BaseContentHidden
    v-else-if="isDeleted"
    @show="showIfDeleted = true"
  >
    Response deleted by author
  </BaseContentHidden>
</template>

<script lang="ts" setup>
import { inject, ComputedRef, ref, computed, watch} from 'vue';
import { decode } from '@msgpack/msgpack';
import { AppAgentClient } from '@holochain/client';
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
import { useRequest } from 'vue-request';

const props = defineProps<{
  dnaHash: Uint8Array,
  originalActionHash: Uint8Array,
  postAuthorHash: Uint8Array
}>();

const emit = defineEmits(['deleted']);

const client = (inject('client') as ComputedRef<AppAgentClient>).value;

const editing = ref(false);
const showIfVoteNegative = ref(false);
const showIfDeleted = ref(false);

const isUpdated = computed(() => {
  if (!comment_metadata.value?.record) return undefined;

  return comment_metadata.value.record.signed_action.hashed.content.type === 'Update';
});
const isDeleted = computed(() => {
  if (!comment_metadata.value?.record) return false;

  return comment_metadata.value.record.signed_action.hashed.content.type === 'Delete';
});
const comment = computed(() => {
  if (!comment_metadata.value?.record.entry) return undefined;

  return decode((comment_metadata.value.record.entry as any).Present.entry) as Comment;
});
const commentContent = computed(() => {
  if(!comment.value?.content) return undefined;

  return DOMPurify.sanitize(marked(comment.value?.content));
});
const authorPubKey = computed(() => {
  if (!comment_metadata.value?.record) return undefined;

  return comment_metadata.value.record.signed_action.hashed.content.author;
});
const dateRelative = computed(() => {
  if(!comment_metadata.value?.record.signed_action.hashed.content.timestamp) return;

  return dayjs(comment_metadata.value.record.signed_action.hashed.content.timestamp/1000).fromNow();
});
const isPostAuthor = computed(() => {
  return isEqual(authorPubKey.value, props.postAuthorHash);
});
const isMyComment = computed(() => {
  return isEqual(authorPubKey.value, client.myPubKey);
});

const fetchComment = async () => {
  const res = await client.callZome({
    cell_id: [props.dnaHash, client.myPubKey],
    zome_name: 'posts',
    fn_name: 'get_comment_metadata',
    payload: props.originalActionHash,
  });

  return res;
};

const deleteComment = async () => {
  try {
    await client.callZome({
      cell_id: [props.dnaHash, client.myPubKey],
      zome_name: 'posts',
      fn_name: 'delete_comment',
      payload: props.originalActionHash
    });

    emit('deleted', props.originalActionHash);
  } catch (e: any) {
    toast.error(`Error deleting the comment: ${e.data.data}`);
  }
};

const { data: comment_metadata, run: runFetchComment } = useRequest(fetchComment, {
  onError: (e: any) => {
    toast.error(`Error fetching the comment: ${e.data.data}`);
  }
});

watch(props, () => {
  runFetchComment();
});
</script>

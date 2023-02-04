<template>
  <div class="w-full">

    <div v-if="loading" class="flex justify-center items-center">
      <mwc-circular-progress indeterminate></mwc-circular-progress>
    </div>

    <div v-else-if="!isDeleted">
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
      <div v-else-if="record" class="py-2 px-4 ">
        <div class="flex flex-row justify-start items-center">
          <div class="flex flex-1" style="white-space: pre-line">{{  comment?.content }} </div>
          <div>
            <mwc-icon-button class="mx-2" icon="edit" @click="editing = true"></mwc-icon-button>
            <mwc-icon-button class="mx-2" icon="delete" @click="deleteComment()"></mwc-icon-button>
          </div>
        </div>
        <div class="flex flex-row justify-between items-center">
          <div class="text-xs text-gray-400">{{ authorHashString }}</div>
          <div class="text-xs text-gray-400">
            <span v-if="isUpdated">
              edited
            </span>
          
            {{ dateRelative }}
          </div>
        </div>
      </div>    
    </div>

    <mwc-snackbar ref="delete-error" leading>
    </mwc-snackbar>
  </div>
</template>

<script lang="ts">
import { defineComponent, inject, ComputedRef, PropType } from 'vue';
import { decode } from '@msgpack/msgpack';
import { AppAgentClient, Record, AgentPubKey, EntryHash, ActionHash, encodeHashToBase64 } from '@holochain/client';
import { Comment } from './types';
import '@material/mwc-circular-progress';
import '@material/mwc-icon-button';
import '@material/mwc-snackbar';
import { Snackbar } from '@material/mwc-snackbar';
import EditComment from './EditComment.vue';
import { error } from 'console';
import dayjs from 'dayjs';
import { toast } from 'vue3-toastify';

export default defineComponent({
  components: {
    EditComment
  },
  props: {
    dnaHash: {
      type: Object as PropType<Uint8Array>,
      required: true
    },
    commentHash: {
      type: Object as PropType<Uint8Array>,
      required: true
    }
  },
  data(): { record: Record | undefined; loading: boolean; editing: boolean; } {
    return {
      record: undefined,
      loading: false,
      editing: false,
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
    authorHashString() {
      if (!this.record) return undefined;

      return encodeHashToBase64(this.record.signed_action.hashed.content.author);
    },
    dateRelative() {
      if(!this.record?.signed_action.hashed.content.timestamp) return;

      return dayjs(this.record.signed_action.hashed.content.timestamp/1000).fromNow();
    },
    commentHashString() {
      return encodeHashToBase64(this.commentHash);
    }
  },
  async mounted() {
    await this.fetchComment();
  },
  methods: {
    async fetchComment() {
      this.loading = true;
      try {
        console.log('fetching comment with this comment hash', this.commentHash)
        this.record = await this.client.callZome({
          cell_id: [this.dnaHash, this.client.myPubKey],
          zome_name: 'posts',
          fn_name: 'get_comment',
          payload: this.commentHash,
        });
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

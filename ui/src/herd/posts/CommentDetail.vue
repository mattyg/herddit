<template>
  <div v-if="!loading">
    <div v-if="editing">
      <EditComment
        :original-comment-hash="commentHash"
        :current-record="record!"
        @comment-updated="editing = false; fetchComment();"
        @edit-canceled="editing = false"
        style="display: flex; flex: 1;"
      ></EditComment>
    </div>
    <div v-else-if="record" style="display: flex; flex-direction: column">
      <div style="display: flex; flex-direction: row">
        <span style="font-size: 18px; flex: 1;">Comment</span>

        <mwc-icon-button style="margin-left: 8px" icon="edit" @click="editing = true"></mwc-icon-button>
        <mwc-icon-button style="margin-left: 8px" icon="delete" @click="deleteComment()"></mwc-icon-button>
      </div>

      <div style="display: flex; flex-direction: row; margin-bottom: 16px;">
	<span><strong>Content</strong></span>
 	<span style="white-space: pre-line">{{  comment?.content }} </span>
      </div>

    </div>
    
    <span v-else>The requested comment was not found.</span>
  </div>

  <div v-else style="display: flex; flex: 1; align-items: center; justify-content: center">
    <mwc-circular-progress indeterminate></mwc-circular-progress>
  </div>

  <mwc-snackbar ref="delete-error" leading>
  </mwc-snackbar>
</template>

<script lang="ts">
import { defineComponent, inject, ComputedRef, PropType } from 'vue';
import { decode } from '@msgpack/msgpack';
import { AppAgentClient, Record, AgentPubKey, EntryHash, ActionHash } from '@holochain/client';
import { Comment } from './types';
import '@material/mwc-circular-progress';
import '@material/mwc-icon-button';
import '@material/mwc-snackbar';
import { Snackbar } from '@material/mwc-snackbar';
import EditComment from './EditComment.vue';
import { error } from 'console';

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
      loading: true,
      editing: false,
    }
  },
  computed: {
    comment() {
      if (!this.record) return undefined;
      return decode((this.record.entry as any).Present.entry) as Comment;
    }
  },
  async mounted() {
    await this.fetchComment();
  },
  methods: {
    async fetchComment() {
      this.loading = true;
      this.record = undefined;

      console.log('fetching comment with this comment hash', this.commentHash)
      this.record = await this.client.callZome({
        cell_id: [this.dnaHash, this.client.myPubKey],
        zome_name: 'posts',
        fn_name: 'get_comment',
        payload: this.commentHash,
      });

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
        this.$emit('comment-deleted', this.commentHash);
        this.fetchComment();
      } catch (e: any) {
        const errorSnackbar = this.$refs['delete-error'] as Snackbar;
        errorSnackbar.labelText = `Error deleting the comment: ${e.data.data}`;
        errorSnackbar.show();
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

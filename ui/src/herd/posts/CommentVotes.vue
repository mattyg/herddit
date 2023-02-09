<template>
  <BaseVoteInput 
    :size="size"
    :votes="votes"
    :dnaHash="dnaHash"
    :commentHash="commentHash"
    :myVote="myVote"
    @upvote="upvote"
    @downvote="downvote"
  />
</template>

<script lang="ts">
import { AppAgentClient } from '@holochain/client';
import { size } from 'lodash';
import { ComputedRef, defineComponent, inject, PropType } from 'vue'
import { toast } from 'vue3-toastify';
import BaseVoteInput from '../../components/BaseVoteInput.vue';

export default defineComponent({
  components: {
    BaseVoteInput
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
    votes: {
      default: 0
    },
    size: {
      default: "lg"
    }
  },
  data(): { myVote?: number; } {
    return {
      myVote: undefined,
    }
  },
  mounted() {
    this.getMyVote();
  },
  methods: {
    async getMyVote() {
      try {
        const vote_tag = await this.client.callZome({
          cell_id: [this.dnaHash, this.client.myPubKey],
          cap_secret: null,
          zome_name: 'posts',
          fn_name: 'get_my_vote_on_comment',
          payload: this.commentHash,
        });
        
        if(vote_tag) {
          this.myVote = vote_tag.value;
        }
      } catch (e: any) {
        toast.error("Failed to vote on post: ", e.data.data);
      }
    },
    async upvote() {
      if(this.myVote === 1) return;

      try {
        await this.client.callZome({
          cell_id: [this.dnaHash, this.client.myPubKey],
          cap_secret: null,
          zome_name: 'posts',
          fn_name: 'upvote_comment',
          payload: this.commentHash,
        });
        this.$emit('upvote');
        this.getMyVote();
      } catch (e: any) {
        toast.error("Failed to vote on post: ", e.data.data);
      }
    },
    async downvote() {
      if(this.myVote === -1) return;

      try {
        await this.client.callZome({
          cell_id: [this.dnaHash, this.client.myPubKey],
          cap_secret: null,
          zome_name: 'posts',
          fn_name: 'downvote_comment',
          payload: this.commentHash,
        });
        this.$emit('downvote');
        this.getMyVote();
      } catch (e: any) {
        toast.error("Failed to vote on post: ", e.data.data);
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

<style scoped>

</style>
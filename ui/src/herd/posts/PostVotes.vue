<template>
  <BaseVoteInput 
    :size="size"
    :votes="votes"
    :dna-hash="dnaHash"
    :comment-hash="postHash"
    :my-vote="myVote"
    @upvote="upvote"
    @downvote="downvote"
  />
</template>

<script lang="ts">
import { AppAgentClient } from '@holochain/client';
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
    postHash: {
      type: Object as PropType<Uint8Array>,
      required: true
    },
    votes: {
      type: Number,
      default: 0
    },
    size: {
      type: String,
      default: "lg"
    }
  },
  emits: ['upvote', 'downvote'],
  setup() {
    const client = (inject('client') as ComputedRef<AppAgentClient>).value;
    return {
      client
    };
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
          fn_name: 'get_my_vote_on_post',
          payload: this.postHash,
        });
        
        if(vote_tag) {
          this.myVote = vote_tag.value;
        }
      } catch (e: any) {
        toast.error("Failed to get my vote", e.data.data);
      }
    },
    async upvote() {
      if(this.myVote === 1) return;

      try {
        await this.client.callZome({
          cell_id: [this.dnaHash, this.client.myPubKey],
          cap_secret: null,
          zome_name: 'posts',
          fn_name: 'upvote_post',
          payload: this.postHash,
        });
        this.$emit('upvote');
        this.getMyVote();
      } catch (e: any) {
        console.log(e);
        toast.error(`Failed to upvote post: ${e.data.data}`);
      }
    },
    async downvote() {
      if(this.myVote === -1) return;

      try {
        await this.client.callZome({
          cell_id: [this.dnaHash, this.client.myPubKey],
          cap_secret: null,
          zome_name: 'posts',
          fn_name: 'downvote_post',
          payload: this.postHash,
        });
        this.$emit('downvote');
        this.getMyVote();
      } catch (e: any) {
        console.log(e);
        toast.error(`Failed to downvote post: ${e.data.data}`);
      }
    },
  }
})
</script>

<style scoped>

</style>
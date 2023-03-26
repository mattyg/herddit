<template>
  <BaseVoteInput 
    :votes="votes"
    :dna-hash="dnaHash"
    :comment-hash="postHash"
    :my-vote="myVote"
    @upvote="upvote"
    @downvote="downvote"
  />
</template>

<script lang="ts" setup>
import { AppAgentClient } from '@holochain/client';
import { ComputedRef, inject } from 'vue'
import { useRequest } from 'vue-request';
import { toast } from 'vue3-toastify';
import BaseVoteInput from '../../components/BaseVoteInput.vue';

const props = withDefaults(defineProps<{
  dnaHash: Uint8Array,
  postHash: Uint8Array,
  votes?: number,
  size?: string
}>(), {
  votes: 0,
  size: 'lg'
});

const emit = defineEmits(['upvote', 'downvote']);
const client = (inject('client') as ComputedRef<AppAgentClient>).value;

const getMyVote = async () => {
  try {
    const vote_tag = await client.callZome({
      cell_id: [props.dnaHash, client.myPubKey],
      cap_secret: null,
      zome_name: 'posts',
      fn_name: 'get_my_vote_on_post',
      payload: props.postHash,
    });
    

    return vote_tag.value;
  } catch (e: any) {
    toast.error("Failed to get my vote", e.data.data);
  }
};

const upvote = async() => {
  if(myVote.value === 1) return;

  try {
    await client.callZome({
      cell_id: [props.dnaHash, client.myPubKey],
      cap_secret: null,
      zome_name: 'posts',
      fn_name: 'upvote_post',
      payload: props.postHash,
    });
    emit('upvote');
    runGetMyVote();
  } catch (e: any) {
    toast.error(`Failed to upvote post: ${e.data.data}`);
  }
};

const downvote = async () => {
  if(myVote.value === -1) return;

  try {
    await client.callZome({
      cell_id: [props.dnaHash, client.myPubKey],
      cap_secret: null,
      zome_name: 'posts',
      fn_name: 'downvote_post',
      payload: props.postHash,
    });
    emit('downvote');
    runGetMyVote();
  } catch (e: any) {
    toast.error(`Failed to downvote post: ${e.data.data}`);
  }
};

const { data: myVote, run: runGetMyVote } = useRequest(getMyVote);
</script>

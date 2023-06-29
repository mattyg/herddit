<template>
  <BaseVoteInput 
    :votes="votes"
    :my-vote="myVote"
    @upvote="upvote"
    @downvote="downvote"
    @rmvote="rmvote"
  />
</template>

<script lang="ts" setup>
import { AppAgentClient } from '@holochain/client';
import { ComputedRef, inject, watch } from 'vue'
import { useRequest } from 'vue-request';
import { toast } from 'vue3-toastify';
import BaseVoteInput from '../../components/BaseVoteInput.vue';

const props = defineProps<{
  dnaHash: Uint8Array,
  originalActionHash: Uint8Array,
  votes?: number
}>();

const emit = defineEmits(['upvote', 'downvote', 'rmvote']);
const client = (inject('client') as ComputedRef<AppAgentClient>).value;
 

const fetchMyVote = async () => {
  try {
    const res = await client.callZome({
      cell_id: [props.dnaHash, client.myPubKey],
      zome_name: 'posts',
      fn_name: 'get_my_vote_on_comment',
      payload: props.originalActionHash,
    });

    return res ? res.value : 0;
  } catch (e: any) {
    toast.error(`Failed to fetch post votes count: ${e.data.data}`);
  }
};

const upvote = async () => {
  if(myVote.value === 1) return;

  try {
    await client.callZome({
      cell_id: [props.dnaHash, client.myPubKey],
      zome_name: 'posts',
      fn_name: 'upvote_comment',
      payload: props.originalActionHash,
    });
    emit('upvote');
    runFetchMyVote();
  } catch (e: any) {
    toast.error(`Failed to vote on post: ${e.data.data}`);
  }
};

const downvote = async () => {
  if(myVote.value === -1) return;

  try {
    await client.callZome({
      cell_id: [props.dnaHash, client.myPubKey],
      zome_name: 'posts',
      fn_name: 'downvote_comment',
      payload: props.originalActionHash,
    });
    emit('downvote');
    runFetchMyVote();
  } catch (e: any) {
    toast.error("Failed to vote on post: ", e.data.data);
  }
};

const rmvote = async () => {
  if(myVote.value === 0) return;

  try {
    await client.callZome({
      cell_id: [props.dnaHash, client.myPubKey],
      zome_name: 'posts',
      fn_name: 'rmvote_comment',
      payload: props.originalActionHash,
    });
    emit('rmvote');
    runFetchMyVote();
  } catch (e: any) {
    toast.error("Failed to vote on post: ", e.data.data);
    console.log(e);
  }
};

const {data: myVote, run: runFetchMyVote } = useRequest(fetchMyVote, {
  onError: (e: any) => {
    toast.error(`Failed to fetch post votes count: ${e.data.data}`);
  }
});

watch(props, () => {
  runFetchMyVote();
});
</script>

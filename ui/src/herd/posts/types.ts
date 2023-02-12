import { ActionHash } from '@holochain/client';


export interface Post { 
  title: string;
  content: string;
}


export interface Comment { 
  content: string;
  post_ah: ActionHash;
}


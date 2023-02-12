import { DnaHash } from '@holochain/client';


export interface Listing { 
  title: string;
  description: string;
  network_seed: string;
  dna: DnaHash;
}


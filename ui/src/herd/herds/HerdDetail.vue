<template>
  <div
    v-if="loading"
    class="h-screen flex flex-col flex-1 justify-center items-center space-y-4 bg-base-10 text-base-content"
  >
    <BaseSpinner>
      Wandering into the herd...
    </BaseSpinner>
  </div>

  <div
    v-else
    class="w-full"
  >
    <div class="h-16 sticky top-0 w-full flex flex-row justify-between items-center shadow-md space-x-4 px-8 bg-neutral text-neutral-content z-30">
      <div class="flex flex-row justify-start items-center space-x-2">
        <div
          v-if="isPrivate"
          class="text-3xl mr-2"
          title="This herd is not published to the Watering Hole"
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            width="42"
            height="42"
            viewBox="0 0 256 256"
          ><path
            fill="currentColor"
            d="M247.3 131.3c-.4.9-10.5 23.3-33.3 43.8a8.6 8.6 0 0 1-5.4 2a8 8 0 0 1-5.9-2.6L101.4 63.1a8.1 8.1 0 0 1 4.6-13.3a132.4 132.4 0 0 1 22-1.8c34.9 0 66.6 13.3 91.7 38.3c18.8 18.9 27.3 37.7 27.6 38.5a8.2 8.2 0 0 1 0 6.5Zm-33.4 79.3a7.9 7.9 0 0 1-.5 11.3a8.2 8.2 0 0 1-5.4 2.1a8 8 0 0 1-5.9-2.6l-22-24.2A128.6 128.6 0 0 1 128 208c-34.9 0-66.6-13.3-91.7-38.3C17.5 150.8 9 132 8.7 131.3a8.2 8.2 0 0 1 0-6.5c.7-1.6 16.3-36 52.6-58.3L42.1 45.4a8 8 0 0 1 11.8-10.8Zm-68.2-51.3l-47.2-51.9a36 36 0 0 0 47.2 51.9Z"
          /></svg>
        </div>
        <RouterLink
          :to="`/herds/${$route.params.listingHashString}`"
          class="text-3xl"
        >
          h/{{ herdInfo?.title }}
        </RouterLink>
      </div>
      <div class="flex-row justify-between items-center space-x-12">
        <div
          class="btn btn-secondary btn-xs"
          @click="leaveHerd()"
        >
          Leave the Herd
        </div>
        <RouterLink
          :to="`/herds/${$route.params.listingHashString}/posts/create`"
          class="btn btn-primary btn-sm"
        >
          Call to {{ listing?.title }}
        </RouterLink>
      </div>
    </div>

    <div
      v-if="listing"
      class="w-full flex justify-center"
    >
      <div class="w-full md:max-w-screen-xl my-16 z-10">
        <RouterView :dna-hash="listing.dna" />
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { AppAgentClient, CellId, Record, encodeHashToBase64, decodeHashFromBase64, ClonedCell } from '@holochain/client';
import { decode } from '@msgpack/msgpack';
import { ComputedRef, defineComponent, inject } from 'vue'
import { Listing } from '../directory/types';
import { isEqual } from 'lodash';
import { toast } from 'vue3-toastify';
import BaseSpinner from '../../components/BaseSpinner.vue';

export default defineComponent({
    components: {
      BaseSpinner,
    },
    setup() {
        const client = (inject('client') as ComputedRef<AppAgentClient>).value;
        return {
          client,
        };
    },
    data(): { record: Record | undefined; listing?: Listing; loading: boolean; editing: boolean; herdInfo: any; cellInstalled: boolean} {
        return {
            record: undefined,
            listing: undefined,
            loading: true,
            editing: false,
            herdInfo: undefined,
            cellInstalled: false,
        };
    },
    computed: {
        dnaHashString() {
            if (!this.listing) return undefined;

            return encodeHashToBase64(this.listing.dna);
        },
        isPrivate() {
            if(this.$route.params.password) return true;
            
            if(!this.record) return false;
            
            // @ts-ignore
            return Object.keys(this.record?.signed_action.hashed.content.entry_type.App.visibility).includes('Private');
        }
    },
    async mounted() {
        if(this.$route.params.password) {
            await this.decodePasswordToListing(this.$route.params.password as string);
        } else {
            await this.fetchListing();
        }

        const cell_id = await this.installHerdCell();
        await this.fetchHerdInfo(cell_id);

        this.loading = false;    
    },
    methods: {
        async decodePasswordToListing(password: string) {
            try {
                // Deserialize Listing from Bubble Babble string
                const listing: Listing = await this.client.callZome({
                    role_name: 'directory',
                    zome_name: 'directory',
                    fn_name: 'bubble_babble_to_listing',
                    payload: password,
                });

                // Save PrivateListing to source chain
                await this.client.callZome({
                    cap_secret: null,
                    role_name: 'directory',
                    zome_name: 'directory',
                    fn_name: 'create_private_listing_idempotent',
                    payload: listing,
                });

                this.listing = listing;
                
            } catch (e: any) {
                toast.error('Invalid Secret Herd-Word', e);
                this.$router.push('/');
            }
        },
        async fetchListing() {
            try {
                this.record = await this.client.callZome({
                    cap_secret: null,
                    role_name: 'directory',
                    zome_name: 'directory',
                    fn_name: 'get_listing',
                    payload: decodeHashFromBase64(this.$route.params.listingHashString as string),
                });

                this.listing = decode((this.record?.entry as any).Present.entry) as Listing;
            } catch(e: any) {
                toast.error('Error fetching listing:', e.data.data);
            }
        },
        async installHerdCell() {  
            if(!this.listing) return;

            const appInfo = await this.client.appInfo();
            const cellInfo = appInfo.cell_info.herd.find((cell) => {  
                // @ts-ignore  
                return cell.cloned && isEqual(cell.cloned.cell_id[0], this.listing?.dna)
            });
            // @ts-ignore
            const clonedCell = cellInfo?.cloned;

            if(cellInfo && !clonedCell.enabled) {
                // If cell is disabled, enable it again
                try {
                    await this.client.enableCloneCell({
                        clone_cell_id: [this.listing?.dna, this.client.myPubKey]
                    });

                    this.cellInstalled = true;
                }
                catch(e: any) {
                   toast.error("Error enabling cloned cell", e.data.data)
                   this.$router.push('/');
                }
                
                return clonedCell.cell_id;
            } else if(!clonedCell) {
                // If cell not found, install it
                try {
                  const cloneCell: ClonedCell = await this.client.createCloneCell({
                      role_name: this.listing?.title,
                      modifiers: {
                          network_seed: this.listing?.network_seed,
                          properties: {
                              title: this.listing?.title,
                          },
                      }
                  });

                  this.cellInstalled = true;
                  return cloneCell.cell_id;
                }
                catch(e: any) {
                  toast.error("Error installing cell", e.data.data)
                  this.$router.push('/');
                }
            } else {
                this.cellInstalled = true;
                return clonedCell.cell_id;
            }
        },
        async fetchHerdInfo(cell_id: CellId) {
            try {
                this.herdInfo = await this.client.callZome({
                    cell_id,
                    cap_secret: null,
                    zome_name: 'herd',
                    fn_name: 'get_info',
                    payload: null,
                });
            } catch (e: any) {
                toast.error(`Error fetching the herd cell info: ${e?.data?.data}`);
            }
        },
        async leaveHerd() {
            if(!this.listing) return;

            try {
                await this.client.disableCloneCell({
                    clone_cell_id: [this.listing.dna, this.client.myPubKey]
                });
                toast.success(`Disabled cloned cell for herd ${this.listing.title}`);
            } catch (e: any) {
                toast.error(`Error disabling the herd cell: ${e.data.data}`);
            }

            this.$router.push('/');
        }
    },
})
</script>

<style scoped>

</style>
<template>
    <div v-if="loading" style="display: flex; flex: 1; align-items: center; justify-content: center">
        <mwc-circular-progress indeterminate></mwc-circular-progress>
    </div>

    <div v-else class="w-full">
        <div class="sticky top-0 w-full flex flex-row justify-between items-center border-b-2 space-x-4 px-8 bg-base-100 z-0">
            <div class="my-2 h-12 flex flex-col justify-center">
                <div class="flex flex-row justify-start items-center space-x-2">
                    <mwc-icon class="text-gray-400 text-3xl" v-if="isPrivate">lock</mwc-icon>
                    <RouterLink :to="`/herds/${$route.params.listingHashString}`" class="text-3xl">h/{{ herdInfo?.title }}</RouterLink>
                </div>
            </div>
            <div class="flex-row justify-between items-center space-x-12">
                <div class="btn btn-secondary btn-xs" @click="leaveHerd()">Leave the Herd</div>
                <RouterLink :to="`/herds/${$route.params.listingHashString}/posts/create`" class="btn btn-primary btn-sm">Call to {{listing?.title}} Herd</RouterLink>
            </div>
        </div>

        <div class="w-full h-4 bg-base-400 text-xs mx-4 mb-4 overflow-clip" v-if="listing">
            <AllListingsInlineText :showEmpty="false" :dnaHash="listing?.dna" />
        </div>
        
        <div class="w-full flex justify-center" v-if="listing">
            <div class="w-full md:max-w-screen-xl z-30">
                <RouterView :dnaHash="listing.dna"></RouterView>
             </div>
        </div>
    </div>
</template>

<script lang="ts">
import { AppAgentClient, CellId, Record, encodeHashToBase64, decodeHashFromBase64, ClonedCell } from '@holochain/client';
import { Snackbar } from '@material/mwc-snackbar';
import { decode } from '@msgpack/msgpack';
import { ComputedRef, defineComponent, inject, PropType } from 'vue'
import AllListingsInlineText from '../directory/AllListingsInlineText.vue';
import { Listing } from '../directory/types';
import AllPosts from '../posts/AllPosts.vue';
import { isEqual } from 'lodash';
import { RouterLink, RouterView } from 'vue-router';
import { toast } from 'vue3-toastify';

export default defineComponent({
    components: {
        AllListingsInlineText,
        AllPosts
    },
    data(): { record: Record | undefined; listing?: Listing; loading: boolean; editing: boolean; herdInfo: any; cellInstalled: boolean;} {
        return {
            record: undefined,
            listing: undefined,
            loading: true,
            editing: false,
            herdInfo: undefined,
            cellInstalled: false,
        }
    },
    computed: {
        dnaHashString() {
            if (!this.listing) return undefined;

            return encodeHashToBase64(this.listing.dna);
        },
        isPrivate() {
            if(this.$route.params.password) return true;
            
            if(!this.record) return false;
            
            return Object.keys(this.record?.signed_action.hashed.content.entry_type.App.visibility).includes('Private');
        }
    },
    async mounted() {
        this.loading = true;

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
                this.listing = await this.client.callZome({
                role_name: 'herd',
                zome_name: 'directory',
                fn_name: 'bubble_babble_to_listing',
                payload: password,
                });
            } catch (e: any) {
                console.log('error', e);
                toast.error('Error converting data to mnemonic', e);
            }
        },
        async fetchListing() {
            try {
                this.record = await this.client.callZome({
                    cap_secret: null,
                    role_name: 'herd',
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
            console.log('cell info', appInfo.cell_info.herd);
            let cellInfo = appInfo.cell_info.herd.find((cell) => {    
                return cell.cloned && isEqual(cell.cloned.cell_id[0], this.listing?.dna)
            });

            if(cellInfo) {
                console.log('already have dna with cell_id:', cellInfo.cloned.cell_id);
                const appInfo = await this.client.appInfo();
                console.log('new app info', appInfo);

                if(!cellInfo.cloned.enabled) {
                    await this.client.enableCloneCell({
                        clone_cell_id: [this.listing?.dna, this.client.myPubKey]
                    });
                }
                
                this.cellInstalled = true;
                return cellInfo.cloned.cell_id;
            } else {
                const cloneCell: ClonedCell = await this.client.createCloneCell({
                    role_name: 'herd',
                    modifiers: {
                        network_seed: this.listing?.network_seed,
                        properties: {
                            title: this.listing?.title,
                        },
                    }
                });

                console.log('installed dna with cell_id:', cloneCell.cell_id);
                const appInfo = await this.client.appInfo();
                console.log('new app info', appInfo);

                this.cellInstalled = true;
                return cloneCell.cell_id;
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
                toast.error(`Error creating the herd cell: ${e.data.data}`);
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
    setup() {
        const client = (inject('client') as ComputedRef<AppAgentClient>).value;
        return {
          client,
        };
    },
})
</script>

<style scoped>

</style>
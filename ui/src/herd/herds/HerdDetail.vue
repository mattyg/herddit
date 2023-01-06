<template>
    <div v-if="loading" style="display: flex; flex: 1; align-items: center; justify-content: center">
        <mwc-circular-progress indeterminate></mwc-circular-progress>
    </div>

    <div v-else class="w-full">
        <div class="w-full h-4 bg-base-400 text-xs mx-4 my-4 overflow-clip">
            <AllListingsInlineText />
        </div>

        <div class="w-full flex flex-row justify-between items-center border-b-2 mb-14 space-x-4">
            <div class="text-3xl mx-8 my-4">h/{{ herdInfo?.title }}</div>
            <div class="flex-row justify-between items-center space-x-2">
                <RouterLink :to="`/herds/${$route.params.listingHashString}/posts/create`" class="btn btn-primary btn-sm">Create Post</RouterLink>
                <RouterLink :to="`/herds/${$route.params.listingHashString}/follow`" class="btn btn-secondary btn-sm">Follow</RouterLink>
            </div>
        </div>

        <div class="w-full h-4 bg-base-400 text-xs mx-4 my-4 overflow-clip" v-if="listing">
            <AllListingsInlineText :dnaHash="listing?.dna" />
        </div>
        
        <div class="w-full flex justify-center" v-if="listing">
            <div class="w-full md:max-w-screen-lg">
                <RouterView :dnaHash="listing.dna"></RouterView>
             </div>
        </div>
    </div>

    <mwc-snackbar ref="create-error"></mwc-snackbar>

</template>

<script lang="ts">
import { deserializeHash, serializeHash } from '@holochain-open-dev/utils';
import { ActionHash, AppAgentClient, CellId, CellInfo, InstalledCell, Record} from '@holochain/client';
import { Snackbar } from '@material/mwc-snackbar';
import { decode } from '@msgpack/msgpack';
import { ComputedRef, defineComponent, inject, PropType } from 'vue'
import AllListingsInlineText from '../directory/AllListingsInlineText.vue';
import { Listing } from '../directory/types';
import AllPosts from '../posts/AllPosts.vue';
import { isEqual } from 'lodash';

export default defineComponent({
    components: {
        AllListingsInlineText,
        AllPosts
    },
    data(): { record: Record | undefined; loading: boolean; editing: boolean; herdInfo: any; cellInstalled: boolean;} {
        return {
            record: undefined,
            loading: true,
            editing: false,
            herdInfo: undefined,
            cellInstalled: false
        }
    },
    computed: {
        listingHash() {
            return deserializeHash(this.$route.params.listingHashString as string);
        },
        listing() {
            if (!this.record) return undefined;
            return decode((this.record.entry as any).Present.entry) as Listing;
        },
        dnaHashString() {
            if (!this.listing) return undefined;

            return serializeHash(this.listing.dna);
        }
    },
    async mounted() {
        await this.init();
    },
    methods: {
        async init() {
            console.log('init HerdDetail');
            this.loading = true;
            await this.fetchListing();
            const cell_id = await this.installHerdCell();
            await this.fetchHerdInfo(cell_id);
            this.loading = false;
        },
        async fetchListing() {
            this.record = await this.client.callZome({
                cap_secret: null,
                role_name: 'herd',
                zome_name: 'directory',
                fn_name: 'get_listing',
                payload: deserializeHash(this.$route.params.listingHashString),
            });
        },
        async installHerdCell() {            
            const appInfo = await this.client.appInfo();
            let cellInfo = appInfo.cell_info.herd.find((cell: CellInfo) => {
                return isEqual(cell.Cloned?.cell_id[0], this.listing?.dna)
            });

            if(cellInfo) {
                console.log('already have dna with cell_id:', cellInfo?.Cloned.cell_id);
                const appInfo = await this.client.appInfo();
                console.log('new app info', appInfo);

                this.cellInstalled = true;
                return cellInfo?.Cloned.cell_id;
            } else {
                const cloneCell: InstalledCell = await this.client.createCloneCell({
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
            }
            catch (e: any) {
                const errorSnackbar = this.$refs['create-error'] as Snackbar;
                errorSnackbar.labelText = `Error creating the herd: ${e.data.data}`;
                errorSnackbar.show();
            }
        },
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
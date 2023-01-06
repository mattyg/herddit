<template>
    <div v-if="loading" style="display: flex; flex: 1; align-items: center; justify-content: center">
        <mwc-circular-progress indeterminate></mwc-circular-progress>
    </div>

    <div class="w-full" v-else>
        <div class="w-full h-4 bg-base-400 text-xs mx-4 my-4 overflow-clip">
            <AllListingsInlineText />
        </div>

        <div class="w-full flex flex-row justify-between items-center border-b-2 mb-14 space-x-4">
            <div class="text-3xl mx-8 my-4">h/herddit</div>
        </div>

        <mwc-snackbar ref="create-error"></mwc-snackbar>
    </div>
</template>

<script lang="ts">
import { deserializeHash, serializeHash } from '@holochain-open-dev/utils';
import { ActionHash, AppAgentClient, CellInfo, InstalledCell, Record} from '@holochain/client';
import { Snackbar } from '@material/mwc-snackbar';
import { decode } from '@msgpack/msgpack';
import { ComputedRef, defineComponent, inject, PropType } from 'vue'
import AllListingsInlineText from '../directory/AllListingsInlineText.vue';
import { Listing } from '../directory/types';

export default defineComponent({
    components: {
        AllListingsInlineText
    },
    data() {
        return {
            loading: false
        };
    },
    async mounted() {
        this.loading = true;
        await this.client.appInfo();
        this.loading = false;
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
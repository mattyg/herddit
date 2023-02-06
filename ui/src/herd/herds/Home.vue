<template>
    <div v-if="loading" style="display: flex; flex: 1; align-items: center; justify-content: center">
        <mwc-circular-progress indeterminate></mwc-circular-progress>
    </div>

    <div class="w-full" v-else>
        <div class="w-full flex flex-row justify-between items-center border-b-2 mb-14 space-x-4">
            <div class="text-3xl mx-8 my-4">The Watering Hole</div>
        </div>

        <div class="w-full flex justify-center items-center space-x-4">
            <div class="text-gray-400 font-bold">Private Herds</div>
            <mwc-switch class="text-gray-400 font-bold" :selected="showPrivate" @click="showPrivate = !showPrivate"></mwc-switch>
        </div>
        
        <div class="w-full bg-base-400 text-xs mx-4 my-4 overflow-clip text-center">
            <AllListingsInlineText :showEmptyMessage="true" :showPrivate="showPrivate" />
        </div>
    </div>
</template>

<script lang="ts">
import { ActionHash, AppAgentClient, CellInfo, InstalledCell, Record} from '@holochain/client';
import { ComputedRef, defineComponent, inject, PropType } from 'vue'
import AllListingsInlineText from '../directory/AllListingsInlineText.vue';

export default defineComponent({
    components: {
        AllListingsInlineText
    },
    data() {
        return {
            loading: false,
            showPrivate: true,
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
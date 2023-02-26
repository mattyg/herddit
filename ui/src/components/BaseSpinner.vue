.<template>
  <div class="flex flex-col items-center justify-center space-y-8">
    <div
      class="animate-spin radial-progress text-neutral-content"
      style="--value:70"
    />
    <p class="text-3xl font-bold text-neutral-content">
      <slot />
    </p>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue'

export default defineComponent({
  setup () {
    

    return {}
  }
})
</script>

<style scoped>

</style>
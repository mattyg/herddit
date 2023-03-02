.<template>
  <div class="flex flex-col items-center justify-center space-y-8">
    <div
      class="animate-spin radial-progress"
      style="--value:70"
    />
    <p class="text-3xl font-bold">
      <slot />
    </p>
  </div>
</template>

<script setup lang="ts">
</script>
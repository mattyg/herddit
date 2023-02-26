<template>
  <select
    data-choose-theme
    class="select select-ghost select-sm max-w-xs py-0"
  >
    <option
      v-for="theme in themes"
      :key="theme"
      class="text-xs"
      :value="theme"
    >
      {{ `${theme[0].toUpperCase()}${theme.slice(1)}` }}
    </option>
  </select>
</template>

<script lang="ts">
import { defineComponent } from 'vue'
import { themeChange } from 'theme-change'


export default defineComponent({
  setup () {
    

    return {}
  },
  data() {
    return {
      themes: ["light", "dark", "cyberpunk", "corporate", "retro",  "wireframe", "black", "luxury", "cupcake", "valentine", "synthwave", "halloween", "garden", "forest", "aqua", "lofi", "pastel", "fantasy", "dracula", "cmyk", "autumn", "business", "acid", "lemonade", "night", "coffee", "winter"],
    };
  },
  mounted() {
    themeChange(false);
  }
})
</script>

<style scoped>

</style>
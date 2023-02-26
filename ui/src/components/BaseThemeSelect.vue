<template>
  <select
    data-choose-theme
    class="select select-ghost max-w-xs"
  >
    <option
      disabled
      selected
    >
      Theme
    </option>
    <option
      v-for="theme in themes"
      :key="theme"
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
      themes: ["light", "dark", "cyberpunk", "cupcake", "corporate", "valentine", "synthwave", "retro",  "halloween", "garden", "forest", "aqua", "lofi", "pastel", "fantasy", "wireframe", "black", "luxury", "dracula", "cmyk", "autumn", "business", "acid", "lemonade", "night", "coffee", "winter"],
    };
  },
  mounted() {
    themeChange(false);
  }
})
</script>

<style scoped>

</style>
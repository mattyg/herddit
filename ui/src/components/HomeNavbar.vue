<template>
  <div class="navbar bg-base-300 py-4">
    <div class="flex-1">
      <RouterLink to="/" class="mx-4 btn btn-ghost normal-case text-3xl">
        herddit
      </RouterLink>
    </div>
    <div>
      <ul class="menu menu-horizontal px-1 flex items-center">
        <label htmlFor="join-herd-modal" className="btn btn-ghost">Join Secret Herd</label>
        <RouterLink :to="`/herds/create`" class="btn btn-ghost btn-sm">Gather a Herd</RouterLink>

        <li tabIndex={0}>
          <RouterLink to="/account">
            My Account
            <svg class="fill-current" xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24"><path d="M7.41,8.58L12,13.17L16.59,8.58L18,10L12,16L6,10L7.41,8.58Z"/></svg>
          </RouterLink>
          <ul class="p-2 bg-base-100 z-40">
            <li><a>Submenu 1</a></li>
            <li><a>Submenu 2</a></li>
          </ul>
        </li>
      </ul>
    </div>
  </div>
</template>

<script setup lang="ts">

</script>

<style scoped>

</style>

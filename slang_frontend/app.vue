<script setup lang="ts">

</script>

<template>
  <div class="app-main">
    <Navbar />
    <div class="page">
      <NuxtPage />
    </div>
  </div>
</template>

<style scoped>
div.app-main {
  height: 100vh;
  width: 100vw;
  overflow: hidden;
}

div.page {
  margin: 0 5rem;
  overflow-y: scroll;
  height: 100%;
}
</style>
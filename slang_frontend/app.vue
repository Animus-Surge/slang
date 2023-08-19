<script setup lang="ts">
var disableNav = useState('disableNav')
var navsDisabled = disableNav

watch(disableNav, (newVal) => {
  navsDisabled = disableNav
})
</script>

<template>
  <div>
    <div v-if="false">
      <Navbar/>
      <Grouplist/>
    </div>
    <div class="app-main"> <!--Horizontal container-->
      <span>{{ disableNav }}</span>
      <ClientOnly fallback-tag="span" fallback="Loading...">
        <NuxtPage/>
      </ClientOnly>
    </div>
    <!--Footer-->
  </div>
</template>

<style scoped>

div {
  height: 100%;
}

div.app-main {
  display: flex;
  height: 100%;
}
</style>
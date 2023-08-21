<script setup lang="ts">

const route = useRoute()

let isLogin = route.fullPath == "/login" ? true : false

</script>

<template>
  <div>
    <div v-if="!isLogin">
      <Navbar/>
      <Grouplist/>
    </div>
    <div class="app-main"> <!--Horizontal container-->
      <div v-if="!isLogin">
        <NuxtPage/>
      </div>
      <div v-else>
        <ClientOnly>
          <NuxtPage />
        </ClientOnly>
      </div>
    </div>
    <!--Footer-->
  </div>
</template>

<style scoped>

div {
  height: 100%;
}

div.app-main div {
  display: flex;
  height: 100%;
}
</style>
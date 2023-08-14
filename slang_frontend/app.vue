<template>
  <div>
    <Navbar/>
    <Grouplist/>
    <div class="app-main"> <!--Horizontal container-->
      <NuxtPage/>
    </div>
    <!--Footer-->
  </div>
</template>

<style scoped>
div.app-main {
  display: flex;
}
</style>
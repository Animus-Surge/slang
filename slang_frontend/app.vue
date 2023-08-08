<template>
  <div>
    <Navbar></Navbar>
    <div class="app-main"> <!--Horizontal container-->
      <!--grouplist-->
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
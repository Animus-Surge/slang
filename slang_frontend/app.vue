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

div {
  height: 100%;
}

div.app-main {
  display: flex;
  height: 100%;
}
</style>
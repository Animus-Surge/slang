<script setup lang="ts">
const route = useRoute();

console.log(route.params.group);

</script>

<template>
    <div class="template-root">
        <Channellist />
    </div>
</template>

<style scoped>
@import url('~/assets/vars.css');

div.template-root {
    display: flex;
    flex-direction: row;
    
}

</style>
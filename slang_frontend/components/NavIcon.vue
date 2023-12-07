<script setup lang="ts">
const emit = defineEmits(['callback'])
const props = defineProps(['type', 'data', 'href'])

let click = async () => {
	if(!props.href) emit('callback')
	else await navigateTo(props.href)
}
</script>

<template>
	<a :class="props.type" @click="click()">
		<i v-if="props.type.startsWith('iconsec')" class="bi" :class="props.data"></i>
		<img v-else-if="props.type.startsWith('groupicon')" :src="props.data" />
        <!--Add notif bubble-->
	</a>
</template>

<style scoped>
@import url('~/assets/vars.css');

a {
    text-decoration: none;
    display: block;
    padding: 10px;
    transition-property: background-color, color;
    transition-duration: 100ms;
}

a.iconsec {
    font-size: 27pt;
    /* padding: 0; */
    background-color: #0004;
    border-radius: 10px;
    width: 45px;
    height: 45px;
    text-align: center;
}

a.active {
	color: var(--text-link-hover);
}

a.iconsec:hover {
    background-color: var(--theme1-color-3);
    color: var(--text-link-hover);
    cursor: pointer;
}

a.groupicon {
    width: 65px;
    height: 65px;
    padding: 0;
}

a.groupicon:hover {
    cursor: pointer;
}

a.groupicon img {
    transition-property: opacity;
    transition-duration: 100ms;
    opacity: 50%;
    border-radius: 10px;
}

a.groupicon:hover img {
    opacity: 100%;
}
</style>
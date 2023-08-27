<script setup lang="ts">
const props = defineProps(['icon'])
const content = ref(null)
const chevron = ref(null)
const root = ref(null)
let toggleBool = false //TODO store as data?


const toggle = () => {
	let container: HTMLBaseElement = content.value!
	let chevronIcon: HTMLBaseElement = chevron.value!
	let group: HTMLBaseElement = root.value!
	toggleBool = !toggleBool

	if(toggleBool) {
		container.style.maxHeight = '100%'
		group.style.maxHeight = 'unset';
		chevronIcon.classList.remove('bi-chevron-down')
		chevronIcon.classList.add('bi-chevron-up')
	} else {
		container.style.maxHeight = '0%'
		group.style.maxHeight = '106px'
		chevronIcon.classList.remove('bi-chevron-up')
		chevronIcon.classList.add('bi-chevron-down')
	}
}
</script>

<template>
	<div class="icongroup" ref="root">
		<NavIcon @callback="toggle" type="iconsec" :data="props.icon" />
		<div ref="content" class="content">
			<slot />
		</div>
		<a @click='toggle' class="groupchevron">
			<i ref='chevron' class="bi bi-chevron-down"></i>
		</a>
	</div>
</template>

<style scoped>
@import url('~/assets/vars.css');

div.icongroup {
	max-height: 106px;
	display: flex;
	background-color: #aaa6;
	flex-direction: column;
	gap: 5px;
	align-items: center;
	border-radius: 10px;
}

div.content {
	max-height: 0%;
	display: flex;
	flex-direction: column;
	gap: 5px;
	align-items: center;
	height: fit-content;
	overflow: hidden;
}

a.groupchevron {
	padding-top: 5px;
	padding-bottom: 5px;
	background-color: #0004;
	width: 65px;
	height: fit-content;
	border-radius: 10px;
	text-align: center;
	font-size: 14pt;
	transition-property: background, color;
}

a.groupchevron:hover {
	background-color: var(--theme1-color-3);
	color: var(--text-link-hover);
	cursor: pointer;
}
</style>
<script setup lang="ts">
//TODO: auto populate grouplist

function moveToGroup() {
	navigateTo("/groups/1")
}
</script>

<template>
	<div class="grouplist">
		<div v-on:click="moveToGroup" class="element">
			<img class="group-icon" src="https://placehold.co/45x45/aqua/aqua"/>
		</div>
		<div class="element">
			<img class="group-icon" src="https://placehold.co/45x45/maroon/maroon"/>
		</div>
		<div class="element">
			<img class="group-icon" src="https://placehold.co/45x45/gold/gold"/>
		</div>
		<div class="element">
			<img class="group-icon" src="https://placehold.co/45x45/salmon/salmon"/>
		</div>
		<div class="element">
			<img class="group-icon" src="https://placehold.co/45x45/"/>
		</div>
	</div>
</template>

<style scoped>
div.grouplist {
	display: flex;
	flex-direction: row;
	justify-content: center;
	gap: 5px;
	padding: 5px;
	background-color: #0004;
	margin-left: auto;
	margin-right: auto;
}

div.element {
	padding: 3px;
	border-radius: 3px;
	background-color: #8884;
	width: fit-content;
	transition-property: background-color;
	transition-duration: 100ms;
}

img.group-icon {
	border-radius: 50%;
	transition-property: border-radius;
	transition-duration: 100ms;
}

div.element.active {
	background-color: #c808;
}

div.element:hover {
	cursor: pointer;
	background-color: #cc08;
}

div.element:hover img.group-icon, div.element.active img.group-icon {
	border-radius: 25%;
}
</style>
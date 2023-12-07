<script setup lang="ts">
const barMenu = ref(null)
let menuVisible = false

let activeId = useState<Number>('activeId')

//TODO: use routeID for activeID

function menuShow() {
	let bm: HTMLBaseElement = barMenu.value!
	menuVisible = !menuVisible

	if(!menuVisible) {
		bm.style.visibility = 'hidden'
	} else {
		bm.style.visibility = 'visible'
	}
}
</script>

<!-- TODO hover tooltips -->

<template>
	<div class="nav">
		<!--horizontal navigation bar to switch between different features (dashboard, messenger, timeline, profile), and a menu to display the settings, security, and logout links-->
		<!--i tag is available with bootstrap icons-->

		<div class="nav-left">
			<div class="nav-item">
				<i class="bi bi-house-door-fill"></i>
			</div>
			<div class="nav-item">
				<i class="bi bi-chat-left-text-fill"></i>
			</div>
			<div class="nav-item">
				<i class="bi bi-clock-history"></i>
			</div>
			<div class="nav-item">
				<i class="bi bi-person-fill"></i>
			</div>
		</div>
	</div>
</template>

<style scoped>
@import url('~/assets/vars.css');

.nav {
	display: flex;
	flex-direction: row;
	justify-content: space-between;
	align-items: center;
	background-color: var(--color-bg);
	height: 50px;
	width: 100%;
}


</style>
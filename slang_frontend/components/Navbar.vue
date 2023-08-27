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
		<NavIcon @callback="menuShow()" type="iconsec" data="bi-list" /> <!--TODO replace with NavIcon-->
		<div class="menu" ref="barMenu" style="visibility: hidden">
			<a class="menubtn">Profile</a>
			<a class="menubtn">Settings</a>
			<a class="menubtn">Log out <i class="bi bi-box-arrow-right"></i></a>
		</div>
		<hr>
		<NavIcon :type="'iconsec ' + (activeId===-1?'active':'')" data="bi-house-fill" href="/" />
		<NavIcon :type="'iconsec ' + (activeId===-2?'active':'')" data="bi-people-fill" href="/friends" />
		<NavIcon :type="'iconsec ' + (activeId===-3?'active':'')" data="bi-bell-fill" href="/notifications" />
		<NavIcon type="iconsec" data="bi-plus" />
		<hr>
		<!--pinned DMs-->
		<Navgroup icon="bi-chat-right-text-fill">
			
		</Navgroup>
		<hr>
		<!--Groups-->
	</div>
</template>

<style scoped>
@import url('~/assets/vars.css');
a:hover {
	text-decoration: none;
}
div.nav {
	display: flex;
	flex-direction: column;
	gap: 5px;
	background-color: #0004;
	padding: 5px;
	align-items: center;
	overflow-y: scroll;
}

div.menu {
	padding: 10px;
	font-size: 14pt;
	background-color: #111;
	position: absolute;
	top: 20px;
	left: 95px;
	border-radius: 5px;
	width: 250px;
}

a.menubtn {
	display: flex;
	padding: 8px;
	flex-direction: row;
	justify-content: space-between;
	align-items: center;
	background-color: #0000;
	border-radius: 5px;
}

a.menubtn i {
	float: right;
	font-size: 16pt;
}

a.menubtn:hover {
	background-color: var(--theme1-color-3);
}

hr {
	color: #999;
	width: 65px;
}
</style>
<script setup lang="ts">

</script>

<!-- TODO hover tooltips -->

<template>
	<div>
		<div class="bigscreen">
			<div class="linkgroup">
				<a class="nav-button active" href="#">
					<i class="bi bi-house"></i>
					<span class="tooltip">Dashboard</span>
				</a>
				<a class="nav-button" href="#">
					<i class="bi bi-people"></i>
					<span class="notification-badge">3</span>
					<span class="tooltip">Friends</span>
				</a>
				<a class="nav-button" href="#">
					<i class="bi bi-search"></i>
					<span class="tooltip">Search</span>
				</a>
				<a class="nav-button" href="#">
					<i class="bi bi-bell"></i>
					<span class="tooltip">Notifications</span>
				</a>
			</div>
			<div class="divider">
			</div>
			<div class="linkgroup bottom">
				<div class="divider"></div>
				<a class="nav-button" href="#">
					<i class="bi bi-person"></i> <!--Replace with profile image-->
					<span class="tooltip">Profile</span>
				</a>
				<a class="nav-button" href="#">
					<i class="bi bi-gear"></i>
					<span class="tooltip">Settings</span>
				</a>
				<a class="nav-button" href="#">
					<i class="bi bi-box-arrow-in-right" style="transform: translateX(-4px)"></i>
					<span class="tooltip">Logout</span>
				</a>
			</div>
		</div>
		<div class="smallscreen">
			<a class="nav-button active" href="#">
				<i class="bi bi-house"></i>
			</a>
			<a class="nav-button" href="#">
				<i class="bi bi-people"></i>
				<span class="notification-badge">3</span>
			</a>
			<a class="nav-button" href="#">
				<i class="bi bi-search"></i>
			</a>
			<a class="nav-button" href="#">
				<i class="bi bi-bell"></i>
			</a>
			<a class="nav-button" href="#">
				<i class="bi bi-person"></i>
			</a>
		</div>
	</div>
</template>

<style scoped>
@import url('~/assets/vars.css');

div {
	width: 100%;
}

/* Desktop mode */

div.bigscreen {
	position: fixed;
	top: 0;
	left: 0;

	height: 100%;
	width: 4rem;
	max-width: 4rem;

	padding-top: 2rem;

	background-color: #222;
	box-shadow: 0 0 0.5rem #222;


	display: flex;
	flex-direction: column;
	align-items: center;
}

.bottom {
	margin-top: auto;
	transform: translateY(-30%);
}

a.nav-button span.tooltip {
	opacity: 0%;

	position: absolute;
	flex-grow: 0;

	/* Need the positioning to always be 8 pixels away from the navbar centered at the nav-button */
	top: 50%;
	left: 140%;
	transform: translateY(-50%);

	font-size: 12pt;
	white-space: nowrap;
	text-align: start;
	color: #fff;

	background-color: #222;
	padding: 3px;
	border-radius: 3px;
	box-shadow: 0 0 0.5rem #000;

	transition-property: opacity;
	transition-duration: 100ms;

	pointer-events: none;
}

a.nav-button:hover span.tooltip {
	opacity: 100%;
	color: #fff;
}

div.linkgroup {
	display: flex;
	flex-direction: column;
	align-items: center;
	gap: 0.75rem;
}

div.divider {
	width: 80%;
	height: 3px;
	margin-top: 1rem;
	margin-bottom: 1rem;
	background-color: #0004;
}

a.nav-button {
	border-radius: 50%;
	/**Make square */
	width: 3rem;
	height: 3rem;

	font-size: 20pt;
	font-weight: bold;

	position: relative;
	display: flex;
	justify-content: center;
	align-items: center;

	transition-property: background-color, box-shadow, color;
	transition-duration: 100ms;

	background-color: #0004;
	box-shadow: 0 0 0.5rem #0004;
}

a.nav-button:hover {
	background-color: #b86d07;
	box-shadow: 0 0 0.5rem #b86d07;
	color: #000;
}

a.nav-button.active {
	background-color: #fbc200;
	color: #000;

	box-shadow: 0 0 0.5rem #fbc200;
}

a.nav-button.active:hover {
	background-color: #b86d07;
	box-shadow: 0 0 0.5rem #b86d07;
}

div.smallscreen {
	display: none;
}

span.notification-badge {
	position: absolute;

	transform: translate(1rem, -1rem);

	background-color: #700;
	color: #fff;
	box-shadow: 0 0 0.5rem #700;

	border-radius: 50%;
	width: 1.5rem;
	height: 1.5rem;

	display: flex;
	justify-content: center;
	align-items: center;
	text-align: center;

	font-size: 12pt;
	font-weight: bold;
}

@media screen and (max-width: 768px) {

	/* Mobile mode */

	div.bigscreen {
		display: none;
	}

	div.smallscreen {
		position: fixed;
		bottom: 0;

		display: flex;
		width: 100%;
		height: 4rem;
		background-color: #222;

		justify-content: space-around;
		align-items: center;
	}

	a.nav-button {
		width: 2.75rem;
		height: 2.75rem;
	}
}

</style>
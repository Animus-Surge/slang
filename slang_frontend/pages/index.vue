<script setup lang="ts">
const activeId = useState('activeId')
onMounted(() => activeId.value = -1)
</script>

<template>
	<div class="content">
		<h2>Dashboard</h2>
		<div class="grid">
			<div class="vert-group">
				<div class="card">
					<h3>Announcement</h3>
					<hr>
					<!--TODO: maybe update the title?-->
					<p>
						Hello, and again welcome to the Aperture Science Enrichment Center. We are currently experiencing technical
						difficulties due to the nature of potentially apocalyptic situations beyond our control. However, thanks to
						emergency protocols, testing can continue.<br><br>

						The portal will open, and testing will begin in 3... 2... 1...
					</p>
				</div>
				<div class="card">
					<h3>Activity</h3>
					<hr>
					<div class="card-content scroll-flex-col">
						<div class="user-card">
							<div class="namepanel">
								<img class="icon" src="https://placehold.co/55x55" />
								<p>
									<strong>Display or Nickname</strong><br>
									Username
								</p>
							</div>
							<div class="activity-card">
								<img class="icon" src="https://placehold.co/65x65" />
								<p>
									Playing: <strong>Some Game</strong><br>
									Playing for: 30 mins
								</p>
							</div>
						</div>
					</div>
				</div>
			</div>
			<div class="vert-group">
				<div class="card post">
					<div class="header">
						<img class="icon" src="https://placehold.co/55x55" />
						<p>
							<strong>Display Name</strong><br>
						</p>
					</div>
					<hr>
					<p>
						Looking forward to being able to use Slang!	
					</p>
					<br>
					<div class="image-group">
						<img class="thumb" src="https://placehold.co/655x300" width="655"/>
						<!--TODO: maybe make a cool looking placeholder image?-->
					</div>
					<br>
					<div class="stats">
						<a class="stat stat-likes"><span id="post-xyz-likes">4.2k</span> <i class="bi bi-heart"></i></a>
						<a class="stat stat-reposts"><span id="post-xyz-rep">1.3k</span> <i class="bi bi-repeat"></i></a>
						<a class="stat stat-comments"><span id="post-xyz-com">402</span> <i class="bi bi-chat-right"></i></a>
					</div>
				</div>
			</div>
			<div class="vert-group">
				<div class="card">
					<h3>Recent Pings</h3>
				</div>
				<div class="card">
					<h3>Discover</h3>
				</div>
			</div>
		</div>
	</div>
</template>

<style scoped>
/****BEGIN: to be abstracted */

/**Post card */
div.header {
	display: flex;
	flex-direction: row;
	align-items: center;
	gap: 10px;
}

div.stats {
	display: flex;
	flex-direction: row;
	gap: 10px;
	justify-content: flex-end;
}

a.stat {
	text-decoration: none;
	font-size: 14pt;
	background-color: #2226;
	padding: 8px;
	border-radius: 5px;
	transition-property: color, background;
	transition-duration: 100ms;
}

a.stat:hover {
	background-color: #2229;
	cursor: pointer;
}

a.stat-likes:hover {
	color: #f33;
}
a.stat-reposts:hover {
	color: #3f3;
}

/**Activity card */
div.user-card {
	background-color: var(--theme1-color-3);
	border-radius: 5px;
	padding: 5px;
}

img.icon {
	border-radius: 10px;
}

div.card-content {
	padding: 5px;
	max-height: 300px;
	overflow-y: scroll;
}

div.scroll-flex-col {
	display: flex;
	flex-direction: column;
}

div.namepanel {
	display: flex;
	flex-direction: row;
	align-items: center;
	margin-bottom: 5px;
	gap: 3px;
}

div.activity-card {
	display: flex;
	flex-direction: row;
	align-items: center;
	justify-content: center;
	gap: 3px;
}

/****END: to be abstracted */

div.content {
	margin: 30px auto;
	width: 70%;
	height: 100%;
	display: flex;
	flex-direction: column;
	align-items: center;
}

div.content h2 {
	font-size: 24pt;
	margin-bottom: 2rem;
}

div.grid {
	display: grid;
	grid-template-columns: 35% 50% 35%;
	justify-content: center;
	gap: 10px;
	width: 100%;
	height: 80%;
}

div.vert-group {
	display: flex;
	flex-direction: column;
	overflow-y: scroll;
	height: fit-content;
	max-height: 100%;
	gap: 5px;
}

div.card {
	padding: 8px;
	max-width: 100%;
	background-color: var(--theme1-color-4);
	border-radius: 5px;
}
</style>
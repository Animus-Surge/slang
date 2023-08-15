<script setup lang="ts">
const route = useRoute();

//TODO: route stuff and auto population of stuff
</script>

<template>
    <div class="template-root">
		<Channellist />
		<div class="messenger">
			<div class="message-panel">
				<div class="inset-scroll">
					<div class="message">
						<img class="icon" src="https://placehold.co/45x45/red/red" />
						<div class="text">
							<h4>Username</h4>
							<p>
								Hewwos! This is a message test. How is everyone doing today?<br>
								AaBbCcDdEe<br>
								FfGgHhIiJj<br>
								KkLlMmNnOo<br>
								PpQqRrSsTt<br>
								UuVvWwXxYy<br>
								Zz12345678<br>
								90 heheheh
							</p>
						</div>
					</div>
					<div class="message">
						<img class="icon" src="https://placehold.co/45x45/blue/blue" />
						<div class="text">
							<h4>Username2</h4>
							<p>
								Hi there!
							</p>
						</div>
					</div>
				</div>
			</div>
			<div class="chatbar">
				<form>
					<input type="text" id="message-box">
					<button class="chatbar-send">Send</button>
				</form>
			</div>
		</div>
	</div>
</template>

<style scoped>
button.chatbar-send {
	display: block;
	padding: 8px;
	background-color: #111;
	border-radius: 5px;
	transition-property: background-color, color;
	transition-duration: 100ms;
	border: none;
	color: #ccc;
	font-size: 12pt;
}

button.chatbar-send:hover {
	background-color: #555;
	text-decoration: none;
	color: #c90;
}

div.chatbar form {
	background-color: #2228;
	padding: 15px;
	display: flex;
	flex-direction: row;
	justify-content: center;
	align-items: center;
	gap: 8px;
	justify-self: end;
}

div.chatbar form input {
	width: 75%;
	display: block;
	padding: 8px;
	border-radius: 5px;
	border: none;
	background-color: #222;
	color: #ccc;
	font-size: 14pt;
	transition-property: background-color;
	transition-duration: 100ms;
}

div.chatbar form input:hover {
	background-color: #333;
}

div.template-root {
	display: flex;
	flex-direction: row;
	width: 100%;
}

div.messenger {
	background-color: #2225;
	display: flex;
	flex-direction: column;
	justify-content: space-between;
	width: 100%;
	height: 100%;
}

div.message-panel {
	padding: 10px;
}

div.inset-scroll {
	display: flex;
	flex-direction: column;
	gap: 12px;

}

div.message {
	display: flex;
	flex-direction: row;
	gap: 15px;
	align-items: flex-start;
}

img.icon {
	border-radius: 50%;
}
</style>
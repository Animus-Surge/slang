<script setup lang="ts">
</script>

<template>
	<div class="channel-list">
		<div class="bar-header">
			<h3>Group Name</h3>
			<a>Hide</a>
		</div>
		<div class="channel-group">
			<h4>Channel Group Name</h4>
			<a class="channel" href="/groups/1/1"><span>#</span> channel-name</a>
			<a class="channel" href="#"><span>#</span> channel-name</a>
			<a class="channel" href="#"><span>#</span> channel-name</a>
			<a class="channel" href="#"><span>#</span> channel-name</a>
			<a class="channel" href="#"><span>#</span> channel-name</a>
			<a class="channel" href="#"><span>#</span> channel-name</a>
			<a class="channel" href="#"><span>#</span> channel-name</a>
		</div>
		<div class="channel-group">
			<h4>Channel Group Name</h4>
			<a class="channel" href="#"><span>#</span> channel-name</a>
			<a class="channel" href="#"><span>#</span> channel-name</a>
		</div>
	</div>
</template>

<style scoped>

div.channel-list {
    padding: 8px;
    display: flex;
    flex-direction: column;
    background-color: #2229;
    height: 100%;
    gap: 8px;
	width: 200px;
	overflow-x: hidden;
}

div.bar-header {
	display: flex;
	flex-direction: row;
	justify-content: space-between;
	align-items: center;
}

div.channel-group {
    display: flex;
    flex-direction: column;
    gap: 3px;
}

a.channel {
    color: #aaa;
    display: block;
    padding: 5px;
    transition-property: background-color;
    transition-duration: 100ms;
    border-radius: 3px;
}

a.channel.selected {
    background-color: #ca08;
    color: #fff;
}

a.channel:hover {
    background-color: #ff05;
    color: #fff;
}

a.channel span {
    color: #999;
}

</style>
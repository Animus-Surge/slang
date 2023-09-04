<script setup lang="ts">
const activeId = useState('activeId')
onMounted(() => activeId.value = -2)
</script>

<template>
	<div class="content">
		<div class="scroll-detail">
			<!--TODO abstract div.friendview-->
			<div class="friend">
				<div>
					<img class="icon" src="https://placehold.co/55x55"/>
					<p>Display Name <span>username</span></p>
				</div>
				<div class="actions">
					<a class="action">
						<i class="bi bi-chat-right-text"></i>
					</a>

					<a class="action">
						<i class="bi bi-telephone-outbound"></i>
					</a>

					<a class="action warn">
						<i class="bi bi-person-dash"></i>
					</a>
					
				</div>
			</div>
		</div>
	</div>
</template>

<style scoped>
/* #region TO BE ABSTRACTED */
/* BEING: to be abstracted */

div.friend {
	display: flex;
	flex-direction: row;
	align-items: center;
	justify-content: space-between;
	gap: 10px
}

div.friend div {
	display: flex;
	flex-direction: row;
	gap: 10px;
	align-items: center;
}

p span {
	font-style: italic;
	color: #aaa;
}

div.actions {
	float: right;
	display: flex;
	flex-direction: row;
	align-items: center;
	width: fit-content;
	gap: 8px;
}

a.action {
	font-size: 18pt;
	background-color: #0005;
	padding: 3px 4px;
	border-radius: 8px;

	transition-property: color, background-color;
	transition-duration: 100ms;
}

a.action:hover {
	background-color: #0008;
	color: #fff;
}

.warn:hover {
	background-color: #f003 !important;
	color: #f00 !important;
}

/* END */
/* #endregion */

div.content {
	margin: auto;
	width: 70%;
	height: 70%;
}

div.scroll-detail {
	display: flex;
	flex-direction: column;
}

img.icon {
	border-radius: 8px;
}

</style>
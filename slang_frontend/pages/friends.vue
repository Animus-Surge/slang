<script setup lang="ts">
const activeId = useState('activeId')
onMounted(() => activeId.value = -2)
</script>

<template>
	<h2>Friends</h2>
</template>
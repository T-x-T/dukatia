<template>
	<div>
		<p>{{ text }}</p>
	</div>
</template>

<script lang="ts">
export default {
	props: {
		text: {
			type: String,
			required: true,
		},
	}
}
</script>

<style lang="sass" scoped>
p
	white-space: pre-line
</style>
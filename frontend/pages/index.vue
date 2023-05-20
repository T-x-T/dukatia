<template>
	<div>
		<Dashboards />
	</div>
</template>

<script lang="ts">
export default {
	data: () => ({
		total_per_currency: null
	}),
}
</script>

<style lang="sass" scoped>

div#grid
	display: grid
	width: 100%
	grid-template-columns: repeat(auto-fit, minmax(10%, 1fr))
	grid-auto-rows: 100px
	align-items: stretch
	justify-items: stretch
	grid-gap: 10px

div.gridItem
	padding: 10px

div.small
	grid-column: span 1
	grid-row: span 1

div.medium
	grid-column: span 2
	grid-row: span 4

div.large
	grid-column: span 4
	grid-row: span 4
	
</style>
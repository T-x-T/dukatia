<template>
	<div>
		<h2>{{ dashboard_data.name }}</h2>
		<p v-if="dashboard_data.description">{{ dashboard_data.description }}</p>
		<div id="grid">
			<div v-for="(chart, index) in charts" :class="`gridItem ${chart.grid_size}`">
				<Chart 
					:chart_options="chart"
				/>
			</div>
		</div>
	</div>
</template>

<script lang="ts" setup>
const charts: ChartOptions[] = [(await useFetch("/api/v1/charts/0")).data.value];
</script>

<script lang="ts">
export default {
	props: {
		dashboard_data: {
			type: Object as PropType<Dashboard>,
			required: true,
		}
	}
}
</script>

<style lang="sass" scoped>

div#grid
	display: grid
	width: 100%
	grid-auto-rows: 10%
	grid-auto-columns: 10%
	align-items: stretch
	justify-items: stretch
	grid-gap: 10px

div.gridItem
	padding: 10px
	overflow: scroll

div.small
	grid-column: span 1
	grid-row: span 1

div.medium
	grid-column: span 2
	grid-row: span 2

div.large
	grid-column: span 4
	grid-row: span 2
	
</style>
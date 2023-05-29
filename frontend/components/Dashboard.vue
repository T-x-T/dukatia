<template>
	<div>
		<h3>{{ dashboard_data.name }}</h3>
		<div id="grid">
			<div v-for="(chart, index) in charts" class="gridItem" :style="`grid-column: ${chart.top_left_x + 1} / ${chart.bottom_right_x + 1}; grid-row: ${chart.top_left_y + 1} / ${chart.bottom_right_y + 1}`">
				<div id="chart_wrapper">
					<Chart 
						:chart_options="chart"
						v-on:change_size="update"
						v-on:deleted="update"
					/>
				</div>
			</div>
		</div>
	</div>
</template>

<script lang="ts">
export default {
	data: () => ({
		charts: [] as ChartOptions[],
	}),

	props: {
		dashboard_data: {
			type: Object as PropType<Dashboard>,
			required: true,
		}
	},

	async created() {
		await this.update();
	},

	methods: {
		async update() {
			this.charts = await $fetch("/api/v1/dashboards/0/charts");
		}
	}
}
</script>

<style lang="sass" scoped>

div#grid
	display: grid
	width: 100%
	grid-auto-rows: 200px
	grid-template-columns: repeat(10, 1fr)
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

div#chart_wrapper
	height: 100%
	
</style>
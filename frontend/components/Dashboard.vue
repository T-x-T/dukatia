<template>
	<div id="main">
		<h3>{{ dashboard_data.name }}</h3>
		
		<button v-if="!add_chart_open" id="add_chart" class="mobile_hidden" @click="add_chart_open = true">Add Chart</button>
		<button id="edit_button" class="mobile_hidden" @click="edit_mode = !edit_mode">Edit mode</button>
		<div v-if="add_chart_open" id="add_chart_box">
			<ChartOptions 
				@back="(add_chart_open = false) || update()"
			/>
		</div>

		<div id="grid">
			<div v-for="(chart, index) in charts" :key="index" class="gridItem" :style="`grid-column: ${chart.top_left_x + 1} / ${chart.bottom_right_x + 1}; grid-row: ${chart.top_left_y + 1} / ${chart.bottom_right_y + 1}`">
				<div id="edit_mode" v-if="edit_mode">
					<button class="resize_top_larger" @click="resize_top_larger(chart)">
						<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6"><path stroke-linecap="round" stroke-linejoin="round" d="M15 11.25l-3-3m0 0l-3 3m3-3v7.5M21 12a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>
					</button>
					<button class="resize_right_larger" @click="resize_right_larger(chart)">
						<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6"><path stroke-linecap="round" stroke-linejoin="round" d="M15 11.25l-3-3m0 0l-3 3m3-3v7.5M21 12a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>
					</button>
					<button class="resize_bottom_larger" @click="resize_bottom_larger(chart)">
						<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6"><path stroke-linecap="round" stroke-linejoin="round" d="M15 11.25l-3-3m0 0l-3 3m3-3v7.5M21 12a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>
					</button>
					<button class="resize_left_larger" @click="resize_left_larger(chart)">
						<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6"><path stroke-linecap="round" stroke-linejoin="round" d="M15 11.25l-3-3m0 0l-3 3m3-3v7.5M21 12a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>
					</button>
					<button class="resize_top_smaller" @click="resize_top_smaller(chart)">
						<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6"><path stroke-linecap="round" stroke-linejoin="round" d="M15 11.25l-3-3m0 0l-3 3m3-3v7.5M21 12a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>
					</button>
					<button class="resize_right_smaller" @click="resize_right_smaller(chart)">
						<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6"><path stroke-linecap="round" stroke-linejoin="round" d="M15 11.25l-3-3m0 0l-3 3m3-3v7.5M21 12a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>
					</button>
					<button class="resize_bottom_smaller" @click="resize_bottom_smaller(chart)">
						<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6"><path stroke-linecap="round" stroke-linejoin="round" d="M15 11.25l-3-3m0 0l-3 3m3-3v7.5M21 12a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>
					</button>
					<button class="resize_left_smaller" @click="resize_left_smaller(chart)">
						<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6"><path stroke-linecap="round" stroke-linejoin="round" d="M15 11.25l-3-3m0 0l-3 3m3-3v7.5M21 12a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>
					</button>
				</div>
				<div id="chart_wrapper" v-if="!chart.disabled">
					<Chart 
						:chart_options="chart"
						@deleted="update"
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
		add_chart_open: false,
		edit_mode: false,
		grid_width: 10,
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
		},

		resize_top_smaller(chart: ChartOptions) {
			if(this.get_height(chart) > 1) {
				chart.top_left_y++;
				this.reload_chart(chart);
			}
		},

		resize_right_smaller(chart: ChartOptions) {
			if(this.get_width(chart) > 1) {
				chart.bottom_right_x--;
			}
		},

		resize_bottom_smaller(chart: ChartOptions) {
			if(this.get_height(chart) > 1) {
				chart.bottom_right_y--;
				this.reload_chart(chart);
			}
		},

		resize_left_smaller(chart: ChartOptions) {
			if(this.get_width(chart) > 1) {
				chart.top_left_x++;
			}
		},

		resize_top_larger(chart: ChartOptions) {
			if(chart.top_left_y > 0 && !this.collides_up(chart)) {
				chart.top_left_y--;
			}
		},

		resize_right_larger(chart: ChartOptions) {
			if(chart.bottom_right_x < 10 && !this.collides_right(chart)) {
				chart.bottom_right_x++;
			}
		},

		resize_bottom_larger(chart: ChartOptions) {
			if(!this.collides_down(chart)) {
				chart.bottom_right_y++;
			}
		},

		resize_left_larger(chart: ChartOptions) {
			if(chart.top_left_x > 0 && !this.collides_left(chart)) {
				chart.top_left_x--;
			}
		},

		get_width(chart: ChartOptions) {
			return chart.bottom_right_x - chart.top_left_x;
		},

		get_height(chart: ChartOptions) {
			return chart.bottom_right_y - chart.top_left_y;
		},

		collides_up(chart: ChartOptions) {
			return this.charts.map(x => {
				return x.bottom_right_y === chart.top_left_y && (x.bottom_right_x > chart.top_left_x && x.top_left_x < chart.bottom_right_x);
			})
			.filter(x => x).length > 0;
		},

		collides_down(chart: ChartOptions) {
			return this.charts.map(x => {
				return x.top_left_y === chart.bottom_right_y && (x.bottom_right_x > chart.top_left_x && x.top_left_x < chart.bottom_right_x);
			})
			.filter(x => x).length > 0;
		},

		collides_right(chart: ChartOptions) {
			return this.charts.map(x => {
				return x.top_left_x === chart.bottom_right_x && (x.bottom_right_y > chart.top_left_y && x.top_left_y < chart.bottom_right_y);
			})
			.filter(x => x).length > 0;
		},

		collides_left(chart: ChartOptions) {
			return this.charts.map(x => {
				return x.bottom_right_x === chart.top_left_x && (x.bottom_right_y > chart.top_left_y && x.top_left_y < chart.bottom_right_y);
			})
			.filter(x => x).length > 0;
		},

		reload_chart(chart: ChartOptions) {
			chart.disabled = true;
			this.$nextTick(() => chart.disabled = false);
		},
	}
}
</script>

<style lang="sass" scoped>

div#main
	margin: 10px

div#grid
	display: grid
	width: 100%
	grid-auto-rows: 200px
	grid-template-columns: repeat(10, 1fr)
	align-items: stretch
	justify-items: stretch
	grid-gap: 10px
	@media screen and (max-width: 800px)
		display: flex
		flex-direction: column
		div.gridItem
			min-height: 50dvh

div.gridItem
	padding: 10px
	overflow: scroll
	position: relative

div#chart_wrapper
	height: 100%

button#add_chart
	position: absolute
	z-index: 100
	bottom: 0
	right: 0

button#edit_button
	position: absolute
	bottom: 40px
	right: 0
	z-index: 100

div#edit_mode
	display: grid
	position: absolute
	z-index: 20
	top: 0
	right: 0
	bottom: 0
	left: 0
	grid-template-columns: 32px 32px 1fr 32px 1fr 32px 32px
	grid-template-rows: 32px 32px 1fr 32px 1fr 32px 32px
	&:hover
		svg
			opacity: 0.6
	button
		background: none
		box-shadow: none
		width: 100%
		height: 100%
		margin: 0
		padding: 0
		&:hover
			background: none
		svg
			height: 32px
			opacity: 0.2
			&:hover
				opacity: 1

button.resize_top_larger
	grid-area: 1 / 4 / 1 / 4

button.resize_right_larger
	grid-area: 4 / 7 / 4 / 7
	rotate: 90deg

button.resize_bottom_larger
	grid-area: 7 / 4 / 7 / 4
	rotate: 180deg

button.resize_left_larger
	grid-area: 4 / 1 / 4 / 1
	rotate: 270deg

button.resize_top_smaller
	grid-area: 2 / 4 / 2 / 4
	rotate: 180deg

button.resize_right_smaller
	grid-area: 4 / 6 / 4 / 6
	rotate: 270deg

button.resize_bottom_smaller
	grid-area: 6 / 4 / 6 / 4

button.resize_left_smaller
	grid-area: 4 / 2 / 4 / 2
	rotate: 90deg
	
</style>
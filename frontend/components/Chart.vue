<template>
	<div :id="show_options ? 'outer_wrapper' : ''" class="full_size">
		<div :class="show_options ? 'fullscreen_wrapper' : 'wrapper'" :id="show_options ? 'popup' : ''">
			<button v-if="show_options" class="special_button" id="close_button" @click="() => {show_options = false; fullscreen = false; reset_chart();}">
				<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6"><path stroke-linecap="round" stroke-linejoin="round" d="M9 9V4.5M9 9H4.5M9 9L3.75 3.75M9 15v4.5M9 15H4.5M9 15l-5.25 5.25M15 9h4.5M15 9V4.5M15 9l5.25-5.25M15 15h4.5M15 15v4.5m0-4.5l5.25 5.25" /></svg>
			</button>
			<button v-if="show_options && fullscreen" id="fullscreened_edit_button" class="special_button" @click="fullscreen = false">
				<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6"><path stroke-linecap="round" stroke-linejoin="round" d="M16.862 4.487l1.687-1.688a1.875 1.875 0 112.652 2.652L6.832 19.82a4.5 4.5 0 01-1.897 1.13l-2.685.8.8-2.685a4.5 4.5 0 011.13-1.897L16.863 4.487zm0 0L19.5 7.125" /></svg>
			</button>
			<div v-if="show_options && !fullscreen" id="chart_options" class="wrapper">
				<ChartOptions 
					:chart_options="options"
					@deleted="(show_options = false) || $emit('deleted')"
					@update="(new_options: ChartOptions) => {options = {...new_options}; reset_chart()}"
				/>
			</div>
			<div class="chart_wrapper">
				<button v-if="!show_options" id="fullscreen_button" class="mobile_hidden special_button" @click="() => {show_options = true; fullscreen = true;}">
					<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6"><path stroke-linecap="round" stroke-linejoin="round" d="M3.75 3.75v4.5m0-4.5h4.5m-4.5 0L9 9M3.75 20.25v-4.5m0 4.5h4.5m-4.5 0L9 15M20.25 3.75h-4.5m4.5 0v4.5m0-4.5L15 9m5.25 11.25h-4.5m4.5 0v-4.5m0 4.5L15 15" /></svg>
				</button>
				<button v-if="!show_options" id="edit_button" class="mobile_hidden special_button" @click="show_options = true">
					<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6"><path stroke-linecap="round" stroke-linejoin="round" d="M16.862 4.487l1.687-1.688a1.875 1.875 0 112.652 2.652L6.832 19.82a4.5 4.5 0 01-1.897 1.13l-2.685.8.8-2.685a4.5 4.5 0 011.13-1.897L16.863 4.487zm0 0L19.5 7.125" /></svg>
				</button>
				<h5>{{ options.title }}</h5>
				<div :class="show_options ? 'fullscreen_chart' : 'chart'" v-if="options.chart_type == 'text' && show_chart">
					<ChartText
						v-if="chart_data.text"
						:text="chart_data.text"
					/>
				</div>
				<div :class="show_options ? 'fullscreen_chart' : 'chart'" v-if="options.chart_type == 'pie' && show_chart">
					<ChartPie
						v-if="chart_data.pie"
						:pie="chart_data.pie"
						:key="key"
					/>
				</div>
				<div :class="show_options ? 'fullscreen_chart' : 'chart'" v-if="options.chart_type == 'line' && show_chart">
					<ChartLine
						v-if="chart_data.line"
						:line="chart_data.line"
						:key="key"
					/>
				</div>
	
				<div id="controls">
					<ChartControl 
						v-if="options.chart_type == 'pie' || options.chart_type == 'line'"
						@update="update_date"
						:prop_date_range="typeof options.date_range === 'number' ? options.date_range.toString() : '0'"
						:prop_date_period="options.date_period"
					/>
				</div>
			</div>
		</div>
	</div>
</template>

<script lang="ts">
export default {
	data: () => ({
		chart_data: {} as any,
		options: {} as ChartOptions,
		key: 0,
		show_options: false,
		fullscreen: false,
		show_chart: true,
		query: "",
		last_data_update: 0,
	}),

	emits: ["deleted"],

	props: {
		chart_options: {
			type: Object as PropType<ChartOptions>,
			required: true,
		}
	},

	created() {
		this.options = structuredClone(toRaw(this.chart_options));
	},

	async mounted() {
		if(this.options.chart_type == "text") this.chart_data = await $fetch(`/api/v1/charts/${this.options.id}/data`);
	},

	methods: {
		async update_date(options: {from_date: string, to_date: string, date_period: string}) {
			this.query = `?from_date=${new Date(options.from_date).toISOString()}&to_date=${new Date(options.to_date).toISOString()}&date_period=${options.date_period}`;
			if((Date.now().valueOf() - this.last_data_update) > 2) {
				this.chart_data = await $fetch(`/api/v1/charts/${this.options.id}/data${this.query}`);
			 	this.last_data_update = Date.now().valueOf();
			}
			this.key++;
		},

		async reload() {
			await this.reset_chart();
			this.options = await $fetch(`/api/v1/charts/${this.options.id}`);
			this.show_options = false
		},

		async reset_chart() {
			this.chart_data = await $fetch(`/api/v1/charts/${this.options.id}/data${this.query}`);
			this.last_data_update = Date.now().valueOf();
			this.show_chart = false;
			this.$nextTick(() => this.show_chart = true);
		},
	},
}
</script>

<style lang="sass" scoped>
div.wrapper
	height: 100%
	position: relative
	display: flex
	flex-direction: column
	&:hover
		#edit_button
			opacity: 0.8
		#fullscreen_button
			opacity: 0.8

div.fullscreen_wrapper
	padding: 1%
	display: flex
	gap: 1%
	div#chart_options
		width: 30%

div#outer_wrapper
	position: fixed
	z-index: 1000
	top: 0
	left: 0
	width: 100vw
	height: 100vh
	display: flex
	align-items: center
	justify-content: center
	background: rgba(0, 0, 0, 0.5)
	backdrop-filter: blur(5px)

div.chart_wrapper
	display: grid
	grid-template-columns: 100%
	grid-template-rows: max-content 1fr max-content
	height: 100%
	width: 100%

h5
	text-align: center

div.fullscreen_chart
	height: 75vh
	width: 75vw

div.full_size
	height: 100%
	width: 100%

#edit_button
	opacity: 0.2
	left: 32px

#fullscreen_button
	opacity: 0.2

#close_button
	z-index: 3000
	opacity: 0.8
	&:hover
		opacity: 1

#fullscreened_edit_button
	z-index: 3000
	margin-left: 30px
	opacity: 0.8
	&:hover
		opacity: 1

.special_button
	position: absolute
	width: fit-content
	box-shadow: none
	background: none
	&:hover
		background: none
		opacity: 1 !important
	svg
		width: 24px
</style>
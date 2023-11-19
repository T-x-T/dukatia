<template>
	<div :id="show_options ? 'outer_wrapper' : ''" class="full_size">
		<div :class="show_options ? 'fullscreen_wrapper' : 'wrapper'" :id="show_options ? 'popup' : ''">
			<div v-if="show_options" id="chart_options" class="wrapper">
				<ChartOptions 
				:chart_options="options"
				@back="reload"
				@change_size="$emit('change_size')"
				@deleted="(show_options = false) || $emit('deleted')"
				/>
			</div>
			<div class="chart_wrapper">
				<button v-if="!show_options" id="edit_button" class="mobile_hidden" @click="show_options = true">Edit</button>
				<h5>{{ options.title }}</h5>
				<div :class="show_options ? 'fullscreen_chart' : 'chart'" v-if="options.chart_type == 'text'">
					<ChartText
						v-if="chart_data.text"
						:text="chart_data.text"
					/>
				</div>
				<div :class="show_options ? 'fullscreen_chart' : 'chart'" v-if="options.chart_type == 'pie'">
					<ChartPie
						v-if="chart_data.pie"
						:pie="chart_data.pie"
						:key="key"
					/>
				</div>
				<div :class="show_options ? 'fullscreen_chart' : 'chart'" v-if="options.chart_type == 'line'">
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
						:default_date_range="(Number.isInteger(options.date_range) ? options.date_range : 0).toString()"
						:default_date_period="options.date_period"
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
	}),

	emits: ["change_size", "deleted"],

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
			let query = `?from_date=${new Date(options.from_date).toISOString()}&to_date=${new Date(options.to_date).toISOString()}&date_period=${options.date_period}`;
			if (Number.isInteger(this.options.asset_id)) query += `&asset_id=${this.options.asset_id}`;
 			this.chart_data = await $fetch(`/api/v1/charts/${this.options.id}/data${query}`);
			this.key++;
		},

		async reload() {
			this.options = await $fetch(`/api/v1/charts/${this.options.id}`);
			this.show_options = false
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
			visibility: visible

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
	position: absolute
	width: fit-content
	visibility: hidden
</style>
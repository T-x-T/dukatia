<template>
	<div id="chart_wrapper">
		<h5>{{ chart_options.title }}</h5>
		<div v-if="chart_options.chart_type == 'text'">
			<ChartText
				v-if="chart_data.text"
				:text="chart_data.text"
			/>
		</div>
		<div v-if="chart_options.chart_type == 'pie'">
			<ChartPie
				v-if="chart_data.pie"
				:pie="chart_data.pie"
				:key="key"
			/>
		</div>
		<div id="line_chart" v-if="chart_options.chart_type == 'line'">
			<ChartLine
				v-if="chart_data.line"
				:line="chart_data.line"
				:key="key"
			/>
		</div>

		<div id="controls">
			<DateControl 
				v-if="chart_options.chart_type == 'pie' || chart_options.chart_type == 'line'"
				v-on:update="update_date"
				default_date_range="0"
			/>
<!-- 			<div v-if="showOnlyParentsToggle">
				<label for="parent">Only Parents:</label>
				<input type="checkbox" id="parent" v-model="only_parents">
			</div> -->
		</div>
	</div>
</template>

<script lang="ts">
export default {
	data: () => ({
		chart_data: {} as any,
		key: 0,
	}),

	props: {
		chart_options: {
			type: Object as PropType<ChartOptions>,
			required: true,
		}
	},

	async mounted() {
		if(this.chart_options.chart_type == "text") this.chart_data = await $fetch(`/api/v1/charts/${this.chart_options.id}/data`);
	},

	methods: {
		async update_date(dates: {from_date: string, to_date: string}) {
			this.chart_data = await $fetch(`/api/v1/charts/${this.chart_options.id}/data?from_date=${new Date(dates.from_date).toISOString()}&to_date=${new Date(dates.to_date).toISOString()}`);
			this.key++;
		}
	},
}
</script>

<style lang="sass" scoped>
div#chart_wrapper
	display: flex
	flex-direction: column
	height: 100%

h5
	text-align: center

div#line_chart
	flex-grow: 1
</style>
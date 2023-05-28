<template>
	<div class="wrapper">
		<div v-if="!showOptions" class="wrapper" id="chart_wrapper">
			<button id="edit_button" @click="showOptions = true">Edit</button>
			<h5>{{ options.title }}</h5>
			<div v-if="options.chart_type == 'text'">
				<ChartText
					v-if="chart_data.text"
					:text="chart_data.text"
				/>
			</div>
			<div v-if="options.chart_type == 'pie'">
				<ChartPie
					v-if="chart_data.pie"
					:pie="chart_data.pie"
					:key="key"
				/>
			</div>
			<div id="line_chart" v-if="options.chart_type == 'line'">
				<ChartLine
					v-if="chart_data.line"
					:line="chart_data.line"
					:key="key"
				/>
			</div>
	
			<div id="controls">
				<ChartControl 
					v-if="options.chart_type == 'pie' || options.chart_type == 'line'"
					v-on:update="update_date"
					:default_date_range="(Number.isInteger(options.date_range) ? options.date_range : 0).toString()"
					:default_date_period="options.date_period"
				/>
			</div>
		</div>

		<div v-if="showOptions" class="wrapper">
			<ChartOptions 
				:chart_options="options"
				v-on:back="reload"
			/>
		</div>
	</div>
</template>

<script lang="ts">
export default {
	data: () => ({
		chart_data: {} as any,
		options: {} as ChartOptions,
		key: 0,
		showOptions: false,
	}),

	props: {
		chart_options: {
			type: Object as PropType<ChartOptions>,
			required: true,
		}
	},

	created() {
		this.options = {...this.chart_options};
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
			this.showOptions = false
		},
	},
}
</script>

<style lang="sass" scoped>
div.wrapper
	height: 100%

div#chart_wrapper
	display: flex
	flex-direction: column
	&:hover
		#edit_button
			visibility: visible

h5
	text-align: center

div#line_chart
	flex-grow: 1

#edit_button
	position: absolute
	width: fit-content
	visibility: hidden
</style>
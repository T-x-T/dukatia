<template>
	<div>
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
			/>
		</div>
	</div>
</template>

<script lang="ts">
export default {
	data: () => ({
		chart_data: {} as any
	}),

	props: {
		chart_options: {
			type: Object as PropType<ChartOptions>,
			required: true,
		}
	},

	async mounted() {
		this.chart_data = await $fetch(`/api/v1/charts/${this.chart_options.id}/data`);
	}
}
</script>

<style lang="sass" scoped>
h5
	text-align: center
</style>
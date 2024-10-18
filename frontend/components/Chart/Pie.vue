<template>
	<div>
		<p v-if="no_data">No data</p>
		<PieChart
			v-if="!no_data"
			:chartData="chart_data"
			:chartOptions="chart_options"
		/>
	</div>
</template>

<script lang="ts">
export default {
	data: () => ({
		no_data: false,
		chart_options: {
			responsive: true,
			maintainAspectRatio: false,
			plugins: {
				legend: {
					position: "bottom",
					labels: {
						color: "#fff"
					}
				},
				title:  {
					display: false,
				},
				tooltip: {
					callbacks: {}
				}
			}
		},
		chart_data: {
			datasets: [{
				data: [] as number[],
				backgroundColor: [
					"#E84444",
					"#F79148",
					"#45DCCA",
					"#619CF5",
					"#E26FFF",
					"#F74887",
					"#F7EA48",
				],
			}],
			labels: [] as string[]
		},
	}),

	props: {
		pie: {
			type: Object,
			required: true,
		},
	},

	created() {
		if(this.$colorMode.value == "light") {
			this.chart_options.plugins.legend.labels.color = "#000";
		}
		this.update();
	},

	methods: {

		update() {
			this.no_data = this.pie.datasets.length === 0;

			this.chart_data.datasets[0].data = this.pie.datasets.map((x: any) => x.data[x.data.length - 1]?.value);
			this.chart_data.labels = this.pie.datasets.map((x: any) => x.label);

			(this as any).chart_options.plugins.tooltip.callbacks.label = (context: any) => {
				return `${this.pie.datasets[context.dataIndex].label}: ${this.pie.datasets[context.dataIndex].data[this.pie.datasets[context.dataIndex].data.length - 1].label}`;
			}
		},
	}
}
</script>

<style lang="sass" scoped>
div
	height: 100%
</style>
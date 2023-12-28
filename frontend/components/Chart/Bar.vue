<template>
	<div>
		<BarChart
			:chartData="chart_data"
			:chartOptions="chart_options"
		/>
	</div>
</template>

<script lang="ts">
export default {
	data: () => ({
		chart_options: {
			responsive: true,
			maintainAspectRatio: false,
			indexAxis: "y",
			scales: {
				x: {
					stacked: true
				},
				y: {
					stacked: true
				}
			},
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
			datasets: [] as any[],
			labels: [] as string[],
		},
	}),

	props: {
		bar: {
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
		get_color(i: number) {
			const colors = [
				"#E84444",
				"#54D88D",
				"#F79148",
				"#45DCCA",
				"#619CF5",
				"#E26FFF",
				"#F74887",
				"#F7EA48",
			];

			while (i >= colors.length) i = i - colors.length;

			return colors[i];
		},

		update() {
			this.chart_data.labels = this.bar.datasets[0].data.map((x: any) => x.name);
			this.chart_data.datasets = this.bar.datasets.map((x: any, i: number) => ({
				data: x.data.map((y: any) => y.value),
				label: x.label,
				backgroundColor: this.get_color(i),
				borderColor: this.get_color(i),
			}));

			(this as any).chart_options.plugins.tooltip.callbacks.label = (context: any) => {
				return `${this.bar.datasets[context.datasetIndex].label}: ${this.bar.datasets[context.datasetIndex].data[context.dataIndex].label}`;
			}
		}
	}
}
</script>

<style lang="sass" scoped>
div
	height: 100%
</style>
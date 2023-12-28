<template>
	<div>
		<LineChart
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
			scales: {
				x: {
					type: "time",
					ticks: {
						fontColor: "#ddd"
					},
					gridLines: {
						color: "#fff2",
						drawBoder: false
					}
				},
				y: {
					ticks: {
						fontColor: "#ddd"
					},
					gridLines: {
						color: "#fff2",
						drawBorder: false
					}
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
			datasets: [] as any[]
		},
	}),

	props: {
		line: {
			type: Object,
			required: true,
		},
	},

	created() {
		if(this.$colorMode.value == "light") {
			this.chart_options.plugins.legend.labels.color = "#000";
			this.chart_options.scales.x.ticks.fontColor = "#111";
			this.chart_options.scales.x.gridLines.color = "#0002";
			this.chart_options.scales.y.ticks.fontColor = "#111";
			this.chart_options.scales.y.gridLines.color = "#0002";
		} else {
			this.chart_options.plugins.legend.labels.color = "#ffffff";
			this.chart_options.scales.x.ticks.fontColor = "#ffffff";
			this.chart_options.scales.x.gridLines.color = "#ffffff";
			this.chart_options.scales.y.ticks.fontColor = "#ffffff";
			this.chart_options.scales.y.gridLines.color = "#ffffff";
		}
		this.update();
	},

	methods: {
		get_color(i: number) {
			const colors = [
				"#E84444",
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
			this.chart_data.datasets = this.line.datasets.map((x: any, i: number) => ({
				label: x.label,
				data: x.data.map((y: any) => ({x: y.timestamp, y: y.value})),
				backgroundColor: this.get_color(i),
				borderColor: this.get_color(i),
			}));

			(this as any).chart_options.plugins.tooltip.callbacks.label = (context: any) => {
				return `${this.line.datasets[context.datasetIndex].label}: ${this.line.datasets[context.datasetIndex].data[context.dataIndex].label}`;
			}
		}
	}
}
</script>

<style lang="sass" scoped>
div
	height: 100%
</style>
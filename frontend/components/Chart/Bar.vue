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
			indexAxis: 'y',
			scales: {
				x: {
					ticks: {
						fontColor: "#ddd"
					},
					gridLines: {
						color: "#fff2",
						drawBorder: false
					},
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
			labels: [] as any,
			datasets: [] as any[]
		},
	}),

	props: {
		bar: {
			type: Object,
			required: true,
		},
	},

	created() {
		if(this.$colorMode.preference == "light") {
			this.chart_options.plugins.legend.labels.color = "#000";
			this.chart_options.scales.x.ticks.fontColor = "111";
			this.chart_options.scales.x.gridLines.color = "0002";
		}
		this.update();
	},

	methods: {
		update() {
			const colors = [
				"rgba(255, 99, 132, 0.2)",
				"rgba(75, 192, 192, 0.2)",
				"rgba(255, 205, 86, 0.2)",
				"rgba(255, 159, 64, 0.2)",
				"rgba(54, 162, 235, 0.2)",
				"rgba(153, 102, 255, 0.2)",
				"rgba(201, 203, 207, 0.2)",
			];
			this.bar.forEach((x: any, i: number) => {
				if(this.chart_data.labels.length === 0) x[1].forEach((y: any) => this.chart_data.labels.push(y.name));
				this.chart_data.datasets.push({
					data: x[1].map((y: any) => y.value),
					label: x[0],
					backgroundColor: colors[i],
				});
			});

			(this as any).chart_options.plugins.tooltip.callbacks.label = (context: any) => {
				return `${this.bar[context.datasetIndex][0]}: ${this.bar[context.datasetIndex][1][context.dataIndex].label}`;
			}
		}
	}
}
</script>

<style lang="sass" scoped>
div
	height: 100%
</style>
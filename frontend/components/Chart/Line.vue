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
					time: {
						unit: "day"
					},
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
			datasets: [

			] as any[]
		},
	}),

	props: {
		line: {
			type: Object,
			required: true,
		},
	},

	created() {
		if(this.$colorMode.preference == "light") {
			this.chart_options.plugins.legend.labels.color = "#000";
			this.chart_options.scales.x.ticks.fontColor = "#111";
			this.chart_options.scales.x.gridLines.color = "#0002";
			this.chart_options.scales.y.ticks.fontColor = "111";
			this.chart_options.scales.y.gridLines.color = "0002";
		}
		this.update();
	},

	methods: {
		update() {
			for (const dataset in this.line) {
				this.chart_data.datasets.push({
					label: this.line[dataset][0],
					data: Object.keys(this.line[dataset][1]).map(item => (
						{
							x: this.line[dataset][1][item].timestamp, 
							y: this.line[dataset][1][item].value
						}
					)),
				})
			}

			(this as any).chart_options.plugins.tooltip.callbacks.label = (context: any) => {
				let line = {} as any;
				Object.keys(this.line).forEach(x => line[this.line[x][0]] = this.line[x][1])

				return ` ${context.dataset.label}: ` + line[context.dataset.label].filter((x: any) => {
					return new Date(context.label.replaceAll(".", "")).toISOString() == new Date(x.timestamp).toISOString();
				})[0].label;
			}
		}
	}
}
</script>

<style lang="sass" scoped>
div
	height: 100%
</style>
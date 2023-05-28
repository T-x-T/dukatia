<template>
	<div>
		<PieChart
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
				data: [] as number[]
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
		if(this.$colorMode.preference == "light") {
			this.chart_options.plugins.legend.labels.color = "#000";
		}
		this.update();
	},

	methods: {
		update() {
			Object.keys(this.pie).forEach(x => {
				this.chart_data.datasets[0].data.push(this.pie[x][1][1]);
				this.chart_data.labels.push(this.pie[x][0]);
			});

			(this as any).chart_options.plugins.tooltip.callbacks.label = (context: any) => {
				return " " + this.pie.filter((x: any) => x[0] === context.label)[0][1][0];
			}
		},
	}
}
</script>
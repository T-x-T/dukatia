<template>
	<div>
		<p v-if="no_data">No data</p>
		<PieChart
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
		if(this.$colorMode.value == "light") {
			this.chart_options.plugins.legend.labels.color = "#000";
		}
		this.update();
	},

	methods: {
		update() {
			const colors = [
				"#E84444",
				"#F79148",
				"#45DCCA",
				"#619CF5",
				"#E26FFF",
				"#F74887",
				"#F7EA48",
			];

			let has_data = false;
			Object.keys(this.pie).forEach((x, i) => {
				if(this.pie[x][1][1]) has_data = true;
				this.chart_data.datasets[0].data.push(this.pie[x][1][1]);
				(this as any).chart_data.datasets[0].backgroundColor = colors;
				this.chart_data.labels.push(this.pie[x][0]);
			});
			this.no_data = !has_data;

			(this as any).chart_options.plugins.tooltip.callbacks.label = (context: any) => {
				return " " + this.pie.filter((x: any) => x[0] === context.label)[0][1][0];
			}
		},
	}
}
</script>

<style lang="sass" scoped>
div
	height: 100%
</style>
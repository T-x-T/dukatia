<template>
	<div id="container">
		<div id="title">
			<p>Balance per {{type}} over time</p>
		</div>
		<div v-if="loaded" id="chart">
			<LineChart
				:chartData="chartData"
				:chartOptions="chartOptions"
			/>
		</div>
		<div id="controls">
			<DateControl 
				default_date_range="7"
				v-on:update="updateDate"
			/>
		</div>
	</div>
</template>

<script>
export default {
	data: () => ({
		loaded: false,
		from_date: null,
		to_date: null,
		colors: {},
		chartData: {},
		chartOptions: {
			responsive: true,
			maintainAspectRatio: false,
			title:  {
				display: false,
			},
			scales: {
				xAxes: [{
					id: "x",
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
				}],
				yAxes: [{
					ticks: {
						fontColor: "#ddd"
					},
					gridLines: {
						color: "#fff2",
						drawBorder: false
					}
				}]
			},
			legend: {
				position: "bottom",
				labels: {
					fontColor: "#fff"
				}
			}
		}
	}),

	props: {
		type: String,
		api_path: String,
		label_property: String
	},

	async mounted() {
		if(this.$colorMode.preference == "light") {
			this.chartOptions.scales.xAxes[0].ticks.fontColor = "#111";
			this.chartOptions.scales.xAxes[0].gridLines.color = "#0002";
			this.chartOptions.scales.yAxes[0].ticks.fontColor = "111";
			this.chartOptions.scales.yAxes[0].gridLines.color = "0002";
			this.chartOptions.legend.labels.fontColor = "#000";
		}
		await this.update();
	},

	methods: {
		updateDate(dates) {
			this.from_date = dates.from_date;
			this.to_date = dates.to_date;
			this.update();
		},

		async update() {
			this.loaded = false;

			this.chartData = {
				datasets: []
			}

			let query = "";
			if(this.from_date && this.to_date) {
				query = `?from_date=${this.from_date}&to_date=${this.to_date}&`;
			}
			const api_data = await this.$axios.$get(this.api_path + query);
			this.$store.state[this.type].forEach((item, i) => {
				if(api_data[item.id]?.data.length > 0) {
					this.chartData.datasets.push({
						label: item[this.label_property],
						data: api_data[item.id].data.map(x => ({x: x.x, y: x.y / 100})), //TODO: not using minor_in_mayor 
						cubicInterpolationMode: "monotone",
						fill: false,
						borderColor: `rgba(0, 255, 255, ${(i + 1) / this.$store.state[this.type].length})`,
						borderWidth: 4,
						pointRadius: 2,
						pointBorderWidth: 4
					});
				}
			});
			
			this.loaded = true;
		}
	}
}
</script>

<style lang="sass" scoped>
div#container
	height: 100%
	display: flex
	flex-direction: column
	justify-content: flex-start

div#chart
	flex-shrink: 4
	min-height: 0

div#title > p
	text-align: center
</style>
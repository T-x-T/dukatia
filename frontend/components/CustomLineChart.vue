<template>
	<div id="container">
		<div id="title">
			<p>{{title}}</p>
		</div>
		<div v-if="loaded" id="chart">
			<LineChart
				:chartData="chartData"
				:chartOptions="chartOptions"
			/>
		</div>
		<div id="controls" v-if="!no_controls">
			<DateControl 
				default_date_range="7"
				v-on:update="updateDate"
			/>
			<div v-if="aggregated">
				<label for="period">Aggregation:</label>
				<select v-model="period" id="period">
					<option value="Monthly">Monthly</option>
					<option value="Quarterly">Quarterly</option>
					<option value="Yearly">Yearly</option>
				</select>
			</div>
		</div>
	</div>
</template>

<script>
export default {
	data: () => ({
		loaded: false,
		from_date: null,
		to_date: null,
		period: "Monthly",
		colors: ["#EC8A83", "#FFAD85", "#F9F176", "#8BE59D", "#6AB4F1", "#A983D8"],
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
		api_data: Object,
		label_property: String,
		aggregated: Boolean,
		title: String,
		no_controls: Boolean,
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

	watch: {
		period() {
			this.update();
		}
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

			let query = "?";
			if(this.from_date && this.to_date) {
				query += `from_date=${this.from_date}&to_date=${this.to_date}&`;
			}
			if(this.aggregated) {
				query += `period=${this.period}&`;
			}
			const api_data = this.api_path ? await this.$axios.$get(this.api_path + query) : this.api_data;

			if (!api_data) {
				return;
			}

			let j = 0;

			const common = {
				cubicInterpolationMode: "monotone",
				fill: false,
				borderWidth: 4,
				pointRadius: 2,
				pointBorderWidth: 4
			}

			if(this.type == "simple_monetary") {
				this.chartData.datasets.push({
					...common,
					label: "",
					data: Object.keys(api_data).map(item => ({x: item, y: api_data[item] / 100})), //TODO: not using minor_in_mayor 
					borderColor: this.colors[0],
					backgroundColor: this.colors[0],
				});
			}else if(this.type == "simple") {
				this.chartData.datasets.push({
					...common,
					label: "",
					data: Object.keys(api_data).map(item => ({x: item, y: api_data[item]})),
					borderColor: this.colors[0],
					backgroundColor: this.colors[0],
				});				
			} else if(this.type) {
				this.$store.state[this.type].forEach((item) => {
					if(api_data[item.id]?.data.length > 0) {
						this.chartData.datasets.push({
							...common,
							label: item[this.label_property],
							data: api_data[item.id].data.map(x => ({x: x.x, y: x.y / 100})), //TODO: not using minor_in_mayor 
							borderColor: this.colors[j],
							backgroundColor: this.colors[j],
						});
						j++;
					}
				});
			} else {
				Object.keys(api_data).forEach((item) => {
					if(api_data[item]?.data.length > 0) {
						this.chartData.datasets.push({
							...common,
							label: item === "0" ? "Earning" : item === "1" ? "Spending" : "Net",
							data: api_data[item].data.map(x => ({x: x.x, y: x.y / 100})), //TODO: not using minor_in_mayor 
							borderColor: this.colors[j],
							backgroundColor: this.colors[j],
						});
						j++;
					}
				});
			}
			
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

div#controls
	display: flex
</style>
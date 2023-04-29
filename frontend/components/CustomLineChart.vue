<template>
	<div id="container">
		<div id="title">
			<p>{{title}}</p>
		</div>
		<div v-if="loaded" id="chart">
			<LineChart
				v-if="Object.keys(chartData).length > 0"
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

<script lang="ts">
export default {
	data: () => ({
		loaded: false,
		from_date: null as string | null,
		to_date: null as string | null,
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
			legend: {
				position: "bottom",
				labels: {
					fontColor: "#fff"
				}
			}
		},
	}),

	props: {
		type: String,
		api_path: String,
		api_data: Object,
		label_property: String,
		aggregated: Boolean,
		title: String,
		no_controls: Boolean,
		currency_id: Number
	},

	async mounted() {
		if(this.$colorMode.preference == "light") {
			this.chartOptions.scales.x.ticks.fontColor = "#111";
			this.chartOptions.scales.x.gridLines.color = "#0002";
			this.chartOptions.scales.y.ticks.fontColor = "111";
			this.chartOptions.scales.y.gridLines.color = "0002";
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
		updateDate(dates: {from_date: string, to_date: string}) {
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
			const api_data: any = this.api_path ? await $fetch(this.api_path + query) : this.api_data;

			if (!api_data) {
				return;
			}

			let j = 0;

			const common = {
				cubicInterpolationMode: "monotone",
				fill: false,
				borderWidth: 4,
				pointRadius: 0.5,
				pointBorderWidth: 4
			}

			if(this.type == "simple_monetary") {
				const minor_in_mayor: number = (await $fetch(`/api/v1/currencies/${this.currency_id}`) as Currency).minor_in_mayor;
				(this as any).chartData.datasets.push({
					...common,
					label: "",
					data: Object.keys(api_data).map(item => ({x: item, y: ((api_data[item] / minor_in_mayor) * 100 + Number.EPSILON) / 100})),
					borderColor: this.colors[0],
					backgroundColor: this.colors[0],
				});
			}else if(this.type == "simple") {
				(this as any).chartData.datasets.push({
					...common,
					label: "",
					data: Object.keys(api_data).map(item => ({x: item, y: api_data[item]})),
					borderColor: this.colors[0],
					backgroundColor: this.colors[0],
				});				
			} else if(this.type) {
				(await $fetch(`/api/v1/${this.type}/all`) as any).forEach((item: any) => {
					let minor_in_mayor = 100;
					if(this.type == "currencies") {
						minor_in_mayor = item.minor_in_mayor;
					}

					if(api_data[item.id]?.data.length > 0) {
						(this as any).chartData.datasets.push({
							...common,
							label: item[(this as any).label_property],
							data: api_data[item.id].data.map((x: any) => ({x: x.x, y: x.y / minor_in_mayor})),
							borderColor: this.colors[j],
							backgroundColor: this.colors[j],
						});
						j++;
					}
				});
			} else {
				Object.keys(api_data).forEach((item) => {
					if(api_data[item]?.data.length > 0) {
						(this as any).chartData.datasets.push({
							...common,
							label: item === "0" ? "Earning" : item === "1" ? "Spending" : "Net",
							data: api_data[item].data.map((x: any) => ({x: x.x, y: x.y / 100})), //TODO: not using minor_in_mayor 
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
<template>
	<div id="container">
		<div id="title">
			<p>Balance per account over time</p>
		</div>
		<div v-if="loaded" id="chart">
			<LineChart
				:chartData="chartData"
				:chartOptions="chartOptions"
			/>
		</div>
		<div id="controls">
			<label for="from">From:</label>
			<input type="date" id="from" v-model="fromDate" @change="update">

			<label for="to">To:</label>
			<input type="date" id="to" v-model="toDate" @change="update">
		</div>
	</div>
</template>

<script>
export default {
	data: () => ({
		loaded: false,
		fromDate: null,
		toDate: null,
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
						color: "#fff5",
						drawBoder: false
					}
				}],
				yAxes: [{
					ticks: {
						fontColor: "#ddd"
					},
					gridLines: {
						color: "#fff5",
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

	async fetch() {
		await this.update();
	},

	methods: {
		async update() {
			this.loaded = false;

			this.chartData = {
				datasets: []
			}

			let query = "";
			if(this.fromDate && this.toDate) {
				query = `?from_date=${this.fromDate}&to_date=${this.toDate}&`;
			}
			const api_data = await this.$axios.$get("/api/v1/reports/balance_over_time_per_account" + query);

			this.$store.state.accounts.forEach(account => {
				if(api_data[account.id].length > 0) {
					this.chartData.datasets.push({
						label: account.name,
						data: api_data[account.id]?.map(x => ({x: x.x, y: x.y / 100})),
						cubicInterpolationMode: "monotone",
						fill: false,
						borderColor: `#${this.colors[account.id] ? this.colors[account.id] : this.generateRandomColor(account.id)}ff`
					});
				}
			});

			this.loaded = true;
		},

		generateRandomColor(key) {
			const chars = "0123456789abcdef";
			let output = "";
			for(let i = 0; i < 6; i++) {
				output += chars.charAt(Math.floor(Math.random() * chars.length));
			}
			this.colors[key] = output;
			return output;
		}
	}
}
</script>

<style lang="sass" scoped>
div#container
	height: 100%
	display: grid
	grid-template-columns: 100%
	grid-template-rows: 30px 350px 30px

div#title > p
	text-align: center
</style>
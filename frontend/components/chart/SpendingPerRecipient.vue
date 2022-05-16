<template>
	<div id="container">
		<div id="title">
			<p>Spending per recipient</p>
		</div>
		<div v-if="loaded"  id="chart">
			<PieChart
				:chartData="chartData"
				:chartOptions="chartOptions"
			/>
		</div>
		<div id="controls">
			<label for="range">Range: </label>
			<select id="range" v-model="date_range">
				<option value="0">Last 28 days</option>
				<option value="1">Last month</option>
				<option value="2">Current month</option>
				<option value="3">Last 90 days</option>
				<option value="4">Last quarter</option>
				<option value="5">Current year</option>
				<option value="6">Last 365 days</option>
				<option value="7">Total</option>
			</select>
		</div>
	</div>
</template>

<script>
export default {
	data: () => ({
		loaded: false,
		date_range: "0",
		fromDate: new Date(Date.now() - 1000 * 60 * 60 * 24 * 28).toISOString().slice(0, 10),
		toDate: new Date().toISOString().slice(0, 10),
		colors: {},
		chartData: {},
		chartOptions: {
			responsive: true,
			maintainAspectRatio: false,
			title:  {
				display: false,
			},
			legend: {
				position: "bottom",
				labels: {
					fontColor: "#fff"
				}
			},
			tooltips: {
    		callbacks: {
					label: function(tooltipItems, data) {  
						return data.labels[tooltipItems.index];
					}
    		}
			}
		}
	}),

	async mounted() {
		await this.update();
	},

	methods: {
		async update() {
			this.loaded = false;

			switch(this.date_range) {
				case "0": {
					this.fromDate = new Date(Date.now() - 1000 * 60 * 60 * 24 * 28).toISOString().slice(0, 10);
					this.toDate = new Date().toISOString().slice(0, 10);
					break;
				}
				case "1": {
					const date_parts = new Date(new Date().setMonth(new Date().getMonth() - 1)).toISOString().slice(0, 10).split("-");
					this.fromDate = new Date(date_parts[1] === "1" ? `${Number(date_parts[0]) - 1}-12-01` : `${date_parts[0]}-${date_parts[1]}-1`).toISOString().slice(0, 10);
					this.toDate = new Date(new Date(new Date(this.fromDate).setMonth(new Date(this.fromDate).getMonth() + 1)).setDate(0)).toISOString().slice(0, 10);
					break;
				}
				case "2": {
					this.fromDate = new Date(new Date().setDate(1)).toISOString().slice(0, 10);
					this.toDate = new Date(new Date(new Date(this.fromDate).setMonth(new Date (this.fromDate).getMonth() + 1)).setDate(0)).toISOString().slice(0, 10);
					break;
				}
				case "3": {
					this.fromDate = new Date(Date.now() - 1000 * 60 * 60 * 24 * 90).toISOString().slice(0, 10);
					this.toDate = new Date().toISOString().slice(0, 10);
					break;
				}
				case "4": {
					let quarter;
					let year = new Date().getFullYear();

					if(new Date().getMonth() <= 2) {
						quarter = 1;
					} else if(new Date().getMonth() <= 5) {
						quarter = 2;
					} else if(new Date().getMonth() <= 8) {
						quarter = 3;
					} else {
						quarter = 4;
					}

					if(quarter == 1) {
						quarter = 4;
						year--;
					} else {
						quarter--;
					}

					switch(quarter) {
						case 1: {
							this.fromDate = year + "-01-01";
							this.toDate = year + "-03-31";
							break;
						}
						case 2: {
							this.fromDate = year + "-04-01";
							this.toDate = year + "-06-30";
							break;
						}
						case 3: {
							this.fromDate = year + "-07-01";
							this.toDate = year + "-09-30";
							break;
						}
						case 4: {
							this.fromDate = year + "-10-01";
							this.toDate = year + "-12-31";
							break;
						}
					}
					break;
				}
				case "5": {
					this.fromDate = new Date().getFullYear() + "-01-01"
					this.toDate = new Date().getFullYear() + "-12-31"
				}
				case "6": {
					this.fromDate = new Date(Date.now() - 1000 * 60 * 60 * 24 * 365).toISOString().slice(0, 10);
					this.toDate = new Date().toISOString().slice(0, 10);
				}
				case "7": {
					this.fromDate = "0000-01-01";
					this.toDate = "9999-12-31";
				}
			}

			this.chartData = {
				datasets: [{
					data: []
				}],
				labels: []
			}

			let query = "";
			if(this.fromDate && this.toDate) {
				query = `?from_date=${this.fromDate}&to_date=${this.toDate}&`;
			}
			const api_data = await this.$axios.$get("/api/v1/reports/spending_per_recipient_in_date_range" + query);
			
			for(const recipient_id in api_data) {
				let total_value = 0;
				let label = `${this.$store.state.recipients.filter(r => r.id === Number(recipient_id))[0].name}: `;
				for(const currency_id in api_data[recipient_id]) {
					const currency = this.$store.state.currencies.filter(c => c.id === Number(currency_id))[0];
					const value = api_data[recipient_id][currency_id] / currency.minor_in_mayor;
					total_value += value;
					label += `${value}${currency.symbol}, `;
				}
				label = label.slice(0, -2);
				this.chartData.datasets[0].data.push(total_value);
				this.chartData.labels.push(label);
			}
			this.chartData.datasets[0].backgroundColor = this.chartData.labels.map((_, i, a) => `rgba(167, 176, 194, ${(i + 1) / a.length})`);
			this.chartData.datasets[0].borderWidth = 2;
			this.chartData.datasets[0].borderColor = "rgba(167, 176, 194, 1)";

			this.loaded = true;
		}
	},

	watch: {
		date_range() {
			this.update();
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
<template>
	<div id="container">
		<div id="title">
			<p>Spending per {{type}}</p>
		</div>
		<div v-if="loaded && !no_data" id="chart">
			<PieChart
				:chartData="chartData"
				:chartOptions="chartOptions"
			/>
		</div>
		<div v-if="no_data">
			<p>No data</p>
		</div>
		<div id="controls">
			<DateControl 
				v-on:update="updateDate"
			/>
			<div v-if="showOnlyParentsToggle">
				<label for="parent">Only Parents:</label>
				<input type="checkbox" id="parent" v-model="only_parents">
			</div>
		</div>
	</div>
</template>

<script lang="ts">
export default {
	data: () => ({
		loaded: false,
		no_data: false,
		from_date: new Date(Date.now() - 1000 * 60 * 60 * 24 * 28).toISOString().slice(0, 10),
		to_date: new Date().toISOString().slice(0, 10),
		colors: ["#EC8A83", "#FFAD85", "#F9F176", "#8BE59D", "#6AB4F1", "#A983D8"],
		chartData: {},
		only_parents: false,
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
					label: function(tooltipItems: any, data: any) {  
						return data.labels[tooltipItems.index];
					}
    		}
			}
		},
		accounts: [],
		currencies: [],
		recipients: [],
		assets: [],
		transactions: []
	}),

	props: {
		type: String,
		api_path: String,
		label_property: String,
		showOnlyParentsToggle: Boolean
	},

	async mounted() {
		this.accounts = await $fetch("/api/v1/accounts/all");
		this.currencies = await $fetch("/api/v1/currencies/all");
		this.recipients = await $fetch("/api/v1/recipients/all");
		this.assets = await $fetch("/api/v1/assets/all");
		this.transactions = await $fetch("/api/v1/transactions/all");

		if(this.$colorMode.preference == "light") {
			this.chartOptions.legend.labels.fontColor = "#000";
		}
		await this.update();
	},

	methods: {
		updateDate(dates: any) {
			this.from_date = dates.from_date;
			this.to_date = dates.to_date;
			this.update();
		},

		async update() {
			this.loaded = false;

			this.chartData = {
				datasets: [{
					data: []
				}],
				labels: []
			}

			let query = "";
			if(this.from_date && this.to_date) {
				query = `?from_date=${this.from_date}&to_date=${this.to_date}`;
			}
			if(this.showOnlyParentsToggle) {
				query += `&only_parents=${this.only_parents}`;
			}
			const api_data: any = await $fetch(this.api_path + query);
			
			this.no_data = Object.keys(api_data).length === 0;
			for(const id in api_data) {
				let total_value = 0;
				let label = (id as any) == 4294967295 ? "other: " : `${(this as any)[(this as any).type].filter((x: any) => x.id === Number(id))[0][(this as any).label_property]}: `;
				for(const currency_id in api_data[id].data) {
					const currency: any = this.currencies.filter((c: any) => c.id === Number(currency_id))[0];
					const value = api_data[id].data[currency_id] / currency.minor_in_mayor;
					total_value += value;
					label += `${value}${currency.symbol}, `;
				}
				label = label.slice(0, -2);
				(this as any).chartData.datasets[0].data[api_data[id].rank] = total_value;
				(this as any).chartData.labels[api_data[id].rank] = label;
			}
			(this as any).chartData.datasets[0].backgroundColor = (this as any).chartData.labels.map((_: any, i: any) => this.colors[i]);

			this.loaded = true;
		}
	},

	watch: {
		only_parents() {
			this.update();
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
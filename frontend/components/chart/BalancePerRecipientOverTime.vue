<template>
	<div id="container">
		<div id="title">
			<p>Balance per recipient over time</p>
		</div>
		<div id="chart">
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

	fetch() {
		this.update();
	},

	methods: {
		update() {
			this.chartData = {
				datasets: []
			}

			const transactions = this.$store.state.transactions.map(x => ({...x}));
			const dateSortedTransactions = transactions.sort((a, b) => Date.parse(a.timestamp) - Date.parse(b.timestamp));

			this.$store.state.recipients.forEach(recipient => {
				const filteredTransactions = dateSortedTransactions.filter(x => x.recipient_id == recipient.id);

				let data = [];
				let labels = [];
	
				filteredTransactions.forEach(x => {
					data.push(x.amount);
					labels.push(x.timestamp);
				});
	
				for(let i = 0; i < labels.length; i++) {
					if(labels[i] && labels[i + 1]) {
						if(labels[i].slice(0, 10) === labels[i + 1].slice(0, 10)) {
							data[i] += data[i + 1];
							data[i + 1] = null;
							labels[i + 1] = null;
						}
					}
				}
				data = data.filter(x => x);
				labels = labels.filter(x => x);
	
				for(let i = 0; i < data.length; i++) {
					if(i > 0) data[i] += data[i - 1];
				}
	
				if(this.fromDate && this.toDate) {
					let newData = [];
					let newLabels = [];
					for(let i = 0; i < labels.length; i++) {
						if(Date.parse(labels[i]) >= Date.parse(this.fromDate) && Date.parse(labels[i]) <= Date.parse(this.toDate)) {
							newData.push(data[i]);
							newLabels.push(labels[i]);
						}
					}
					data = newData;
					labels = newLabels;
				}
	
				data = data.map(x => x / 100);
				labels = labels.map(x => x.slice(0, 10));
	
				for(let i = 0; i < labels.length; i++) {
					data[i] = {
						y: data[i],
						x: labels[i]
					}
				}
				
				if(data.length > 0) {
					this.chartData.datasets.push({
						label: recipient.name,
						data: data,
						cubicInterpolationMode: "monotone",
						fill: false,
						borderColor: `#${this.colors[recipient.id] ? this.colors[recipient.id] : this.generateRandomColor(recipient.id)}ff`

					});
				}
			});
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
<template>
	<div>
		<LineChart
			:chartData="chartData"
			:chartOptions="chartOptions"
		/>
	</div>
</template>

<script>
export default {
	data: () => ({
		chartData: {},
		chartOptions: {
			responsive: true,
			maintainAspectRatio: false,
			pointBackgroundColor: "rgba(255, 255, 255, 0)",
			title:  {
				display: true,
				text: "Total Balance over time"
			},
			scales: {
				xAxes: [{
					id: "x",
					type: "time",
					time: {
						unit: "day"
					}
				}]
			}
		}
	}),

	fetch() {
		const transactions = this.$store.state.transactions.map(x => ({...x}));
		const dateSortedTransactions = transactions.sort((a, b) => new Date(a.timestamp).valueOf() - new Date(b.timestamp).valueOf());

		let data = [];
		let labels = [];

		dateSortedTransactions.forEach(x => {
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

		data = data.map(x => x / 100);
		
		this.chartData = {
			labels: labels,
			datasets: [
				{
					label: "Amount",
					data: data,
					cubicInterpolationMode: "monotone",
					pointBackgroundColor: "#fff"
				}
			]
		}
	}
}
</script>

<style lang="sass" scoped>
div
	height: 100%
</style>
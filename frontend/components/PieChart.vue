<template>
	<div>
		<Pie
			ref="chart"
			:data="chartData"
			:options="chartOptions"
		/>
	</div>
</template>

<script>
import { Pie } from 'vue-chartjs'
import { Chart as ChartJS, Title, Tooltip, Legend, ArcElement, Colors } from 'chart.js'
ChartJS.register(Title, Tooltip, Legend, ArcElement, Colors)

export default {
	props: {
		chartData: Object,
		chartOptions: Object
	},

	components: {
		Pie
	},
}
</script>

<style lang="sass" scoped>
div
	height: 100%
</style>
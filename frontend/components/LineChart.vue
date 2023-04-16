<template>
	<Line
		:data="chartData"
		:options="chartOptions"
	/>
</template>

<script>
import 'chartjs-adapter-date-fns';
import { Line } from 'vue-chartjs'
import { Chart as ChartJS, Title, Tooltip, Legend, BarElement, CategoryScale, LinearScale, TimeScale, PointElement, LineElement } from 'chart.js'
ChartJS.register(Title, Tooltip, Legend, BarElement, CategoryScale, LinearScale, TimeScale, PointElement, LineElement)

export default {
	props: {
		chartData: Object,
		chartOptions: Object
	},

	components: {
		Line
	}
}
</script>

<style lang="sass" scoped>
div, canvas
	width: 100%
	height: 100%
	position: relative
</style>
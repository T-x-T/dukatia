<template>
	<Pie
		:data="chartData"
		:options="chartOptions"
	/>
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
	}
}
</script>

<style lang="sass" scoped>
div, canvas
	width: 100%
	height: 100%
	position: relative
</style>
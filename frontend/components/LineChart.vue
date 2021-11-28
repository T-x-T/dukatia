<script>
import { Line, mixins } from 'vue-chartjs'

export default {
  extends: Line,
	mixins: [mixins.reactiveProp],
	props: {
		chartData: Object,
		chartOptions: Object
	},

  mounted() {
    this.renderChart(this.chartData, this.chartOptions)
  }
}
</script>

<style lang="sass" scoped>
div, canvas
	width: 100%
	height: 100%
	position: relative
</style>
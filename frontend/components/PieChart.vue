<script>
import { Pie, mixins } from 'vue-chartjs'

export default {
  extends: Pie,
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
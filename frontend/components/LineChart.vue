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
<template>
	<div>
		<div id="grid">
			<div class="gridItem form">
				<DetailsPage
					v-if="Object.keys(config).length > 0"
					:config="config"
					v-on:back="$emit('back')"
				/>
			</div>

			<div v-if="budget?.id !== undefined && render_charts" class="gridItem pie_chart">
				<ChartPie
					:pie="chart_utilization"
				/>
			</div>
		</div>
	</div>	
</template>

<script lang="ts">
export default {
	data: () => ({
		config: {} as DetailFormConfig,
		render_charts: false,
		chart_utilization: {} as ChartOptions,
	}),

	props: {
		budget: {
			type: Object as PropType<Budget>,
			required: true,
		}
	},

	async created() {
		await this.update();
	},

	methods: {
		async update() {
			this.budget.filter_tag_ids = Array.isArray(this.budget.filter_tag_ids) ? [...this.budget.filter_tag_ids] : []

			this.config = {
				...this.$detailPageConfig().budget,
				data: {
					...this.budget,
					active_from: new Date(new Date(this.budget.active_from).valueOf() - (new Date(this.budget.active_from).getTimezoneOffset() * 60000)).toISOString().slice(0, -8),
					active_to: this.budget.active_to ? new Date(new Date(this.budget.active_to).valueOf() - (new Date(this.budget.active_to).getTimezoneOffset() * 60000)).toISOString().slice(0, -8) : null,
				},
			}

			if(this.budget?.id !== undefined) {
				this.chart_utilization = (await $fetch(`/api/v1/charts/pie/single_budget_current_period/data?budget_id=${this.budget.id}`)).pie;
				this.render_charts = true;
			}
		}
	}
}
</script>

<style lang="sass" scoped>

div#grid
	display: flex
	width: 100%
	justify-content: flex-start
	align-items: flex-start
	align-content: flex-start
	gap: 10px
	flex-wrap: wrap

div.form
	display: flex
	align-items: center
	justify-content: center
	height: max-content

div.gridItem
	padding: 10px

div.pie_chart
	width: 20em
	height: 20em
</style>
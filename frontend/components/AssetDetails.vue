<template>
	<div id="grid">
		<DetailsPage
			class="gridItem medium"
			id="detailPage"
			:config="config"
			v-on:back="$emit('back')"
		/>
		<div class=" gridItem large">
			<CustomLineChart
				:api_path="`/api/v1/reports/value_per_unit_over_time_for_asset/${asset.id}`"
				title="Value over time per single unit"
				type="simple_monetary"
				:no_controls="true"
			/>
		</div>
		<div class=" gridItem large">
			<CustomLineChart
				:api_path="`/api/v1/reports/amount_over_time_for_asset/${asset.id}`"
				title="Amount over time"
				type="simple"
				:no_controls="true"
			/>
		</div>
		<div class="gridItem medium">

		</div>
		<div class="gridItem small">

		</div>
	</div>
</template>

<script>
export default {
	data: () => ({
		config: {}
	}),

	props: {
		asset: Object
	},

	created() {
		this.config = {
			...this.$detailPageConfig.asset,
			data: {
				...this.asset,
				value_per_unit: this.asset.value_per_unit / 100, //TODO: use minor_in_mayor
			},
		};
	}
}
</script>

<style lang="sass" scoped>
div#grid
	display: grid
	width: 100%
	grid-template-columns: repeat(auto-fit, minmax(10%, 1fr))
	grid-auto-rows: 100px
	align-items: stretch
	justify-items: stretch
	grid-gap: 10px

div.gridItem
	padding: 10px

div.small
	grid-column: span 1
	grid-row: span 1

div.medium
	grid-column: span 2
	grid-row: span 4

div.large
	grid-column: span 4
	grid-row: span 4
</style>
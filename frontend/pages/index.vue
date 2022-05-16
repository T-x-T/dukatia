<template>
	<div>
		<h2>This is the Dashboard</h2>
		<div id="grid">
			<div class="gridItem small" v-for="(amount, currency_id, i) in total_per_currency" :key="i">
				<TotalBalance :currency_id="parseInt(currency_id)" :amount="amount"/>	
			</div>
			<div class="gridItem medium">
				<ChartSpendingPerRecipient />
			</div>
			<div class="gridItem large">
				<ChartTotalBalanceOverTime />
			</div>
			<div class="gridItem large">
				<ChartBalancePerRecipientOverTime />
			</div>
			<div class="gridItem large">
				<ChartBalancePerAccountOverTime />
			</div>
		</div>
	</div>
</template>

<script>
export default {
	data: () => ({
		total_per_currency: {}
	}),

	async fetch() {
		this.total_per_currency = await this.$axios.$get("/api/v1/reports/total_per_currency");
	}
}
</script>

<style lang="sass" scoped>
@import "assets/_vars.sass"

div#grid
	display: grid
	width: 100%
	height: 100vh
	grid-template-columns: repeat(auto-fit, minmax(10%, 1fr))
	grid-auto-rows: 100px
	align-items: stretch
	justify-items: stretch
	grid-gap: 10px

div.gridItem
	background: $darkest
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
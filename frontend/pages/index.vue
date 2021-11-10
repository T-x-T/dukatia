<template>
	<div>
		<h2>This is the Dashboard</h2>
		<div id="grid">
			<div class="gridItem">
				<p>You have {{totalMoney}} monies</p>		
			</div>
			<div class="gridItem">
				<p>You have {{totalMoney}} monies</p>		
			</div>
			<div class="gridItem">
				<p>You have {{totalMoney}} monies</p>		
			</div>
			<div class="gridItem">
				<p>You have {{totalMoney}} monies</p>		
			</div>
			<div class="gridItem">
				<p>You have {{totalMoney}} monies</p>		
			</div>
			<div class="gridItem large">
				<ChartBalancePerRecipientOverTime />
			</div>
			<div class="gridItem large">
				<ChartTotalBalanceOverTime />
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
		totalMoney: null
	}),

	async fetch() {
		this.$store.state.transactions.forEach(x => this.totalMoney += x.amount);
		this.totalMoney = (this.totalMoney / 100) + "€"
	}
}
</script>

<style lang="sass" scoped>
@import "assets/_vars.sass"

div#grid
	display: grid
	width: 100%
	height: 100vh
	grid-template-columns: repeat(auto-fit, minmax(200px, 1fr))
	grid-auto-rows: 100px
	align-items: stretch
	justify-items: stretch
	grid-gap: 10px

div.gridItem
	background: $darkest
	padding: 10px

div.large
	grid-column: span 4
	grid-row: span 4
	
</style>
<template>
	<div>
		<h2>This is the Dashboard</h2>
		<p>You have {{totalMoney}} monies</p>
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
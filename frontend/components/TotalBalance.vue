<template>
	<div>
		<p>{{amount}}{{symbol}}</p>
	</div>
</template>

<script>
export default {
	data: () => ({
		amount: 0,
		symbol: ""
	}),
	props: {
		currencyId: Number
	},

	mounted() {
		const currency = this.$store.state.currencies.filter(x => x.id == this.currencyId)[0];
		this.symbol = currency.symbol;
		this.$store.state.transactions.filter(x => x.currencyId === this.currencyId).forEach(x => this.amount += x.amount);
		this.amount = this.amount / currency.minorinmayor;
	}
}
</script>
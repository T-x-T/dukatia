<template>
	<div>
		<p>Current Balance: <b>{{amount}}{{symbol}}</b></p>
	</div>
</template>

<script>
export default {
	data: () => ({
		amount: 0,
		symbol: ""
	}),
	props: {
		currency_id: Number
	},

	mounted() {
		const currency = this.$store.state.currencies.filter(x => x.id == this.currency_id)[0];
		this.symbol = currency.symbol;
		this.$store.state.transactions.filter(x => x.currency_id === this.currency_id).forEach(x => this.amount += x.amount);
		this.amount = this.amount / currency.minor_in_mayor;
	}
}
</script>
<template>
	<div>
		<p>Current Balance: <b>{{amount_for_display}}{{symbol}}</b></p>
	</div>
</template>

<script>
export default {
	data: () => ({
		amount_for_display: 0,
		symbol: ""
	}),
	props: {
		currency_id: Number,
		amount: Number
	},

	mounted() {
		const currency = this.$store.state.currencies.filter(x => x.id == this.currency_id)[0];
		this.symbol = currency.symbol;
		this.amount_for_display = this.amount / currency.minor_in_mayor;
	}
}
</script>
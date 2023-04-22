<template>
	<div>
		<p>Current Balance: <b>{{amount_for_display}}{{symbol}}</b></p>
	</div>
</template>

<script lang="ts">
export default {
	data: () => ({
		amount_for_display: 0,
		symbol: "",
		currencies: [] as Currency[],
	}),

	props: {
		currency_id: {
			type: Number,
			required: true,
		},
		amount: {
			type: Number,
			required: true,
		}
	},

	async created() {
		this.currencies = await $fetch("/api/v1/currencies/all");
		const currency = this.currencies.filter(x => x.id == this.currency_id)[0];
		this.symbol = currency.symbol;
		this.amount_for_display = this.amount / currency.minor_in_mayor;
	}
}
</script>
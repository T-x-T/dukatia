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
		currencies: [],
	}),

	props: {
		currency_id: Number,
		amount: Number
	},

	async created() {
		this.currencies = await $fetch("/api/v1/currencies/all");
		const currency: any = this.currencies.filter((x: any) => x.id == this.currency_id)[0];
		this.symbol = currency.symbol;
		this.amount_for_display = (this as any).amount / currency.minor_in_mayor;
	}
}
</script>
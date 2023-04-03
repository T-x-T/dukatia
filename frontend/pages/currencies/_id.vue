<template>
	<div>
		<CurrencyDetails
			:currency="currencyData"
			v-on:back="$router.push('/currencies')"
		/>
	</div>
</template>

<script>
export default {
	data: () => ({
		currencyData: {}
	}),

	async fetch() {
		if(this.$route.path.split("/")[2] == "new") {
			this.currencyData = this.$detailPageConfig.currency.defaultData;
		} else {
			const id = Number(this.$route.path.split("/")[2]);
			const currencyFromStore = this.$store.state.currencies.filter(x => x.id == id)[0];
			this.currencyData = {...currencyFromStore};
		}
	}
}
</script>
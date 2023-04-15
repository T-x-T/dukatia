<template>
	<div>
		<CurrencyDetails
			:currency="currencyData"
			v-on:back="useRouter().push('/currencies')"
		/>
	</div>
</template>

<script lang="ts">
export default {
	data: () => ({
		currencyData: {}
	}),

	async fetch() {
		if(this.$route.path.split("/")[2] == "new") {
			this.currencyData = this.$detailPageConfig().currency.defaultData;
		} else {
			const currencies: any = await $fetch("/api/v1/currencies/all");
			const id = Number(this.$route.path.split("/")[2]);
			const currencyFromStore = currencies.filter((x: any) => x.id == id)[0];
			this.currencyData = {...currencyFromStore};
		}
	}
}
</script>
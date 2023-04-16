<template>
	<div>
		<CurrencyDetails
			v-if="Object.keys(currencyData).length > 0"
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

	async created() {
		if(useRoute().path.split("/")[2] == "new") {
			this.currencyData = this.$detailPageConfig().currency.defaultData;
		} else {
			const currencies: any = await $fetch("/api/v1/currencies/all");
			const id = Number(useRoute().path.split("/")[2]);
			const currencyFromStore = currencies.filter((x: any) => x.id == id)[0];
			console.log(currencyFromStore)
			this.currencyData = {...currencyFromStore};
		}
	}
}
</script>
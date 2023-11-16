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
		currencyData: {} as Currency
	}),

	async created() {
		if(useRoute().path.split("/")[2] == "new") {
			this.currencyData = {
				name: "",
				minor_in_major: 100,
				symbol: ""
			};
		} else {
			const id = Number(useRoute().path.split("/")[2]);
			this.currencyData = await $fetch(`/api/v1/currencies/${id}`);
		}
	}
}
</script>
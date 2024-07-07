<template>
	<Head>
		<Title>Dukatia - Currencies - {{ currencyData.name }}</Title>
	</Head>

	<div>
		<CurrencyDetails
			v-if="loaded"
			:currency="currencyData"
			@back="useRouter().push('/currencies')"
		/>
	</div>
</template>

<script lang="ts">
export default {
	data: () => ({
		currencyData: {} as Currency,
		loaded: false,
	}),

	async created() {
		if(useRoute().path.split("/")[2] == "new") {
			this.loaded = true;
		} else {
			const id = useRoute().path.split("/")[2];
			this.currencyData = await $fetch(`/api/v1/currencies/${id}`);
			this.loaded = true;
		}
	}
}
</script>
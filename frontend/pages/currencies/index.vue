<template>
	<div>
		<button class="green" @click="newCurrency">Add</button>
		<CustomTable
			v-if="tableData"
			:tableData="tableData"
			v-on:rowClick="rowClick"
		/>
	</div>
</template>

<script>
export default {
	data: () => ({
		tableData: null
	}),

	async fetch() {
		await this.updateCurrencies();
	},

	methods: {
		async updateCurrencies() {
			await this.$store.dispatch("fetchCurrencies");
			this.tableData = {
				multiSelect: false,
				defaultSort: {
					column: 0,
					sort: "asc"
				},
				columns: [
					{name: "ID", type: "number"},
					{name: "Name", type: "string"},
					{name: "Symbol", type: "string"},
					{name: "Minor in Mayor", type: "number"},
				],
				rows: this.$store.state.currencies.map(x => ([
					x.id,
					x.name,
					x.symbol,
					x.minor_in_mayor
				]))
			}
		},
		
		async rowClick(row) {
			await useRouter().push(`/currencies/${row[0]}`);
		},

		async newCurrency() {
			await useRouter().push("/currencies/new");
		}
	}
}
</script>
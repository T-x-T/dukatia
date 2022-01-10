<template>
	<div id="main">
		<CustomTable
			:tableData="tableData"
		/>
	</div>
</template>

<script>
export default {
	data: () => ({
		tableData: {}
	}),

	async fetch() {
		await this.updateCurrencies();
	},

	methods: {
		async updateCurrencies() {
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
					x.minorinmayor
				]))
			}
		}
	}
}
</script>
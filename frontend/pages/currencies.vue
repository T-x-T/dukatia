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
				headers: [
					"ID", "Name", "Symbol", "Minor in Mayor"
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
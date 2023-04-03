<template>
	<div id="main">
		<button class="green" @click="newAccount">Add</button>
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
		await this.updateAccounts();
	},

	methods: {
		async updateAccounts() {
			await this.$store.dispatch("fetchAccounts");
			this.tableData = {
				multiSelect: false,
				defaultSort: {
					column: 0,
					sort: "asc"
				},
				columns: [
					{name: "ID", type: "number"},
					{name: "Name", type: "string"},
					{name: "Currency", type: "choice", options: this.$store.state.currencies.map(x => x.name)},
					{name: "Tags", type: "choice", options: [...new Set(this.$store.state.tags.map(x => x.name))]},
					{name: "Balance", type: "number"}
				],
				rows: this.$store.state.accounts.map(x => ([
					x.id,
					x.name,
					this.$store.state.currencies.filter(c => c.id == x.default_currency_id)[0].name,
					this.$store.state.tags.filter(y => x.tag_ids?.includes(y.id)).map(y => y.name).join(", "),
					this.$store.state.transactions.filter(t => t.account_id == x.id).reduce((a, b) => a + b.amount, 0) / this.$store.state.currencies.filter(c => c.id == x.default_currency_id)[0].minor_in_mayor + this.$store.state.currencies.filter(c => c.id == x.default_currency_id)[0].symbol
				]))
			}
		},
		
		rowClick(row) {
			this.$router.push(`/accounts/${row[0]}`);
		},

		async newAccount() {
			this.$router.push("/accounts/new");
		}
	}
}
</script>

<style lang="sass" scoped>
div#main
	height: 100vh
	overflow: scroll
</style>
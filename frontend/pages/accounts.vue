<template>
	<div id="main">
		<div id="table" v-if="mode=='table'">
			<button class="green" @click="newAccount">Add</button>

			<CustomTable
				:tableData="tableData"
				v-on:rowClick="rowClick"
			/>
		</div>

		<div id="details" v-if="mode=='details'">
			<AccountDetails
				:account="selectedRow"
				v-on:back="updateAndLoadTable"
			/>
		</div>
	</div>
</template>

<script>
export default {
	data: () => ({
		tableData: {},
		selectedRow: {},
		mode: "table"
	}),

	async fetch() {
		await this.updateAccounts();
	},

	methods: {
		async updateAccounts() {
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
					{name: "Tags", type: "choice", options: [...new Set(this.$store.state.tags.map(x => x.name))]}
				],
				rows: this.$store.state.accounts.map(x => ([
					x.id,
					x.name,
					this.$store.state.currencies.filter(c => c.id == x.default_currency_id)[0].name,
					this.$store.state.tags.filter(y => x.tag_ids?.includes(y.id)).map(y => y.name).join(", ")
				]))
			}
		},
		
		rowClick(row) {
			const rowFromStore = this.$store.state.accounts.filter(x => x.id == row[0])[0];
			const default_currency = this.$store.state.currencies.filter(x => x.id == rowFromStore.default_currency)[0]
			this.selectedRow = {...rowFromStore, default_currency: {...default_currency}};
			this.mode = "details";
		},

		async newAccount() {
			this.selectedRow = {
				id: "",
				name: "",
				default_currency: this.$store.state.currencies.filter(x => x.id == 0)[0]
			}

			this.mode = "details";
		},

		async updateAndLoadTable() {
			await this.$store.dispatch("fetchAccounts");
			await this.updateAccounts();
			this.mode = "table";
		}
	}
}
</script>
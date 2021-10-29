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
				headers: [
					"ID", "Name", "Default Currency"
				],
				rows: this.$store.state.accounts.map(x => ([
					x.id,
					x.name,
					this.$store.state.currencies.filter(c => c.id == x.defaultCurrency)[0].name
				]))
			}
		},
		
		rowClick(row) {
			const rowFromStore = this.$store.state.accounts.filter(x => x.id == row[0])[0];
			const defaultCurrency = this.$store.state.currencies.filter(x => x.id == rowFromStore.defaultCurrency)[0]
			this.selectedRow = {...rowFromStore, defaultCurrency: {...defaultCurrency}};
			this.mode = "details";
		},

		async newAccount() {
			this.selectedRow = {
				id: "",
				name: "",
				defaultCurrency: this.$store.state.currencies.filter(x => x.id == 0)[0]
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
<template>
	<div id="main">
		<div id="table" v-if="mode=='table'">
			<button class="green" @click="newTransaction">Add</button>
			<CustomTable
				:tableData="tableData"
				v-on:rowClick="rowClick"
			/>
		</div>

		<div v-if="mode=='details'" id="details">
			<TransactionDetails 
				:transaction="selectedRow"
				v-on:back="updateAndLoadTable"
			/>
		</div>
	</div>
</template>

<script>
export default {
	data: () => ({
		tableData: {},
		mode: "table",
		selectedRow: {}
	}),

	async fetch() {
		await this.updateTransactions();
	},

	methods: {
		async updateTransactions() {
			const transactionsForDisplay = this.$store.state.transactions.map(x => {
				x.account = this.$store.state.accounts.filter(a => a.id == x.accountId)[0];
				x.currency = this.$store.state.currencies.filter(c => c.id == x.currencyId)[0];
				x.recipient = this.$store.state.recipients.filter(r => r.id == x.recipientId)[0];
				return x;
			});

			this.tableData = {
				headers: [
					"ID", "User", "Account", "Recipient", "Status", "Timestamp", "Amount", "Comment"
				],
				rows: transactionsForDisplay.map(x => ([
					x.id,
					x.userId,
					x.account.name,
					x.recipient.name,
					x.status === 1 ? "Completed" : "Withheld",
					new Date(x.timestamp).toISOString().substring(0, 10),
					`${x.amount / x.currency.minorinmayor}${x.currency.symbol}`,
					x.comment
				]))
			}
		},

		rowClick(row) {
			const rowFromStore = this.$store.state.transactions.filter(x => x.id == row[0])[0]
			this.selectedRow = {...rowFromStore, amount: rowFromStore.amount / 100};
			this.mode = "details";
		},

		async newTransaction() {
			this.selectedRow = {
				id: "",
				accountId: 0,
				currencyId: 0,
				recipientId: 0,
				status: 1,
				timestamp: new Date().toISOString(),
				amount: 0,
				comment: "",
				currency: this.$store.state.currencies.filter(x => x.id == 0)[0]
			}

			this.mode = "details";
		},

		async updateAndLoadTable() {
			await this.$store.dispatch("fetchTransactions");
			await this.updateTransactions();
			this.mode = "table";
		}
	}
}
</script>

<style lang="sass" scoped>
@import "assets/_vars.sass"

</style>
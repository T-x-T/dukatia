<template>
	<div id="main">
		<div id="table" v-if="mode=='table'">
			<div>
				<button class="green" @click="newTransaction">Add</button>
			</div>
			<div v-if="selectedRows.length > 0" id="batchEditContainer">
				<div id="batchEdit">
					<label for="account">Account:</label>
					<select id="account" v-model="batchAccountId">
						<option v-for="(account, index) in $store.state.accounts" :key="index" :value="account.id">{{account.name}}</option>
					</select>

					<label for="recipient">Recipient:</label>
					<select id="recipient" v-model="batchRecipientId">
						<option v-for="(recipient, index) in $store.state.recipients" :key="index" :value="recipient.id">{{recipient.name}}</option>
					</select>
					<button class="green" @click="applyBatchEdit()">Edit selected rows</button>
				</div>		
			</div>
			<CustomTable
				:tableData="tableData"
				v-on:rowClick="rowClick"
				v-on:rowSelect="rowSelect"
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
		selectedRow: {},
		selectedRows: [],
		batchAccountId: null,
		batchRecipientId: null
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
				multiSelect: true,
				defaultSort: {
					column: 0,
					sort: "asc"
				},
				columns: [
					{name: "ID", type: "number"},
					{name: "Account", type: "choice", options: [...new Set(this.$store.state.accounts.map(x => x.name))]},
					{name: "Recipient", type: "choice", options: [...new Set(this.$store.state.recipients.map(x => x.name))]},
					{name: "Status", type: "choice", options: ["Completed"]},
					{name: "Timestamp", type: "date"},
					{name: "Amount", type: "number"},
					{name: "Comment", type: "string"}
				],
				rows: transactionsForDisplay.map(x => ([
					x.id,
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

		rowSelect(rows) {
			this.selectedRows = null;
			this.selectedRows = rows;
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

		async applyBatchEdit() {
			await Promise.all(this.selectedRows.map(async row => {
				let transaction = {...this.$store.state.transactions.filter(x => row && x.id === row[0])[0]};
				transaction.accountId = Number.isInteger(this.batchAccountId) ? this.batchAccountId : transaction.accountId;
				transaction.recipientId = Number.isInteger(this.batchRecipientId) ? this.batchRecipientId : transaction.recipientId;


				const transactionData = {
					accountId: transaction.accountId,
					currencyId: transaction.currencyId,
					recipientId: transaction.recipientId,
					status: transaction.status,
					timestamp: transaction.timestamp,
					amount: transaction.amount,
					comment: transaction.comment
				}
				await this.$axios.$put(`/api/v1/transactions/${transaction.id}`, transactionData);
			}));
			this.batchAccountId = null;
			this.batchRecipientId = null;
			setTimeout(() => this.updateAndLoadTable(), 100);
		},

		async updateAndLoadTable() {
			await this.$store.dispatch("fetchTransactions");
			await this.updateTransactions();
			this.mode = "";
			setImmediate(() => this.mode = "table");
		}
	}
}
</script>
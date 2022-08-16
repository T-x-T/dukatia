<template>
	<div id="main">
		<div id="table" v-if="tableOpen">
			<div>
				<button class="green" @click="newTransaction">Add</button>
			</div>
			<div v-if="selectedRows.length > 0" id="batchEditContainer">
				<div id="batchEdit">
					<label for="account">Account:</label>
					<select id="account" v-model="batchaccount_id">
						<option v-for="(account, index) in $store.state.accounts" :key="index" :value="account.id">{{account.name}}</option>
					</select>

					<label for="recipient">Recipient:</label>
					<select id="recipient" v-model="batchrecipient_id">
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

		<div v-if="detailsOpen" id="detailBar">
			<TransactionDetails 
				:transaction="selectedRow"
				v-on:back="updateAndLoadTable"
				v-on:updateData="updateTable"
			/>
		</div>
	</div>
</template>

<script>
export default {
	data: () => ({
		tableData: {},
		detailsOpen: false,
		tableOpen: true,
		selectedRow: {},
		selectedRows: [],
		batchaccount_id: null,
		batchrecipient_id: null
	}),

	async fetch() {
		await this.updateTransactions();
	},

	methods: {
		async updateTransactions() {
			const transactionsForDisplay = this.$store.state.transactions.map(x => {
				x.account = this.$store.state.accounts.filter(a => a.id == x.account_id)[0];
				x.currency = this.$store.state.currencies.filter(c => c.id == x.currency_id)[0];
				x.recipient = this.$store.state.recipients.filter(r => r.id == x.recipient_id)[0];
				return x;
			});

			this.tableData = {
				multiSelect: true,
				displaySum: true,
				sumColumn: 4,
				defaultSort: {
					column: 3,
					sort: "desc"
				},
				columns: [
					{name: "ID", type: "number"},
					{name: "Account", type: "choice", options: [...new Set(this.$store.state.accounts.map(x => x.name))]},
					{name: "Recipient", type: "choice", options: [...new Set(this.$store.state.recipients.map(x => x.name))]},
					{name: "Timestamp", type: "date"},
					{name: "Amount", type: "number"},
					{name: "Comment", type: "string"},
					{name: "Tags", type: "choice", options: [...new Set(this.$store.state.tags.map(x => x.name))]}
				],
				rows: transactionsForDisplay.map(x => ([
					x.id,
					x.account.name,
					x.recipient.name,
					new Date(x.timestamp).toISOString().substring(0, 10),
					`${x.amount / x.currency.minor_in_mayor}${x.currency.symbol}`,
					x.comment,
					this.$store.state.tags.filter(y => x.tag_ids?.includes(y.id)).map(y => y.name).join(", ")
				]))
			}
		},

		rowClick(row) {
			const rowFromStore = this.$store.state.transactions.filter(x => x.id == row[0])[0]
			this.selectedRow = {...rowFromStore, amount: rowFromStore.amount / 100, timestamp: rowFromStore.timestamp.slice(0, -1)};
			this.detailsOpen = false;
			this.$nextTick(() => this.detailsOpen = true);
		},

		rowSelect(rows) {
			this.selectedRows = null;
			this.selectedRows = rows;
		},

		async newTransaction() {
			this.selectedRow = {
				id: "",
				account_id: 0,
				currency_id: 0,
				recipient_id: 0,
				status: 1,
				timestamp: new Date(Date.now() - new Date().getTimezoneOffset() * 60000).toISOString().slice(0, -8),
				amount: 0,
				comment: "",
				currency: this.$store.state.currencies.filter(x => x.id == 0)[0]
			}

			this.detailsOpen = false;
			this.$nextTick(() => this.detailsOpen = true);
		},

		async applyBatchEdit() {
			await Promise.all(this.selectedRows.map(async row => {
				let transaction = {...this.$store.state.transactions.filter(x => row && x.id === row[0])[0]};
				transaction.account_id = Number.isInteger(this.batchaccount_id) ? this.batchaccount_id : transaction.account_id;
				transaction.recipient_id = Number.isInteger(this.batchrecipient_id) ? this.batchrecipient_id : transaction.recipient_id;

				await this.$axios.$put(`/api/v1/transactions/${transaction.id}`, transaction);
			}));
			this.batchaccount_id = null;
			this.batchrecipient_id = null;
			setTimeout(() => this.updateAndLoadTable(), 100);
		},

		async updateAndLoadTable() {
			await this.$store.dispatch("fetchTransactions");
			setTimeout(() => this.updateTransactions(), 100);
			this.detailsOpen = false;
		},

		async updateTable() {
			await this.$store.dispatch("fetchTransactions");
			setTimeout(() => this.updateTransactions(), 100);
		}
	}
}
</script>

<style lang="sass" scoped>
div#main
	display: flex
	justify-content: space-between

div#table
	flex-grow: 1

div#detailBar
	padding-left: 8px
</style>
<template>
	<div id="main">
		<div id="table" v-if="tableOpen">
			<div>
				<button class="green" @click="newTransaction">Add</button>
			</div>
			<div v-if="selectedRows.length > 0" id="batchEditContainer">
				<div id="batchEdit">
					<div>
						<label for="account">Account:</label>
						<select id="account" v-model="batchaccount_id">
							<option v-for="(account, index) in $store.state.accounts" :key="index" :value="account.id">{{account.name}}</option>
						</select>
					</div>

					<div>
						<label for="recipient">Recipient:</label>
						<select id="recipient" v-model="batchrecipient_id">
							<option v-for="(recipient, index) in $store.state.recipients" :key="index" :value="recipient.id">{{recipient.name}}</option>
						</select>
					</div>

					<div>
						<label for="asset">Asset:</label>
						<select id="asset" v-model="batchasset_id">
							<option v-for="(asset, index) in $store.state.assets" :key="index" :value="asset.id">{{asset.name}}</option>
						</select>
					</div>

					<div>
						<CustomSelect
							:selectData="selectData"
							v-on:update="tagUpdate"
						/>	
					</div>

					<button class="green" @click="applyBatchEdit()">Edit selected rows</button>
				</div>		
			</div>
			<CustomTable
				ref="table"
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
		batchrecipient_id: null,
		batchasset_id: null,
		batchtag_ids: [],
		selectData: null,
	}),

	async fetch() {
		await this.updateTransactions();

		this.selectData = {
			options: [...this.$store.state.tags.map(x => ({id: x.id, name: x.name}))],
			selected: undefined,
			label: "Tags:",
			openTop: true
		}
	},

	mounted() {
		const id = Number(this.$route.path.split("/")[2]);
		if(Number.isInteger(id)) {
			this.openDetailPage(id);
		}
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
				sumColumn: 5,
				defaultSort: {
					column: 4,
					sort: "desc"
				},
				columns: [
					{name: "ID", type: "number"},
					{name: "Account", type: "choice", options: [...new Set(this.$store.state.accounts.map(x => x.name))]},
					{name: "Recipient", type: "choice", options: [...new Set(this.$store.state.recipients.map(x => x.name))]},
					{name: "Asset", type: "choice", options: [...new Set(this.$store.state.assets.map(x => x.name).sort((a, b) => a > b ? 1 : -1))]},
					{name: "Timestamp", type: "date"},
					{name: "Amount", type: "number"},
					{name: "Comment", type: "string"},
					{name: "Tags", type: "choice", options: [...new Set(this.$store.state.tags.map(x => x.name))]}
				],
				rows: transactionsForDisplay.map(x => ([
					x.id,
					x.account.name,
					x.recipient.name,
					x.asset ? x.asset.name : "",
					new Date(x.timestamp).toISOString().substring(0, 10),
					`${x.amount / x.currency.minor_in_mayor}${x.currency.symbol}`,
					x.comment,
					this.$store.state.tags.filter(y => x.tag_ids?.includes(y.id)).map(y => y.name).join(", ")
				]))
			}
		},

		rowClick(row) {
			if(this.selectedRow.id === row[0]) return;
			history.pushState({}, "", `/transactions/${row[0]}`);
			this.openDetailPage(row[0]);
		},

		openDetailPage(transaction_id) {
			const rowFromStore = this.$store.state.transactions.filter(x => x.id == transaction_id)[0];
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

		tagUpdate(selected) {
			this.batchtag_ids = selected;
		},

		async applyBatchEdit() {
			await Promise.all(this.selectedRows.map(async row => {
				let transaction = {...this.$store.state.transactions.filter(x => row && x.id === row[0])[0]};
				transaction.account_id = Number.isInteger(this.batchaccount_id) ? this.batchaccount_id : transaction.account_id;
				transaction.recipient_id = Number.isInteger(this.batchrecipient_id) ? this.batchrecipient_id : transaction.recipient_id;
				transaction.asset_id = Number.isInteger(this.batchasset_id) ? this.batchasset_id : transaction.asset_id;
				transaction.tag_ids = this.batchtag_ids.length > 0 ? this.batchtag_ids : transaction.tag_ids;

				try {
					await this.$axios.$put(`/api/v1/transactions/${transaction.id}`, transaction);
				} catch(e) {
					console.error(e.response);
					window.alert(e.response.data);
					return;
				}
			}));
			this.batchaccount_id = null;
			this.batchrecipient_id = null;
			setTimeout(() => this.updateAndLoadTable(), 100);
		},

		async updateAndLoadTable() {
			await this.$store.dispatch("fetchTransactions");
			setTimeout(() => this.updateTransactions(), 100);
			this.detailsOpen = false;
			history.pushState({}, "", "/transactions");
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
	overflow: hidden
	height: 100vh

div#table
	flex-grow: 1
	overflow: auto
	padding-bottom: 20px

div#detailBar
	padding-left: 8px
	flex-shrink: 0

div#batchEdit
	display: flex
</style>
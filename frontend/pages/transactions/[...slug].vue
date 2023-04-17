<template>
	<div id="main">
		<div id="table">
			<div>
				<button class="green" @click="newTransaction">Add</button>
			</div>
			<div v-if="selectedRows && selectedRows.length > 0" id="batchEditContainer">
				<div id="batchEdit">
					<div>
						<label for="account">Account:</label>
						<select id="account" v-model="batchaccount_id">
							<option v-for="(account, index) in accounts" :key="index" :value="account.id">{{account.name}}</option>
						</select>
					</div>

					<div>
						<label for="recipient">Recipient:</label>
						<select id="recipient" v-model="batchrecipient_id">
							<option v-for="(recipient, index) in recipients" :key="index" :value="recipient.id">{{recipient.name}}</option>
						</select>
					</div>

					<div>
						<label for="asset">Asset:</label>
						<select id="asset" v-model="batchasset_id">
							<option v-for="(asset, index) in assets" :key="index" :value="asset.id">{{asset.name}}</option>
						</select>
					</div>

					<div>
						<CustomSelect
							v-if="selectData"
							:selectData="selectData"
							v-on:update="tagUpdate"
						/>	
					</div>

					<button class="green" @click="applyBatchEdit()">Edit selected rows</button>
				</div>		
			</div>
			<CustomTable
				ref="table"
				v-if="tableData"
				:tableData="tableData"
				v-on:rowClick="rowClick"
				v-on:rowSelect="rowSelect"
			/>
		</div>

		<div v-if="detailsOpen" id="detailBar">
			<TransactionDetails 
				v-if="selectedRow"
				:transaction="selectedRow"
				v-on:back="updateAndLoadTable"
				v-on:updateData="updateTable"
			/>
		</div>
	</div>
</template>

<script lang="ts">
export default {
	data: () => ({
		tableData: null,
		detailsOpen: false,
		selectedRow: {} as Transaction | null,
		selectedRows: [] as Transaction[] | null,
		batchaccount_id: null,
		batchrecipient_id: null,
		batchasset_id: null,
		batchtag_ids: [],
		selectData: null as unknown as SelectData,
		tags: [] as Tag[],
		accounts: [] as Account[],
		currencies: [] as Currency[],
		recipients: [] as Recipient[],
		assets: [] as Asset[],
		transactions: [] as Transaction[],
	}),
	
	async mounted() {
		this.tags = await $fetch("/api/v1/tags/all");
		this.accounts = await $fetch("/api/v1/accounts/all");
		this.currencies = await $fetch("/api/v1/currencies/all");
		this.recipients = await $fetch("/api/v1/recipients/all");
		this.assets = await $fetch("/api/v1/assets/all");
		this.transactions = await $fetch("/api/v1/transactions/all");
		this.updateTransactions();
	
		this.selectData = {
			options: [...this.tags.map(x => ({id: x.id ? x.id : -1, name: x.name}))],
			selected: undefined,
			label: "Tags:",
			openTop: true
		}

		const id = Number(useRoute().path.split("/")[2]);
		if(Number.isInteger(id)) {
			this.openDetailPage(id);
		}
	},

	methods: {
		updateTransactions() {
			const transactionsForDisplay = this.transactions.map(x => {
				x.account = this.accounts.filter(a => a.id == x.account_id)[0];
				x.currency = this.currencies.filter(c => c.id == x.currency_id)[0];
				x.recipient = this.recipients.filter(r => r.id == x.recipient_id)[0];
				return x;
			});

			(this as any).tableData = {
				multiSelect: true,
				displaySum: true,
				sumColumn: 5,
				defaultSort: {
					column: 4,
					sort: "desc"
				},
				columns: [
					{name: "ID", type: "number"},
					{name: "Account", type: "choice", options: [...new Set(this.accounts.map(x => x.name))]},
					{name: "Recipient", type: "choice", options: [...new Set(this.recipients.map(x => x.name))]},
					{name: "Asset", type: "choice", options: [...new Set(this.assets.map(x => x.name).sort((a, b) => a > b ? 1 : -1))]},
					{name: "Timestamp", type: "date"},
					{name: "Amount", type: "number"},
					{name: "Comment", type: "string"},
					{name: "Tags", type: "choice", options: [...new Set(this.tags.map(x => x.name))]}
				],
				rows: transactionsForDisplay.map(x => ([
					x.id,
					x.account?.name,
					x.recipient?.name,
					x.asset ? x.asset.name : "",
					new Date(x.timestamp).toISOString().substring(0, 10),
					`${x.amount / (x.currency?.minor_in_mayor ? x.currency?.minor_in_mayor : 100)}${x.currency?.symbol}`,
					x.comment,
					this.tags.filter(y => x.tag_ids?.includes(y.id ? y.id : -1)).map(y => y.name).join(", ")
				]))
			}
		},

		rowClick(row: Row) {
			if((this.selectedRow as any).id === row[0]) return;
			history.pushState({}, "", `/transactions/${row[0]}`);
			this.openDetailPage(row[0]);
		},

		openDetailPage(transaction_id: number) {
			const transaction = this.transactions.filter(x => x.id == transaction_id)[0];
			this.selectedRow = {...transaction, amount: transaction.amount / 100, timestamp: transaction.timestamp.slice(0, -1)};			
			this.detailsOpen = false;
			this.$nextTick(() => this.detailsOpen = true);
		},

		rowSelect(rows: any) {
			this.selectedRows = null;
			this.selectedRows = rows;
		},

		async newTransaction() {
			this.selectedRow = {
				account_id: 0,
				currency_id: 0,
				recipient_id: 0,
				status: 1,
				timestamp: new Date(Date.now() - new Date().getTimezoneOffset() * 60000).toISOString().slice(0, -8),
				amount: 0,
				comment: "",
				currency: this.currencies.filter(x => x.id == 0)[0]
			}

			this.detailsOpen = false;
			this.$nextTick(() => this.detailsOpen = true);
		},

		tagUpdate(selected: any) {
			this.batchtag_ids = selected;
		},

		async applyBatchEdit() {
			if(!this.selectedRows) return;
			await Promise.all(this.selectedRows.map(async row => {
				let transaction = {...this.transactions.filter(x => row && x.id === (row as any)[0])[0]};
				(transaction as any).account_id = Number.isInteger(this.batchaccount_id) ? this.batchaccount_id : transaction.account_id;
				(transaction as any).recipient_id = Number.isInteger(this.batchrecipient_id) ? this.batchrecipient_id : transaction.recipient_id;
				(transaction as any).asset_id = Number.isInteger(this.batchasset_id) ? this.batchasset_id : transaction.asset_id;
				transaction.tag_ids = this.batchtag_ids.length > 0 ? this.batchtag_ids : transaction.tag_ids;

				try {
					await $fetch(`/api/v1/transactions/${transaction.id}`, {
						method: "PUT",
						body: transaction
					});
				} catch(e: any) {
					console.error(e?.data?.data);
					window.alert(e?.data?.data?.error);
					return;
				}
			}));
			this.batchaccount_id = null;
			this.batchrecipient_id = null;
			this.updateAndLoadTable();
		},

		async updateAndLoadTable() {
			this.transactions = await $fetch("/api/v1/transactions/all");
			this.updateTransactions();
			this.detailsOpen = false;
			this.selectedRow = null;
			history.pushState({}, "", "/transactions");
		},

		async updateTable() {
			this.transactions = await $fetch("/api/v1/transactions/all");
			this.updateTransactions();
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
<template>
	<div id="main">
		<div id="table">
			<div>
				<button class="green" @click="newTransaction">Add</button>
			</div>
			<div v-if="selectedRows.length > 0" id="batchEditContainer">
				<div id="batchEdit">
					<div>
						<label for="account">Account:</label>
						<select id="account" v-model="batchaccount_id">
							<option v-for="(account, index) in (accounts as any)" :key="index" :value="account.id">{{account.name}}</option>
						</select>
					</div>

					<div>
						<label for="recipient">Recipient:</label>
						<select id="recipient" v-model="batchrecipient_id">
							<option v-for="(recipient, index) in (recipients as any)" :key="index" :value="recipient.id">{{recipient.name}}</option>
						</select>
					</div>

					<div>
						<label for="asset">Asset:</label>
						<select id="asset" v-model="batchasset_id">
							<option v-for="(asset, index) in (assets as any)" :key="index" :value="asset.id">{{asset.name}}</option>
						</select>
					</div>

					<div>
						<CustomSelect
							:selectData="(selectData as any)"
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
		selectedRow: {},
		selectedRows: [],
		batchaccount_id: null,
		batchrecipient_id: null,
		batchasset_id: null,
		batchtag_ids: [],
		selectData: null,
		tags: [],
		accounts: [],
		currencies: [],
		recipients: [],
		assets: [],
		transactions: []
	}),
	
	async mounted() {
		this.tags = await $fetch("/api/v1/tags/all");
		this.accounts = await $fetch("/api/v1/accounts/all");
		this.currencies = await $fetch("/api/v1/currencies/all");
		this.recipients = await $fetch("/api/v1/recipients/all");
		this.assets = await $fetch("/api/v1/assets/all");
		this.transactions = await $fetch("/api/v1/transactions/all");
		this.updateTransactions();
	
		(this as any).selectData = {
			options: [...this.tags.map((x: any) => ({id: x.id, name: x.name}))],
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
			const transactionsForDisplay = this.transactions.map((x: any) => {
				x.account = this.accounts.filter((a: any) => a.id == x.account_id)[0];
				x.currency = this.currencies.filter((c: any) => c.id == x.currency_id)[0];
				x.recipient = this.recipients.filter((r: any) => r.id == x.recipient_id)[0];
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
					{name: "Account", type: "choice", options: [...new Set(this.accounts.map((x: any) => x.name))]},
					{name: "Recipient", type: "choice", options: [...new Set(this.recipients.map((x: any) => x.name))]},
					{name: "Asset", type: "choice", options: [...new Set(this.assets.map((x: any) => x.name).sort((a, b) => a > b ? 1 : -1))]},
					{name: "Timestamp", type: "date"},
					{name: "Amount", type: "number"},
					{name: "Comment", type: "string"},
					{name: "Tags", type: "choice", options: [...new Set(this.tags.map((x: any) => x.name))]}
				],
				rows: transactionsForDisplay.map(x => ([
					x.id,
					x.account.name,
					x.recipient.name,
					x.asset ? x.asset.name : "",
					new Date(x.timestamp).toISOString().substring(0, 10),
					`${x.amount / x.currency.minor_in_mayor}${x.currency.symbol}`,
					x.comment,
					this.tags.filter((y: any) => x.tag_ids?.includes(y.id)).map((y: any) => y.name).join(", ")
				]))
			}
		},

		rowClick(row: any) {
			if((this.selectedRow as any).id === row[0]) return;
			history.pushState({}, "", `/transactions/${row[0]}`);
			this.openDetailPage(row[0]);
		},

		openDetailPage(transaction_id: any) {
			const rowFromStore: any = this.transactions.filter((x: any) => x.id == transaction_id)[0];
			this.selectedRow = {...rowFromStore, amount: rowFromStore.amount / 100, timestamp: rowFromStore.timestamp.slice(0, -1)};			
			this.detailsOpen = false;
			this.$nextTick(() => this.detailsOpen = true);
		},

		rowSelect(rows: any) {
			(this as any).selectedRows = null;
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
				currency: this.currencies.filter((x: any) => x.id == 0)[0]
			}

			this.detailsOpen = false;
			this.$nextTick(() => this.detailsOpen = true);
		},

		tagUpdate(selected: any) {
			this.batchtag_ids = selected;
		},

		async applyBatchEdit() {
			await Promise.all(this.selectedRows.map(async row => {
				let transaction = {...(this as any).transactions.filter((x: any) => row && x.id === row[0])[0]};
				transaction.account_id = Number.isInteger(this.batchaccount_id) ? this.batchaccount_id : transaction.account_id;
				transaction.recipient_id = Number.isInteger(this.batchrecipient_id) ? this.batchrecipient_id : transaction.recipient_id;
				transaction.asset_id = Number.isInteger(this.batchasset_id) ? this.batchasset_id : transaction.asset_id;
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
			this.selectedRow = {};
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
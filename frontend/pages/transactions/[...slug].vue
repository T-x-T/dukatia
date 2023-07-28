<template>
	<div id="main">		
		<div v-if="!(small_device && detailsOpen)" id="top_controls">
			<button class="green" @click="newTransaction">Add</button>
		</div>

		<div id="table_and_details">
			<div id="table">
				<CustomTable
					v-if="Object.keys(tableData).length > 0 && !(small_device && detailsOpen)"
					:tableDataProp="tableData"
					v-on:rowClick="rowClick"
					v-on:rowSelect="rowSelect"
					v-on:updatePage="updatePage"
					v-on:updateSort="updateSort"
				/>
			</div>

			<div v-if="selectedRows && selectedRows.length > 0" class="detailBar">
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
							v-if="Object.keys(selectData).length > 0"
							:selectData="selectData"
							v-on:update="tagUpdate"
						/>	
					</div>

					<button class="green" @click="applyBatchEdit()">Edit selected</button>
					<button class="red" @click="deleteBatchEdit()">Delete selected</button>
				</div>		
			</div>

			<div v-if="detailsOpen && selectedRows.length === 0" class="detailBar">
				<TransactionDetails 
					v-if="Object.keys(selectedRow).length > 0"
					:transaction="selectedRow"
					v-on:back="updateAndLoadTable"
					v-on:updateData="updateTable"
				/>
			</div>
		</div>
	</div>
</template>

<script lang="ts">
export default {
	data: () => ({
		tableData: {} as TableData,
		detailsOpen: false,
		selectedRow: {} as Transaction,
		selectedRows: [] as Row[],
		batchaccount_id: null as null | number,
		batchrecipient_id: null as null | number,
		batchasset_id: null as null | number,
		batchtag_ids: [],
		selectData: {} as SelectData,
		tags: [] as Tag[],
		accounts: [] as Account[],
		currencies: [] as Currency[],
		recipients: [] as Recipient[],
		assets: [] as Asset[],
		transactions: [] as Transaction[],
		small_device: false,
		query_parameters: {
			skip_results: 0,
			max_results: 50,
			sort_property: "timestamp",
			sort_direction: "DESC",
		} as QueryParameters,
		total_row_count: 0,
		total_amount: 0,
	}),
	
	async mounted() {
		this.$nextTick(() => {
      window.addEventListener('resize', this.on_resize);
    });
		this.on_resize();

		this.tags = await $fetch("/api/v1/tags/all");
		this.accounts = await $fetch("/api/v1/accounts/all");
		this.currencies = await $fetch("/api/v1/currencies/all");
		this.recipients = await $fetch("/api/v1/recipients/all");
		this.assets = await $fetch("/api/v1/assets/all");
		this.transactions = await $fetch(`/api/v1/transactions/all
			?skip_results=${this.query_parameters.skip_results}
			&max_results=${this.query_parameters.max_results}
			&sort_property=${this.query_parameters.sort_property}
			&sort_direction=${this.query_parameters.sort_direction}
		`);
		const summary = await $fetch("/api/v1/transactions/summary");
		this.total_row_count = summary.count;
		this.total_amount = summary.total_amount;
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
			this.tableData = {} as TableData;
			this.$nextTick(() => {
				this.tableData = {
					multiSelect: true,
					displaySum: true,
					sumColumn: 5,
					row_count: this.total_row_count,
					total_amount: this.total_amount,
					defaultSort: {
						column: 4,
						sort: "desc"
					},
					columns: [
						{name: "ID", type: "number", sortable: true},
						{name: "Account", type: "choice", options: [...new Set(this.accounts.map(x => x.name))]},
						{name: "Recipient", type: "choice", options: [...new Set(this.recipients.map(x => x.name))]},
						{name: "Asset", type: "choice", options: [...new Set(this.assets.map(x => x.name).sort((a, b) => a > b ? 1 : -1))]},
						{name: "Timestamp", type: "date", sortable: true},
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
						`${x.total_amount / (x.currency?.minor_in_mayor ? x.currency?.minor_in_mayor : 100)}${x.currency?.symbol}`,
						x.comment,
						this.tags.filter(y => x.tag_ids?.includes((Number.isInteger(y.id) ? y.id : -1) as number)).map(y => y.name).join(", ")
					]))
				};
			});
		},

		rowClick(row: Row) {
			if(this.selectedRow?.id === row[0]) return;
			history.pushState({}, "", `/transactions/${row[0]}`);
			this.openDetailPage(row[0]);
		},

		openDetailPage(transaction_id: number) {
			const transaction = this.transactions.filter(x => x.id == transaction_id)[0];
			this.selectedRow = {...transaction, total_amount: transaction.total_amount / 100, timestamp: transaction.timestamp.slice(0, -1)};			
			this.detailsOpen = false;
			this.$nextTick(() => this.detailsOpen = true);
		},

		rowSelect(rows: Row) {
			this.selectedRows = rows;
		},

		async newTransaction() {
			this.selectedRow = {
				account_id: 0,
				currency_id: 0,
				recipient_id: 0,
				status: 1,
				timestamp: new Date(Date.now() - new Date().getTimezoneOffset() * 60000).toISOString().slice(0, -8),
				total_amount: 0,
				comment: "",
				currency: this.currencies.filter(x => x.id == 0)[0],
				positions: [
					{
						amount: 0,
						comment: "",
					}
				],
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
				let transaction = {...this.transactions.filter(x => row && x.id === row[0])[0]};
				transaction.account_id = typeof this.batchaccount_id == "number" ? this.batchaccount_id : transaction.account_id;
				transaction.recipient_id = typeof this.batchrecipient_id == "number" ? this.batchrecipient_id : transaction.recipient_id;
				transaction.asset_id = typeof this.batchasset_id == "number"  ? this.batchasset_id : transaction.asset_id;
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

		async deleteBatchEdit() {
			if(!this.selectedRows) return;

			await Promise.all(this.selectedRows.map(async row => {
				try {
					await $fetch(`/api/v1/transactions/${row[0]}`, {
						method: "DELETE",
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
			await this.updateTable();
			this.detailsOpen = false;
			this.selectedRow = {} as Transaction;
			history.pushState({}, "", "/transactions");
		},

		async updatePage(current_page: number, page_size: number) {
			this.query_parameters.skip_results = current_page * page_size;
			this.query_parameters.max_results = page_size;
			await this.updateTable();
		},

		async updateSort(property_name: string, direction: "asc" | "desc") {
			this.query_parameters.sort_property = property_name.toLowerCase();
			this.query_parameters.sort_direction = direction.toUpperCase() as "ASC" | "DESC";
			await this.updateTable();
		},

		async updateTable() {
			this.transactions = await $fetch(`/api/v1/transactions/all
				?skip_results=${this.query_parameters.skip_results}
				&max_results=${this.query_parameters.max_results}
				&sort_property=${this.query_parameters.sort_property}
				&sort_direction=${this.query_parameters.sort_direction}
			`);

			const transactionsForDisplay = this.transactions.map(x => {
				x.account = this.accounts.filter(a => a.id == x.account_id)[0];
				x.currency = this.currencies.filter(c => c.id == x.currency_id)[0];
				x.recipient = this.recipients.filter(r => r.id == x.recipient_id)[0];
				return x;
			});
			this.tableData.rows = transactionsForDisplay.map(x => ([
				x.id,
				x.account?.name,
				x.recipient?.name,
				x.asset ? x.asset.name : "",
				new Date(x.timestamp).toISOString().substring(0, 10),
				`${x.total_amount / (x.currency?.minor_in_mayor ? x.currency?.minor_in_mayor : 100)}${x.currency?.symbol}`,
				x.comment,
				this.tags.filter(y => x.tag_ids?.includes((Number.isInteger(y.id) ? y.id : -1) as number)).map(y => y.name).join(", ")
			]));
		},

		on_resize() {
			this.small_device = window.innerWidth <= 800;
		}
	}
}
</script>

<style lang="sass" scoped>
div#main
	height: 100svh

div#table_and_details
	display: flex

div#table
	overflow: auto

div.detailBar
	padding-left: 8px
	@media screen and (max-width: 800px)
		position: absolute
		width: 100%
		height: 100%
		margin: 0

div#batchEdit
	select
		max-width: 10em
	button
		margin: 0
		margin-left: 1em
		height: 100%

</style>
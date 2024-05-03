<template>
	<div id="main">		
		<div id="table_and_details">
			<div id="table">
				<div v-if="!(small_device && detailsOpen)" id="top_controls">
					<button id="add" class="green" @click="newTransaction">Add</button>
				</div>

				<CustomTable
					v-if="Object.keys(tableData).length > 0 && !(small_device && detailsOpen)"
					:tableDataProp="tableData"
					@rowClick="rowClick"
					@rowSelect="rowSelect"
					@updatePage="updatePage"
					@updateSort="updateSort"
					@updateFilter="updateFilter"
					@resetFilter="resetFilter"
					@applyFilter="applyFilter"
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
						<InputMultiSelect
							v-if="Object.keys(selectData).length > 0"
							:selectData="selectData"
							@update="tagUpdate"
						/>	
					</div>

					<button class="green" @click="applyBatchEdit()">Edit selected</button>
					<button class="red" @click="deleteBatchEdit()">Delete selected</button>
				</div>		
			</div>

			<div v-if="detailsOpen && selectedRows.length === 0" class="detailBar">
				<TransactionDetails 
					v-if="Object.keys(selectedRow).length > 0"
					:prop_transaction="selectedRow"
					:default_transaction="default_transaction"
					@back="closeDetails"
					@updateData="updateTable"
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
			sort_direction: "desc",
		} as QueryParameters,
		total_row_count: 0,
		total_amount: 0,
		data_revision: 0,
		default_transaction: {} as Transaction,
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
		this.transactions = await $fetch(this.build_request_url("/api/v1/transactions/all"));
		const summary = await $fetch(this.build_request_url("/api/v1/transactions/summary")) as any;
		this.total_row_count = summary.count;
		this.total_amount = summary.total_amount;
		this.updateTransactions();
	
		this.selectData = {
			options: [...this.tags.map(x => ({id: x.id?.length == 36 ? x.id : "", name: x.name}))],
			selected: undefined,
			label: "Tags:",
			openTop: true
		}

		if (useRoute().path.split("/")[2] == "new") {
			this.$nextTick(() => this.newTransaction());
		} else if(useRoute().path[useRoute().path.length - 1] != "/" && Number.isInteger(Number(useRoute().path.split("/")[2]))) {
			const id = useRoute().path.split("/")[2];
			if(this.transactions.filter(x => x.id === id).length === 0) {
				this.transactions.push(await $fetch(`/api/v1/transactions/${id}`));
			}
			this.openDetailPage(id);
		}

		this.default_transaction = {
			account_id: this.accounts[0].id,
			currency_id: this.accounts[0].default_currency_id,
			recipient_id: this.recipients[0].id,
			tag_ids: [],
			status: 1,
			timestamp: new Date(Date.now() - new Date().getTimezoneOffset() * 60000).toISOString().slice(0, -8),
			total_amount: {
				major: 0,
				minor: 0,
				minor_in_major: this.currencies[0].minor_in_major,
				symbol: this.currencies[0].symbol,
				is_negative: false,
			},
			comment: "",
			currency: structuredClone(toRaw(this.currencies[0])),
			positions: [{
				amount: {
					major: 0,
					minor: 0,
					minor_in_major: this.currencies[0].minor_in_major,
					symbol: this.currencies[0].symbol,
					is_negative: false,
				},
				comment: "",
			}],
		};
	},

	methods: {
		updateTransactions() {
			const transactionsForDisplay = this.transactions.map(x => {
				x.account = this.accounts.filter(a => a.id === x.account_id)[0];
				x.currency = this.currencies.filter(c => c.id === x.currency_id)[0];
				x.recipient = this.recipients.filter(r => r.id === x.recipient_id)[0];
				return x;
			});
			this.tableData = {} as TableData;
			this.$nextTick(() => {
				this.tableData = {
					multiSelect: true,
					row_count: this.total_row_count,
					total_amount: this.total_amount,
					defaultSort: {
						column: 4,
						sort: "desc"
					},
					columns: [
						{name: "ID", type: "number", sortable: true, hidden: true},
						{name: "Account", type: "choice", sortable: true, options: this.accounts.map(x => ({id: x.id, name: x.name}))},
						{name: "Recipient", type: "choice", sortable: true, options: this.recipients.map(x => ({id: x.id, name: x.name}))},
						{name: "Asset", type: "choice", options: this.assets.map(x => ({id: x.id, name: x.name}))},
						{name: "Timestamp", type: "date", sortable: true},
						{name: "Amount", type: "number", sortable: true},
						{name: "Comment", type: "string", sortable: true},
						{name: "Tags", type: "choice", options: this.tags.map(x => ({id: x.id, name: x.name}))},
					],
					rows: transactionsForDisplay.map(this.get_row)
				};
			});
		},

		rowClick(row: Row) {
			if(this.selectedRow?.id === row[0]) return;
			history.pushState({}, "", `/transactions/${row[0]}`);
			this.openDetailPage(row[0]);
		},

		openDetailPage(transaction_id: string) {
			const transaction = this.transactions.filter(x => x.id === transaction_id)[0];
			this.selectedRow = {...transaction, timestamp: transaction.timestamp.slice(0, -1)};			
			this.detailsOpen = false;
			this.$nextTick(() => this.detailsOpen = true);
		},

		rowSelect(rows: Row) {
			this.selectedRows = rows;
		},

		async newTransaction() {
			this.selectedRow = structuredClone(toRaw(this.default_transaction));

			if(useRoute().path != "/transactions/new") {
				history.pushState({}, "", `/transactions/new`);
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
				transaction.account_id = typeof this.batchaccount_id == "string" ? this.batchaccount_id : transaction.account_id;
				transaction.recipient_id = typeof this.batchrecipient_id == "string" ? this.batchrecipient_id : transaction.recipient_id;
				transaction.asset_id = typeof this.batchasset_id == "string" ? this.batchasset_id : transaction.asset_id;
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
			this.closeDetails();
			await this.updateTable();
		},
		
		closeDetails() {
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
			this.query_parameters.sort_direction = direction;
			await this.updateTable();
		},

		async updateFilter(property_name: string, value: any, mode: string) {
			property_name = property_name.toLowerCase();
			switch(property_name) {
				case "id": {
					this.query_parameters.filter_id = value;
					this.query_parameters.filter_mode_id = mode;
					break;
				}
				case "comment": {
					this.query_parameters.filter_comment = value;
					this.query_parameters.filter_mode_comment = mode;
					break;
				}
				case "timestamp": {
					this.query_parameters.filter_time_range_lower = value.lower;
					this.query_parameters.filter_time_range_upper = value.upper;
					this.query_parameters.filter_mode_time_range = mode;
					break;
				}
				case "account": {
					this.query_parameters.filter_account_id = value;
					this.query_parameters.filter_mode_account_id = mode;
					break;
				}
				case "recipient": {
					this.query_parameters.filter_recipient_id = value;
					this.query_parameters.filter_mode_recipient_id = mode;
					break;
				}
				case "tags": {
					this.query_parameters.filter_tag_id = value;
					this.query_parameters.filter_mode_tag_id = mode;
					break;
				}
				case "asset": {
					this.query_parameters.filter_asset_id = value;
					this.query_parameters.filter_mode_asset_id = mode;
					break;
				}
				case "amount": {
					this.query_parameters.filter_total_amount = value;
					this.query_parameters.filter_mode_total_amount = mode;
					break;
				}
			}
		},

		async resetFilter(property_name: string) {
			property_name = property_name.toLowerCase();
			switch(property_name) {
				case "id": {
					this.query_parameters.filter_id = undefined;
					this.query_parameters.filter_mode_id = undefined;
					break;
				}
				case "comment": {
					this.query_parameters.filter_comment = undefined;
					this.query_parameters.filter_mode_comment = undefined;
					break;
				}
				case "timestamp": {
					this.query_parameters.filter_time_range_lower = undefined;
					this.query_parameters.filter_time_range_upper = undefined;
					this.query_parameters.filter_mode_time_range = undefined;
					break;
				}
				case "account": {
					this.query_parameters.filter_account_id = undefined;
					this.query_parameters.filter_mode_account_id = undefined;
					break;
				}
				case "recipient": {
					this.query_parameters.filter_recipient_id = undefined;
					this.query_parameters.filter_mode_recipient_id = undefined;
					break;
				}
				case "tags": {
					this.query_parameters.filter_tag_id = undefined;
					this.query_parameters.filter_mode_tag_id = undefined;
					break;
				}
				case "asset": {
					this.query_parameters.filter_asset_id = undefined;
					this.query_parameters.filter_mode_asset_id = undefined;
					break;
				}
				case "amount": {
					this.query_parameters.filter_total_amount = undefined;
					this.query_parameters.filter_mode_total_amount = undefined;
					break;
				}
			}
		},

		async applyFilter() {
			await this.updateTable();
		},

		async updateTable() {
			this.data_revision += 1;
			const local_data_revision = this.data_revision;
			this.transactions = await $fetch(this.build_request_url("/api/v1/transactions/all"));
			const summary = await $fetch(this.build_request_url("/api/v1/transactions/summary")) as any;
			if(this.data_revision > local_data_revision) return;

			this.total_row_count = summary.count;
			this.total_amount = summary.total_amount;

			const transactionsForDisplay = this.transactions.map(x => {
				x.account = this.accounts.filter(a => a.id == x.account_id)[0];
				x.currency = this.currencies.filter(c => c.id == x.currency_id)[0];
				x.recipient = this.recipients.filter(r => r.id == x.recipient_id)[0];
				return x;
			});
			this.tableData.rows = transactionsForDisplay.map(this.get_row);
			this.tableData.row_count = this.total_row_count;
			this.tableData.total_amount = this.total_amount;
		},

		on_resize() {
			this.small_device = window.innerWidth <= 800;
		},

		build_request_url(base_url: string) {
			let url = `${base_url}
				?skip_results=${this.query_parameters.skip_results}
				&max_results=${this.query_parameters.max_results}
				&sort_property=${this.query_parameters.sort_property}
				&sort_direction=${this.query_parameters.sort_direction}`;

			if(this.query_parameters.filter_id) url += `&filter_id=${this.query_parameters.filter_id}`;
			if(this.query_parameters.filter_mode_id) url += `&filter_mode_id=${this.query_parameters.filter_mode_id}`;
			if(this.query_parameters.filter_asset_id) url += `&filter_asset_id=${this.query_parameters.filter_asset_id}`;
			if(this.query_parameters.filter_mode_asset_id) url += `&filter_mode_asset_id=${this.query_parameters.filter_mode_asset_id}`;
			if(this.query_parameters.filter_account_id) url += `&filter_account_id=${this.query_parameters.filter_account_id}`;
			if(this.query_parameters.filter_mode_account_id) url += `&filter_mode_account_id=${this.query_parameters.filter_mode_account_id}`;
			if(this.query_parameters.filter_recipient_id) url += `&filter_recipient_id=${this.query_parameters.filter_recipient_id}`;
			if(this.query_parameters.filter_mode_recipient_id) url += `&filter_mode_recipient_id=${this.query_parameters.filter_mode_recipient_id}`;
			if(this.query_parameters.filter_tag_id) url += `&filter_tag_id=${this.query_parameters.filter_tag_id}`;
			if(this.query_parameters.filter_mode_tag_id) url += `&filter_mode_tag_id=${this.query_parameters.filter_mode_tag_id}`;
			if(typeof this.query_parameters.filter_total_amount == "number") url += `&filter_total_amount=${Number(this.query_parameters.filter_total_amount) * 100}`; //TODO not using minor_in_major
			if(this.query_parameters.filter_mode_total_amount) url += `&filter_mode_total_amount=${this.query_parameters.filter_mode_total_amount}`;
			if(this.query_parameters.filter_comment) url += `&filter_comment=${this.query_parameters.filter_comment}`;
			if(this.query_parameters.filter_mode_comment) url += `&filter_mode_comment=${this.query_parameters.filter_mode_comment}`;
			if(this.query_parameters.filter_time_range_lower) url += `&filter_time_range_lower=${new Date(this.query_parameters.filter_time_range_lower).toISOString()}`;
			if(this.query_parameters.filter_time_range_upper) url += `&filter_time_range_upper=${new Date(this.query_parameters.filter_time_range_upper).toISOString()}`;
			if(this.query_parameters.filter_mode_time_range) url += `&filter_mode_time_range=${this.query_parameters.filter_mode_time_range}`;

			return url;
		},

		get_row(x: Transaction) {
			return [
				x.id,
				x.account?.name,
				x.recipient?.name,
				x.asset ? x.asset.name : "",
				new Date(new Date(x.timestamp).valueOf() - (new Date(x.timestamp).getTimezoneOffset() * 60000)).toISOString().slice(0, 10),
				`${x.total_amount.major >= 0 && x.total_amount.is_negative ? "-" : ""}${x.total_amount.major}.${x.total_amount.minor.toString().padStart(x.total_amount.minor_in_major.toString().length - 1, "0")}${x.total_amount.symbol}`,
				x.comment,
				this.tags.filter(y => x.tag_ids?.includes(y.id?.length == 36 ? y.id : "")).map(y => y.name).join(", ")
			];
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
	padding: 10px
	@media screen and (max-width: 800px)
		position: absolute

div#batchEdit
	select
		max-width: 10em
	button
		margin: 0
		margin-left: 1em
		height: 100%

button#add
	margin: 10px

</style>
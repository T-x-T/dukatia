<template>
	<div>
		<button id="add" class="green" @click="newAccount">Add</button>
		<CustomTable
			v-if="Object.keys(tableData).length > 0"
			:tableDataProp="tableData"
			v-on:rowClick="rowClick"
			v-on:updatePage="updatePage"
			v-on:updateFilter="updateFilter"
			v-on:resetFilter="resetFilter"
			v-on:applyFilter="applyFilter"
		/>
	</div>
</template>

<script lang="ts">
export default {
	data: () => ({
		tableData: {} as TableData,
		data_revision: 0,
		query_parameters: {
			skip_results: 0,
			max_results: 50,
		} as QueryParameters,
	}),

	async mounted() {
		const accounts = await $fetch("/api/v1/accounts/all") as Account[];
		const currencies = await $fetch("/api/v1/currencies/all") as Currency[];
		const tags = await $fetch("/api/v1/tags/all") as Tag[];

		this.tableData = {
			multiSelect: false,
			defaultSort: {
				column: 0,
				sort: "asc"
			},
			columns: [
				{name: "ID", type: "number"},
				{name: "Name", type: "string"},
				{name: "Currency", type: "choice", options: currencies.map(x => ({id: x.id, name: x.name}))},
				{name: "Tags", type: "choice", options: (tags.map(x => ({id: x.id, name: x.name})))},
				{name: "Balance", type: "number"}
			],
			rows: accounts.map(x => ([
				x.id,
				x.name,
				currencies.filter(c => c.id == x.default_currency_id)[0].name,
				tags.filter(t => x.tag_ids?.includes(Number.isInteger(t.id) ? Number(t.id) : -1)).map(t => t.name).join(", "),
				`${Number(x.balance) / currencies.filter(c => c.id == x.default_currency_id)[0].minor_in_major}${currencies.filter(c => c.id == x.default_currency_id)[0].symbol}`
			]))
		};
	},

	methods: {
		async rowClick(row: Row) {
			await useRouter().push(`/accounts/${row[0]}`);
		}, 

		async newAccount() {
			await useRouter().push("/accounts/new");
		},

		async updatePage(current_page: number, page_size: number) {
			this.query_parameters.skip_results = current_page * page_size;
			this.query_parameters.max_results = page_size;
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
				case "name": {
					this.query_parameters.filter_name = value;
					this.query_parameters.filter_mode_name = mode;
					break;
				}
				case "currency": {
					this.query_parameters.filter_currency_id = value;
					this.query_parameters.filter_mode_currency_id = mode;
					break;
				}
				case "tags": {
					this.query_parameters.filter_tag_id = value;
					this.query_parameters.filter_mode_tag_id = mode;
					break;
				}
				case "balance": {
					this.query_parameters.filter_balance = value;
					this.query_parameters.filter_mode_balance = mode;
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
				case "name": {
					this.query_parameters.filter_name = undefined;
					this.query_parameters.filter_mode_name = undefined;
					break;
				}
				case "currency": {
					this.query_parameters.filter_currency_id = undefined;
					this.query_parameters.filter_mode_currency_id = undefined;
					break;
				}
				case "tags": {
					this.query_parameters.filter_tag_id = undefined;
					this.query_parameters.filter_mode_tag_id = undefined;
					break;
				}
				case "balance": {
					this.query_parameters.filter_balance = undefined;
					this.query_parameters.filter_mode_balance = undefined;
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
			const accounts = await $fetch(this.build_request_url("/api/v1/accounts/all")) as Account[];
			const currencies = (await useFetch("/api/v1/currencies/all")).data.value as Currency[];
			const tags = (await useFetch("/api/v1/tags/all")).data.value as Tag[];
			if(this.data_revision > local_data_revision) return;

			this.tableData.rows = accounts.map(x => ([
				x.id,
				x.name,
				currencies.filter(c => c.id == x.default_currency_id)[0].name,
				tags.filter(t => x.tag_ids?.includes(Number.isInteger(t.id) ? Number(t.id) : -1)).map(t => t.name).join(", "),
				`${typeof x.balance == "number" ? x.balance : 0 / currencies.filter(c => c.id == x.default_currency_id)[0].minor_in_major}${currencies.filter(c => c.id == x.default_currency_id)[0].symbol}`
			]));
		},

		build_request_url(base_url: string) {
			let url = `${base_url}
				?skip_results=${this.query_parameters.skip_results}
				&max_results=${this.query_parameters.max_results}`;

			if(Number.isInteger(this.query_parameters.filter_id)) url += `&filter_id=${this.query_parameters.filter_id}`;
			if(this.query_parameters.filter_mode_id) url += `&filter_mode_id=${this.query_parameters.filter_mode_id}`;
			if(Number.isInteger(this.query_parameters.filter_currency_id)) url += `&filter_currency_id=${this.query_parameters.filter_currency_id}`;
			if(this.query_parameters.filter_mode_currency_id) url += `&filter_mode_currency_id=${this.query_parameters.filter_mode_currency_id}`;
			if(Number.isInteger(this.query_parameters.filter_tag_id)) url += `&filter_tag_id=${this.query_parameters.filter_tag_id}`;
			if(this.query_parameters.filter_mode_tag_id) url += `&filter_mode_tag_id=${this.query_parameters.filter_mode_tag_id}`;
			if(Number.isInteger(this.query_parameters.filter_balance)) url += `&filter_balance=${this.query_parameters.filter_balance}`;
			if(this.query_parameters.filter_mode_balance) url += `&filter_mode_balance=${this.query_parameters.filter_mode_balance}`;
			if(this.query_parameters.filter_name) url += `&filter_name=${this.query_parameters.filter_name}`;
			if(this.query_parameters.filter_mode_name) url += `&filter_mode_name=${this.query_parameters.filter_mode_name}`;

			return url;
		},
	}
}
</script>

<style lang="sass" scoped>
button#add
	margin: 10px
</style>	
<template>
	<div>
		<button id="add" class="green" @click="newCurrency">Add</button>
		<CustomTable
			v-if="Object.keys(tableData).length > 0"
			:tableDataProp="tableData"
			@rowClick="rowClick"
			@updatePage="updatePage"
			@updateFilter="updateFilter"
			@resetFilter="resetFilter"
			@applyFilter="applyFilter"
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
		const currencies = await $fetch("/api/v1/currencies/all") as Currency[];
		
		this.tableData = {
			multiSelect: false,
			defaultSort: {
				column: 0,
				sort: "asc"
			},
			columns: [
				{name: "ID", type: "string", hidden: true},
				{name: "Name", type: "string"},
				{name: "Symbol", type: "string"},
				{name: "Minor in major", type: "number"},
			],
			rows: currencies.map(x => ([
				x.id,
				x.name,
				x.symbol,
				x.minor_in_major
			]))
		}
	},

	methods: {
		async rowClick(row: Row) {
			await useRouter().push(`/currencies/${row[0]}`);
		},

		async newCurrency() {
			await useRouter().push("/currencies/new");
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
				case "symbol": {
					this.query_parameters.filter_symbol = value;
					this.query_parameters.filter_mode_symbol = mode;
					break;
				}
				case "minor in major": {
					this.query_parameters.filter_minor_in_major = value;
					this.query_parameters.filter_mode_minor_in_major = mode;
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
				case "symbol": {
					this.query_parameters.filter_symbol = undefined;
					this.query_parameters.filter_mode_symbol = undefined;
					break;
				}
				case "minor in major": {
					this.query_parameters.filter_minor_in_major = undefined;
					this.query_parameters.filter_mode_minor_in_major = undefined;
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
			const currencies = await $fetch(this.build_request_url("/api/v1/currencies/all")) as Currency[];
			if(this.data_revision > local_data_revision) return;

			this.tableData.rows = currencies.map(x => ([
				x.id,
				x.name,
				x.symbol,
				x.minor_in_major
			]));
		},

		build_request_url(base_url: string) {
			let url = `${base_url}
				?skip_results=${this.query_parameters.skip_results}
				&max_results=${this.query_parameters.max_results}`;

			if(this.query_parameters.filter_id) url += `&filter_id=${this.query_parameters.filter_id}`;
			if(this.query_parameters.filter_mode_id) url += `&filter_mode_id=${this.query_parameters.filter_mode_id}`;
			if(Number.isInteger(this.query_parameters.filter_minor_in_major)) url += `&filter_minor_in_major=${this.query_parameters.filter_minor_in_major}`;
			if(this.query_parameters.filter_mode_minor_in_major) url += `&filter_mode_minor_in_major=${this.query_parameters.filter_mode_minor_in_major}`;
			if(this.query_parameters.filter_name) url += `&filter_name=${this.query_parameters.filter_name}`;
			if(this.query_parameters.filter_mode_name) url += `&filter_mode_name=${this.query_parameters.filter_mode_name}`;
			if(this.query_parameters.filter_symbol) url += `&filter_symbol=${this.query_parameters.filter_symbol}`;
			if(this.query_parameters.filter_mode_symbol) url += `&filter_mode_symbol=${this.query_parameters.filter_mode_symbol}`;

			return url;
		},
	}
}
</script>

<style lang="sass" scoped>
button#add
	margin: 10px
</style>

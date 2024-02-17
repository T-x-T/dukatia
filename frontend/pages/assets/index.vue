<template>
	<div>
		<button id="add" class="green" @click="newAsset">Add</button>
		<CustomTable
			v-if="Object.keys(tableData).length > 0"
			:tableDataProp="tableData"
			@rowClick="rowClick"
			@updatePage="updatePage"
			@updateFilter="updateFilter"
			@resetFilter="resetFilter"
			@applyFilter="applyFilter"
			@updateSort="updateSort"
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
			sort_property: "id",
			sort_direction: "asc",
		} as QueryParameters,
		tags: {} as Tag[],
	}),

	async mounted() {
		const assets = await $fetch(this.build_request_url("/api/v1/assets/all")) as Asset[];
		this.tags = await $fetch("/api/v1/tags/all") as Tag[];

		this.tableData = {
			multiSelect: false,
			defaultSort: {
				column: 0,
				sort: "asc"
			},
			columns: [
				{name: "ID", type: "number", sortable: true, hidden: true},
				{name: "Name", type: "string", sortable: true},
				{name: "Description", type: "string", sortable: true},
				{name: "Amount", type: "number", sortable: true},
				{name: "Value per Unit", type: "number", sortable: true},
				{name: "Total value", type: "number", no_filter: true},
				{name: "total TCO", type: "number", no_filter: true},
				{name: "monthly TCO", type: "number", no_filter: true},
				{name: "yearly TCO", type: "number", no_filter: true},
				{name: "Tags", type: "choice", options: this.tags.map(x => ({id: x.id, name: x.name}))}
			],
			rows: this.tableData.rows = assets.map(x => this.get_row(x, this.tags)),
		};
	},

	methods: {
		async rowClick(row: Row) {
			await useRouter().push(`/assets/${row[0]}`);
		},

		async newAsset() {
			await useRouter().push("/assets/new");
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
				case "name": {
					this.query_parameters.filter_name = value;
					this.query_parameters.filter_mode_name = mode;
					break;
				}
				case "description": {
					this.query_parameters.filter_description = value;
					this.query_parameters.filter_mode_description = mode;
					break;
				}
				case "amount": {
					this.query_parameters.filter_amount = value;
					this.query_parameters.filter_mode_amount = mode;
					break;
				}
				case "value per unit": {
					this.query_parameters.filter_value_per_unit = value;
					this.query_parameters.filter_mode_value_per_unit = mode;
					break;
				}
				case "tags": {
					this.query_parameters.filter_tag_id = value;
					this.query_parameters.filter_mode_tag_id = mode;
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
				case "description": {
					this.query_parameters.filter_description = undefined;
					this.query_parameters.filter_mode_description = undefined;
					break;
				}
				case "amount": {
					this.query_parameters.filter_amount = undefined;
					this.query_parameters.filter_mode_amount = undefined;
					break;
				}
				case "value per unit": {
					this.query_parameters.filter_value_per_unit = undefined;
					this.query_parameters.filter_mode_value_per_unit = undefined;
					break;
				}
				case "tags": {
					this.query_parameters.filter_tag_id = undefined;
					this.query_parameters.filter_mode_tag_id = undefined;
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
			const assets = await $fetch(this.build_request_url("/api/v1/assets/all")) as Asset[];
			if(this.data_revision > local_data_revision) return;

			this.tableData.rows = assets.map(x => this.get_row(x, this.tags));
		},

		build_request_url(base_url: string) {
			let url = `${base_url}
				?skip_results=${this.query_parameters.skip_results}
				&max_results=${this.query_parameters.max_results}
				&sort_property=${this.query_parameters.sort_property}
				&sort_direction=${this.query_parameters.sort_direction}`;
				
			if(Number.isInteger(this.query_parameters.filter_id)) url += `&filter_id=${this.query_parameters.filter_id}`;
			if(this.query_parameters.filter_mode_id) url += `&filter_mode_id=${this.query_parameters.filter_mode_id}`;
			if(typeof this.query_parameters.filter_amount == "number") url += `&filter_amount=${this.query_parameters.filter_amount}`; 
			if(this.query_parameters.filter_mode_amount) url += `&filter_mode_amount=${this.query_parameters.filter_mode_amount}`;
			if(typeof this.query_parameters.filter_value_per_unit == "number") url += `&filter_value_per_unit=${Number(this.query_parameters.filter_value_per_unit) * 100}`; //TODO not using minor_in_major
			if(this.query_parameters.filter_mode_value_per_unit) url += `&filter_mode_value_per_unit=${this.query_parameters.filter_mode_value_per_unit}`;
			if(this.query_parameters.filter_name) url += `&filter_name=${this.query_parameters.filter_name}`;
			if(this.query_parameters.filter_mode_name) url += `&filter_mode_name=${this.query_parameters.filter_mode_name}`;
			if(this.query_parameters.filter_description) url += `&filter_description=${this.query_parameters.filter_description}`;
			if(this.query_parameters.filter_mode_description) url += `&filter_mode_description=${this.query_parameters.filter_mode_description}`;
			if(Number.isInteger(this.query_parameters.filter_tag_id)) url += `&filter_tag_id=${this.query_parameters.filter_tag_id}`;
			if(this.query_parameters.filter_mode_tag_id) url += `&filter_mode_tag_id=${this.query_parameters.filter_mode_tag_id}`;

			return url;
		},

		get_row(x: Asset, tags: Tag[]) {
			x.amount = x.amount ? x.amount : 0;
				x.value_per_unit = x.value_per_unit ? x.value_per_unit : {major: 0, minor: 0, minor_in_major: 100, symbol: "€"};
				x.currency = x.currency ? x.currency : {name: "Euro", minor_in_major: 100, symbol: "€"};

				return [
					x.id,
					x.name,
					x.description,
					Math.round(x.amount * 10000 + Number.EPSILON) / 10000,
					`${x.value_per_unit.major >= 0 && x.value_per_unit.is_negative ? "-" : ""}${x.value_per_unit.major}.${x.value_per_unit.minor.toString().padStart(x.value_per_unit.minor_in_major.toString().length - 1, "0")}${x.value_per_unit.symbol}`,
					`${((((x.value_per_unit.major * x.value_per_unit.minor_in_major) + x.value_per_unit.minor) * x.amount) / 100).toFixed(2)}${x.value_per_unit.symbol}`,
					x.total_cost_of_ownership ? `${(x.total_cost_of_ownership?.total.major ? x.total_cost_of_ownership?.total.major : 0) >= 0 && x.total_cost_of_ownership?.total.is_negative ? "-" : ""}${x.total_cost_of_ownership?.total.major}.${x.total_cost_of_ownership?.total.minor.toString().padStart(x.total_cost_of_ownership?.total.minor_in_major.toString().length - 1, "0")}${x.total_cost_of_ownership?.total.symbol}` : "",
					x.total_cost_of_ownership ? `${(x.total_cost_of_ownership?.monthly.major ? x.total_cost_of_ownership?.monthly.major : 0) >= 0 && x.total_cost_of_ownership?.monthly.is_negative ? "-" : ""}${x.total_cost_of_ownership?.monthly.major}.${x.total_cost_of_ownership?.monthly.minor.toString().padStart(x.total_cost_of_ownership?.monthly.minor_in_major.toString().length - 1, "0")}${x.total_cost_of_ownership?.monthly.symbol}` : "",
					x.total_cost_of_ownership ? `${(x.total_cost_of_ownership?.yearly.major ? x.total_cost_of_ownership?.yearly.major : 0) >= 0 && x.total_cost_of_ownership?.yearly.is_negative ? "-" : ""}${x.total_cost_of_ownership?.yearly.major}.${x.total_cost_of_ownership?.yearly.minor.toString().padStart(x.total_cost_of_ownership?.yearly.minor_in_major.toString().length - 1, "0")}${x.total_cost_of_ownership?.yearly.symbol}` : "",
					tags.filter(y => x.tag_ids?.includes(Number.isFinite(y.id) ? Number(y.id) : -1)).map(y => y.name).join(", ")
				];
		}
	},
}
</script>

<style lang="sass" scoped>
button#add
	margin: 10px
</style>
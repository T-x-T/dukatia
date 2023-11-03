<template>
	<div>
		<div v-if="chart_utilization_overview" class="gridItem line_chart">
			<h3>Current period utilization overview</h3>
			<ChartBar
				:bar="chart_utilization_overview"
			/>
		</div>
		<button class="green" @click="newBudget">Add</button>
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
		chart_utilization_overview: null as any
	}),

	async mounted() {
		const budgets = await $fetch("/api/v1/budgets/all") as Budget[];
		const tags = await $fetch("/api/v1/tags/all") as Tag[];
		const currencies = await $fetch("/api/v1/currencies/all") as Currency[];
		this.chart_utilization_overview = (await $fetch(`/api/v1/charts/bar/compute_all_budget_utilization_overview/data`)).bar;
		
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
				{name: "Rollover", type: "string"},
				{name: "Period", type: "choice", no_filter: true, options: [{id: 0, name: "Daily"}, {id: 1, name: "Weekly"}, {id: 2, name: "Monthly"}, {id: 3, name: "Quarterly"}, {id: 4, name: "Yearly"}]},
				{name: "Active from", type: "date"},
				{name: "Active to", type: "date"},
				{name: "Total Amount", type: "number"},
				{name: "Used Amount", type: "number", no_filter: true},
				{name: "Available Amount", type: "number", no_filter: true},
				{name: "Utilization", type: "number", no_filter: true},
				{name: "Filter Tags", type: "choice", options: tags.map(x => ({id: x.id, name: x.name}))},
			],
			rows: budgets.map(x => ([
				x.id,
				x.name,
				currencies.filter(y => y.id === x.currency_id)[0].name,
				x.rollover ? "Enabled" : "Disabled",
				x.period === 0 ? "Daily" : x.period === 1 ? "Weekly" : x.period === 2 ? "Monthly" : x.period === 3 ? "Quarterly" : "Yearly",
				new Date(new Date(x.active_from).valueOf() - (new Date(x.active_from).getTimezoneOffset() * 60000)).toISOString().slice(0, 10),
				x.active_to ? new Date(new Date(x.active_to).valueOf() - (new Date(x.active_to).getTimezoneOffset() * 60000)).toISOString().slice(0, 10) : null,
				`${x.amount.major >= 0 && x.amount.is_negative ? "-" : ""}${x.amount.major}.${x.amount.minor.toString().padStart(x.amount.minor_in_major.toString().length - 1, "0")}${x.amount.symbol}`,
				`${x.used_amount === undefined ? "" : x.used_amount.major >= 0 && x.used_amount.is_negative ? "-" : ""}${x.used_amount === undefined ? "0": x.used_amount.major}.${(x.used_amount === undefined ? 0 : x.used_amount.minor).toString().padStart(x.amount.minor_in_major.toString().length - 1, "0")}${x.amount.symbol}`,
				`${x.available_amount === undefined ? "" : x.available_amount.major >= 0 && x.available_amount.is_negative ? "-" : ""}${x.available_amount === undefined ? "0": x.available_amount.major}.${(x.available_amount === undefined ? 0 : x.available_amount.minor).toString().padStart(x.amount.minor_in_major.toString().length - 1, "0")}${x.amount.symbol}`,
				((x.utilization ? x.utilization : 0) * 100).toFixed(2) + "%",
				tags.filter(y => x.filter_tag_ids?.includes(Number.isInteger(y.id) ? Number(y.id) : -1)).map(y => y.name).join(", ")
			]))
		};
	},

	methods: {
		async rowClick(row: Row) {
			await useRouter().push(`/budgets/${row[0]}`);
		},

		async newBudget() {
			await useRouter().push("/budgets/new");
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
				case "total amount": {
					this.query_parameters.filter_amount = value;
					this.query_parameters.filter_mode_amount = mode;
					break;
				}
				case "rollover": {
					this.query_parameters.filter_rollover = value;
					this.query_parameters.filter_mode_rollover = mode;
					break;
				}
				case "filter_tags": {
					this.query_parameters.filter_filter_tag_id = value;
					this.query_parameters.filter_mode_filter_tag_id = mode;
					break;
				}
				case "active from": {
					this.query_parameters.filter_lower_active_from = value;
					this.query_parameters.filter_upper_active_from = value;
					this.query_parameters.filter_mode_active_from = mode;
					break;
				}
				case "active to": {
					this.query_parameters.filter_lower_active_to = value;
					this.query_parameters.filter_upper_active_to = value;
					this.query_parameters.filter_mode_active_to = mode;
					break;
				}
				case "currency": {
					this.query_parameters.filter_currency_id = value;
					this.query_parameters.filter_mode_currency_id = mode;
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
				case "amount": {
					this.query_parameters.filter_amount = undefined;
					this.query_parameters.filter_mode_amount = undefined;
					break;
				}
				case "rollover": {
					this.query_parameters.filter_rollover = undefined;
					this.query_parameters.filter_mode_rollover = undefined;
					break;
				}
				case "filter_tags": {
					this.query_parameters.filter_filter_tag_id = undefined;
					this.query_parameters.filter_mode_filter_tag_id = undefined;
					break;
				}
				case "active_from": {
					this.query_parameters.filter_lower_active_from = undefined;
					this.query_parameters.filter_upper_active_from = undefined;
					this.query_parameters.filter_mode_active_from = undefined;
					break;
				}
				case "active_to": {
					this.query_parameters.filter_lower_active_to = undefined;
					this.query_parameters.filter_upper_active_to = undefined;
					this.query_parameters.filter_mode_active_to = undefined;
					break;
				}
				case "currency_id": {
					this.query_parameters.filter_currency_id = undefined;
					this.query_parameters.filter_mode_currency_id = undefined;
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
			const budgets = await $fetch(this.build_request_url("/api/v1/budgets/all")) as Budget[];
			const tags = await $fetch("/api/v1/tags/all") as Tag[];
			const currencies = await $fetch("/api/v1/currencies/all") as Currency[];
			if(this.data_revision > local_data_revision) return;

			this.tableData.rows = budgets.map(x => ([
				x.id,
				x.name,
				currencies.filter(y => y.id === x.currency_id)[0].name,
				x.rollover ? "Enabled" : "Disabled",
				x.period === 0 ? "Daily" : x.period === 1 ? "Weekly" : x.period === 2 ? "Monthly" : x.period === 3 ? "Quarterly" : "Yearly",
				new Date(new Date(x.active_from).valueOf() - (new Date(x.active_from).getTimezoneOffset() * 60000)).toISOString().slice(0, 10),
				x.active_to ? new Date(new Date(x.active_to).valueOf() - (new Date(x.active_to).getTimezoneOffset() * 60000)).toISOString().slice(0, 10) : null,
				`${x.amount.major >= 0 && x.amount.is_negative ? "-" : ""}${x.amount.major}.${x.amount.minor.toString().padStart(x.amount.minor_in_major.toString().length - 1, "0")}${x.amount.symbol}`,
				`${x.used_amount === undefined ? "" : x.used_amount.major >= 0 && x.used_amount.is_negative ? "-" : ""}${x.used_amount === undefined ? "0": x.used_amount.major}.${(x.used_amount === undefined ? 0 : x.used_amount.minor).toString().padStart(x.amount.minor_in_major.toString().length - 1, "0")}${x.amount.symbol}`,
				`${x.available_amount === undefined ? "" : x.available_amount.major >= 0 && x.available_amount.is_negative ? "-" : ""}${x.available_amount === undefined ? "0": x.available_amount.major}.${(x.available_amount === undefined ? 0 : x.available_amount.minor).toString().padStart(x.amount.minor_in_major.toString().length - 1, "0")}${x.amount.symbol}`,
				((x.utilization ? x.utilization : 0) * 100).toFixed(2) + "%",
				tags.filter(y => x.filter_tag_ids?.includes(Number.isInteger(y.id) ? Number(y.id) : -1)).map(y => y.name).join(", ")
			]));
		},

		build_request_url(base_url: string) {
			let url = `${base_url}
				?skip_results=${this.query_parameters.skip_results}
				&max_results=${this.query_parameters.max_results}`;

			if(Number.isInteger(this.query_parameters.filter_id)) url += `&filter_id=${this.query_parameters.filter_id}`;
			if(this.query_parameters.filter_mode_id) url += `&filter_mode_id=${this.query_parameters.filter_mode_id}`;
			if(this.query_parameters.filter_name) url += `&filter_name=${this.query_parameters.filter_name}`;
			if(this.query_parameters.filter_mode_name) url += `&filter_mode_name=${this.query_parameters.filter_mode_name}`;
			if(Number.isInteger(this.query_parameters.filter_amount)) url += `&filter_amount=${this.query_parameters.filter_amount}`;
			if(this.query_parameters.filter_mode_amount) url += `&filter_mode_amount=${this.query_parameters.filter_mode_amount}`;
			if(this.query_parameters.filter_rollover) url += `&filter_rollover=${this.query_parameters.filter_rollover}`;
			if(this.query_parameters.filter_mode_rollover) url += `&filter_mode_rollover=${this.query_parameters.filter_mode_rollover}`;
			if(Number.isInteger(this.query_parameters.filter_filter_tag_id)) url += `&filter_filter_tag_id=${this.query_parameters.filter_filter_tag_id}`;
			if(this.query_parameters.filter_mode_filter_tag_id) url += `&filter_mode_filter_tag_id=${this.query_parameters.filter_mode_filter_tag_id}`;
			if(this.query_parameters.filter_lower_active_from) url += `&filter_lower_active_from=${new Date(this.query_parameters.filter_lower_active_from).toISOString()}`;
			if(this.query_parameters.filter_upper_active_from) url += `&filter_upper_active_from=${new Date(this.query_parameters.filter_upper_active_from).toISOString()}`;
			if(this.query_parameters.filter_mode_active_from) url += `&filter_mode_active_from=${this.query_parameters.filter_mode_active_from}`;
			if(this.query_parameters.filter_lower_active_to) url += `&filter_lower_active_to=${new Date(this.query_parameters.filter_lower_active_to).toISOString()}`;
			if(this.query_parameters.filter_upper_active_to) url += `&filter_upper_active_to=${new Date(this.query_parameters.filter_upper_active_to).toISOString()}`;
			if(this.query_parameters.filter_mode_active_to) url += `&filter_mode_active_to=${this.query_parameters.filter_mode_active_to}`;
			if(Number.isInteger(this.query_parameters.filter_currency_id)) url += `&filter_currency_id=${this.query_parameters.filter_currency_id}`;
			if(this.query_parameters.filter_mode_currency_id) url += `&filter_mode_currency_id=${this.query_parameters.filter_mode_currency_id}`;

			return url;
		},
	}
}
</script>

<style lang="sass" scoped>

h3
	text-align: center
	font-size: 1.5em

div.gridItem
	padding: 10px

div.pie_chart
	width: 20em
	height: 20em

div.line_chart
	width: 60em
	height: 20em

</style>
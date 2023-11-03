<template>
	<div>
		<button id="add" class="green" @click="newRecipient">Add</button>
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
		const recipients = await $fetch("/api/v1/recipients/all") as Recipient[];
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
				{name: "Tags", type: "choice", options: tags.map(x => ({id: x.id, name: x.name}))},
			],
			rows: recipients.map(x => ([
				x.id,
				x.name,
				tags.filter(y => x.tag_ids?.includes(Number.isInteger(y.id) ? Number(y.id) : -1)).map(y => y.name).join(", ")
			]))
		};
	},

	methods: {
		async rowClick(row: Row) {
			await useRouter().push(`/recipients/${row[0]}`);
		},

		async newRecipient() {
			await useRouter().push("/recipients/new");
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
			const recipients = await $fetch(this.build_request_url("/api/v1/recipients/all")) as Recipient[];
			const tags = await $fetch("/api/v1/tags/all") as Tag[];
			if(this.data_revision > local_data_revision) return;

			this.tableData.rows = recipients.map(x => ([
				x.id,
				x.name,
				tags.filter(y => x.tag_ids?.includes(Number.isInteger(y.id) ? Number(y.id) : -1)).map(y => y.name).join(", ")
			]))
		},

		build_request_url(base_url: string) {
			let url = `${base_url}
				?skip_results=${this.query_parameters.skip_results}
				&max_results=${this.query_parameters.max_results}`;

			if(Number.isInteger(this.query_parameters.filter_id)) url += `&filter_id=${this.query_parameters.filter_id}`;
			if(this.query_parameters.filter_mode_id) url += `&filter_mode_id=${this.query_parameters.filter_mode_id}`;
			if(Number.isInteger(this.query_parameters.filter_tag_id)) url += `&filter_tag_id=${this.query_parameters.filter_tag_id}`;
			if(this.query_parameters.filter_mode_tag_id) url += `&filter_mode_tag_id=${this.query_parameters.filter_mode_tag_id}`;
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
<template>
	<div id="wrapper">
		<button id="add" class="green" @click="newTag()">Add</button>
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
		const tags = (await useFetch("/api/v1/tags/all")).data.value as Tag[];

		this.tableData = {
			multiSelect: false,
			defaultSort: {
				column: 0,
				sort: "asc"
			},
			columns: [
				{name: "ID", type: "number", hidden: true},
				{name: "Name", type: "string"},
				{name: "Parent", type: "choice", options: tags.map(x => ({id: x.id, name: x.name}))},
			],
			rows: tags.map(x => ([
				x.id,
				x.name,
				tags.filter(y => y.id === x.parent_id)[0]?.name
			]))
		};
	},

	methods: {
		async rowClick(row: Row) {
			await useRouter().push(`/tags/${row[0]}`);
		},

		async newTag() {
			await useRouter().push("/tags/new");
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
				case "parent": {
					this.query_parameters.filter_parent_id = value;
					this.query_parameters.filter_mode_parent_id = mode;
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
				case "parent": {
					this.query_parameters.filter_parent_id = undefined;
					this.query_parameters.filter_mode_parent_id = undefined;
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
			const tags = await $fetch(this.build_request_url("/api/v1/tags/all")) as Tag[];
			const all_tags = await $fetch("/api/v1/tags/all") as Tag[];
			if(this.data_revision > local_data_revision) return;

			this.tableData.rows = tags.map(x => ([
				x.id,
				x.name,
				all_tags.filter(y => y.id === x.parent_id)[0]?.name
			]))
		},

		build_request_url(base_url: string) {
			let url = `${base_url}
				?skip_results=${this.query_parameters.skip_results}
				&max_results=${this.query_parameters.max_results}`;

			if(this.query_parameters.filter_id) url += `&filter_id=${this.query_parameters.filter_id}`;
			if(this.query_parameters.filter_mode_id) url += `&filter_mode_id=${this.query_parameters.filter_mode_id}`;
			if(this.query_parameters.filter_parent_id) url += `&filter_parent_id=${this.query_parameters.filter_parent_id}`;
			if(this.query_parameters.filter_mode_parent_id) url += `&filter_mode_parent_id=${this.query_parameters.filter_mode_parent_id}`;
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
	
div#wrapper
	width: 100%
</style>
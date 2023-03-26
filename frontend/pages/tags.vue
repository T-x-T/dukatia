<template>
	<div id="main">
		<div id="table" v-if="mode=='table'">
			<button class="green" @click="newTag()">Add</button>
			<CustomTable
				:tableData="tableData"
				v-on:rowClick="rowClick"
			/>
		</div>

		<div id="details" v-if="mode=='details'">
			<TagDetails
				:tag="selectedRow"
				v-on:back="updateAndLoadTable"
			/>
		</div>
	</div>
</template>

<script>
export default {
	data: () => ({
		tableData: {},
		selectedRow: {},
		mode: "table"
	}),

	async fetch() {
		await this.updateTags();
	},

	methods: {
		async updateTags() {
			this.tableData = {
				multiSelect: false,
				defaultSort: {
					column: 0,
					sort: "asc"
				},
				columns: [
					{name: "ID", type: "number"},
					{name: "Name", type: "string"},
					{name: "Parent", type: "string"}
				],
				rows: this.$store.state.tags.map(x => ([
					x.id,
					x.name,
					this.$store.state.tags.filter(y => y.id === x.parent_id)[0]?.name
				]))
			}
		},

		rowClick(row) {
			this.selectedRow = {...this.$store.state.tags.filter(x => x.id == row[0])[0]};
			this.mode = "details";
		},

		async newTag() {
			this.selectedRow = {
				id: "",
				name: ""
			}

			this.mode = "details";
		},

		async updateAndLoadTable() {
			await this.$store.dispatch("fetchTags");
			await this.updateTags();
			this.mode = "table";
		}
	}
}
</script>

<style lang="sass" scoped>
div#main
	height: 100vh
	overflow: scroll
</style>
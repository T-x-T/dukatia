<template>
	<div>
		<button class="green" @click="newTag()">Add</button>
		<CustomTable
			v-if="tableData"
			:tableData="tableData"
			v-on:rowClick="rowClick"
		/>
	</div>
</template>

<script>
export default {
	data: () => ({
		tableData: null
	}),

	async fetch() {
		await this.updateTags();
	},

	methods: {
		async updateTags() {
			await this.$store.dispatch("fetchTags");
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
			this.$router.push(`/tags/${row[0]}`);
		},

		async newTag() {
			this.$router.push("/tags/new");
		}
	}
}
</script>
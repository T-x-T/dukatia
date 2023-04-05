<template>
	<div>
		<button class="green" @click="newRecipient">Add</button>
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
		tableData: null,
	}),

	async fetch() {
		await this.updateRecipients();
	},

	methods: {
		async updateRecipients() {
			await this.$store.dispatch("fetchRecipients");
			this.tableData = {
				multiSelect: false,
				defaultSort: {
					column: 0,
					sort: "asc"
				},
				columns: [
					{name: "ID", type: "number"},
					{name: "Name", type: "string"},
					{name: "Tags", type: "choice", options: [...new Set(this.$store.state.tags.map(x => x.name))]}
				],
				rows: this.$store.state.recipients.map(x => ([
					x.id,
					x.name,
					this.$store.state.tags.filter(y => x.tag_ids?.includes(y.id)).map(y => y.name).join(", ")
				]))
			}
		},
		
		rowClick(row) {
			this.$router.push(`/recipients/${row[0]}`);
		},

		async newRecipient() {
			this.$router.push("/recipients/new");
		}
	}
}
</script>
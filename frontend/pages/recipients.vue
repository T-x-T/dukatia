<template>
	<div id="main">
		<div id="table" v-if="mode=='table'">
			<button class="green" @click="newRecipient">Add</button>
			<CustomTable
				:tableData="tableData"
				v-on:rowClick="rowClick"
			/>
		</div>

		<div id="details" v-if="mode=='details'">
			<RecipientDetails 
				:recipient="selectedRow"
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
		await this.updateRecipients();
	},

	methods: {
		async updateRecipients() {
			this.tableData = {
				multiSelect: false,
				defaultSort: {
					column: 0,
					sort: "asc"
				},
				columns: [
					{name: "ID", type: "number"},
					{name: "Name", type: "string"}
				],
				rows: this.$store.state.recipients.map(x => ([
					x.id,
					x.name
				]))
			}
		},
		
		rowClick(row) {
			this.selectedRow = {...this.$store.state.recipients.filter(x => x.id == row[0])[0]};
			this.mode = "details";
		},

		async newRecipient() {
			this.selectedRow = {
				id: "",
				name: ""
			}

			this.mode = "details";
		},

		async updateAndLoadTable() {
			await this.$store.dispatch("fetchRecipients");
			await this.updateRecipients();
			this.mode = "table";
		}
	}
}
</script>
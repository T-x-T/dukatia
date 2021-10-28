<template>
	<div id="main">
		<CustomTable
			:tableData="tableData"
		/>

		<form @submit.prevent="addRecipient">
			<label for="name">Name:</label>
			<input type="text" id="name" v-model="name">
			<button type="submit">Add</button>
		</form>
	</div>
</template>

<script>
export default {
	data: () => ({
		tableData: {},
		name: ""
	}),

	async fetch() {
		await this.updateRecipients();
	},

	methods: {
		async updateRecipients() {
			this.tableData = {
				headers: [
					"ID", "Name"
				],
				rows: this.$store.state.recipients.map(x => ([
					x.id,
					x.name
				]))
			}
		},

		async addRecipient(){
			await this.$axios.$post("/api/v1/recipients", {
				name: this.name
			});

			await this.$store.dispatch("fetchRecipients");
			await this.updateRecipients();
		}
	}
}
</script>
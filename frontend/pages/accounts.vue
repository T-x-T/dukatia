<template>
	<div id="main">
		<CustomTable
			:tableData="tableData"
		/>

		<form @submit.prevent="addAccount">
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
		await this.updateAccounts();
	},

	methods: {
		async updateAccounts() {
			this.tableData = {
				headers: [
					"ID", "Name", "Default Currency"
				],
				rows: this.$store.state.accounts.map(x => ([
					x.id,
					x.name,
					this.$store.state.currencies.filter(c => c.id == x.defaultCurrency)[0].name
				]))
			}
		},

		async addAccount() {
			await this.$axios.$post("/api/v1/accounts", {
				name: this.name,
				defaultCurrency: 0
			});

			await this.$store.dispatch("fetchAccounts");
			await this.updateAccounts();
		}
	}
}
</script>
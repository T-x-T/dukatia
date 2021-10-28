<template>
	<div id="main">
		<div id="table" v-if="mode=='table'">
			<CustomTable
				:tableData="tableData"
				v-on:rowClick="rowClick"
			/>
			<form @submit.prevent="addTransaction">
				<label for="amount">Amount:</label>
				<input id="amount" type="number" step="0.01" v-model="amount">
				
				<label for="comment">Comment:</label>
				<input id="comment" type="text" v-model="comment">
				
				<label for="account">Account:</label>
				<select id="account" v-model="account">
					<option v-for="(account, index) in $store.state.accounts" :key="index" :value="account.id">{{account.name}}</option>
				</select>
				
				<label for="currency">Currency:</label>
				<select id="account" v-model="currency">
					<option v-for="(currency, index) in $store.state.currencies" :key="index" :value="currency.id">{{currency.name}}</option>
				</select>
				
				<label for="recipient">Recipient:</label>
				<select id="account" v-model="recipient">
					<option v-for="(recipient, index) in $store.state.recipients" :key="index" :value="recipient.id">{{recipient.name}}</option>
				</select>
				
				<button type="submit">Add</button>
			</form>
		</div>

		<div v-if="mode=='details'" id="details">
			<button @click="mode='table'">Back</button>
			<p>{{selectedRow}}</p>
		</div>

	</div>
</template>

<script>
export default {
	data: () => ({
		amount: 0,
		comment: "",
		account: 0,
		currency: 0,
		recipient: 0,
		tableData: {},
		mode: "table",
		selectedRow: {}
	}),

	async fetch() {
		await this.updateTransactions();
	},

	methods: {
		async updateTransactions() {
			const transactionsForDisplay = this.$store.state.transactions.map(x => {
				x.account = this.$store.state.accounts.filter(a => a.id == x.accountId)[0];
				x.currency = this.$store.state.currencies.filter(c => c.id == x.currencyId)[0];
				x.recipient = this.$store.state.recipients.filter(r => r.id == x.recipientId)[0];
				return x;
			});

			this.tableData = {
				headers: [
					"ID", "User", "Account", "Recipient", "Status", "Timestamp", "Amount", "Comment"
				],
				rows: transactionsForDisplay.map(x => ([
					x.id,
					x.userId,
					x.account.name,
					x.recipient.name,
					x.status === 1 ? "Completed" : "Withheld",
					new Date(x.timestamp).toISOString().substring(0, 10),
					`${x.amount / x.currency.minorinmayor}${x.currency.symbol}`,
					x.comment
				]))
			}
		},

		async addTransaction() {
			await this.$axios.$post("/api/v1/transactions", {
				accountId: this.account,
				currencyId: this.currency,
				recipientId: this.recipient,
				timestamp: new Date(),
				amount: Number(this.amount) * 100,
				comment: this.comment
			});
			await this.$store.dispatch("fetchTransactions");
			await this.updateTransactions();
		},

		rowClick(row) {
			this.mode = "details";
			this.selectedRow = this.$store.state.transactions.filter(x => x.id == row[0])[0];
		}
	}
}
</script>

<style lang="sass" scoped>
@import "assets/_vars.sass"

</style>
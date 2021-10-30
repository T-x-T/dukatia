<template>
	<div id="wrapper">
		<button @click="$emit('back')">Back</button>

		<div id="details">
			<label for="id">ID:</label>
			<input type="text" id="id" disabled v-model="recipient.id">
			<br>
			<label for="name">Name:</label>
			<input type="text" id="name" v-model="recipient.name">
			<br>
			<button class="green" @click="sendRecipient">Save</button>
			<button class="red" @click="$emit('back')">Cancel</button>	
		</div>

		<div id="table">
			<CustomTable
				:tableData="tableData"
			/>
		</div>
	</div>	
</template>

<script>
export default {
	data: () => ({
		tableData: {}
	}),

	props: {
		recipient: Object
	},

	mounted() {
		const transactionsForDisplay = this.$store.state.transactions.filter(x => x.recipientId == this.recipient.id).map(x => {
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

	methods: {
		async sendRecipient() {
			const recipientData = {
				name: this.recipient.name
			}

			if(this.recipient.id) {
				await this.$axios.$put(`/api/v1/recipients/${this.recipient.id}`, recipientData);
			} else {
				await this.$axios.$post("/api/v1/recipients", recipientData);
			}

			this.$emit("back");
		}
	}
}
</script>

<style lang="sass" scoped>
div#details
	margin: 20px 0px 20px
	button
		margin-top: 10px
</style>
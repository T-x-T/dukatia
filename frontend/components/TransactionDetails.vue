<template>
	<div id="wrapper">
		<button @click="$emit('back')">Back</button>
		<button class="red" @click="deleteTransaction">Delete</button>

		<div id="details">
			<label for="id">ID:</label>
			<input type="text" id="id" disabled v-model="transaction.id">
			<br>
			<label for="account">Account:</label>
			<select id="account" v-model="transaction.accountId" @change="updateAccount">
				<option v-for="(account, index) in $store.state.accounts" :key="index" :value="account.id">{{account.name}}</option>
			</select>
			<br>
			<label for="recipient">Recipient:</label>
			<select id="account" v-model="transaction.recipientId">
				<option v-for="(recipient, index) in $store.state.recipients" :key="index" :value="recipient.id">{{recipient.name}}</option>
			</select>
			<br>
			<label for="status">Status:</label>
			<select id="status" v-model="transaction.status">
				<option value="0">Withheld</option>
				<option value="1">Completed</option>
			</select>
			<br>
			<label for="timestamp">Timestamp:</label>
			<input type="datetime" id="timestamp" v-model="transaction.timestamp">
			<br>
			<label for="amount">Amount:</label>
			<input type="number" id="amount" step="0.01" v-model="transaction.amount"><span>{{transaction.currency.symbol}}</span>
			<br>
			<label for="comment">Comment:</label>
			<input type="text" id="comment" v-model="transaction.comment">
			<CustomSelect
				:selectData="selectData"
				v-on:update="tagUpdate"
			/>
			<br>
			<button class="green" @click="sendTransaction">Save</button>
			<button class="orange" @click="$emit('back')">Cancel</button>
		</div>
	</div>
</template>

<script>
export default {
	data: () => ({
		selectData: {}
	}),
	props: {
		transaction: Object
	},

	created() {
		this.transaction.tagIds = Array.isArray(this.transaction.tagIds) ? [...this.transaction.tagIds] : [null]
		this.selectData = {
			options: [...this.$store.state.tags.map(x => ({id: x.id, name: x.name}))],
			selected: this.transaction.tagIds ? [...this.transaction.tagIds] : undefined,
			label: "Tags:"
		}
	},

	methods: {
		async deleteTransaction() {
			await this.$axios.$delete(`/api/v1/transactions/${this.transaction.id}`);

			this.$emit("back");
		},

		async sendTransaction() {
			const transactionData = {
				accountId: this.transaction.accountId,
				recipientId: this.transaction.recipientId,
				currencyId: this.transaction.currencyId,
				status: this.transaction.status,
				timestamp: this.transaction.timestamp,
				amount: this.transaction.amount * 100,
				comment: this.transaction.comment,
				tagIds: Array.isArray(this.transaction.tagIds) && typeof this.transaction.tagIds[0] == "number" ? this.transaction.tagIds : undefined
			}

			if(typeof this.transaction.id == "number") {
				await this.$axios.$put(`/api/v1/transactions/${this.transaction.id}`, transactionData);
			} else {
				await this.$axios.$post("/api/v1/transactions", transactionData);
			}

			this.$emit("back");
		},

		updateAccount() {
			this.transaction.account = this.$store.state.accounts.filter(x => x.id === this.transaction.accountId)[0];
			this.transaction.currency = this.$store.state.currencies.filter(x => x.id === this.transaction.account.defaultCurrency)[0];
		},

		tagUpdate(selected) {
			this.transaction.tagIds = selected;
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
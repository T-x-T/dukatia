<template>
	<div id="wrapper">
		<button @click="$emit('back')">Back</button>

		<div id="details">
			<label for="id">ID:</label>
			<input type="text" id="id" disabled v-model="account.id">
			<br>
			<label for="name">Name:</label>
			<input type="text" id="name" v-model="account.name">
			<br>
			<label for="currency">Currency:</label>
			<select id="currency" v-model="account.defaultCurrency.id">
				<option v-for="(currency, index) in $store.state.currencies" :key="index" :value="currency.id">{{currency.name}}</option>
			</select>
			<CustomSelect
				:selectData="selectData"
				v-on:update="tagUpdate"
			/>
			<br>
			<button class="green" @click="sendAccount">Save</button>
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
		tableData: {},
		selectData: {}
	}),

	props: {
		account: Object
	},

	created() {
		this.account.tagIds = Array.isArray(this.account.tagIds) ? [...this.account.tagIds] : [null];
		this.selectData = {
			options: [...this.$store.state.tags.map(x => ({id: x.id, name: x.name}))],
			selected: this.account.tagIds ? [...this.account.tagIds] : undefined,
			label: "Tags:"
		}

		const transactionsForDisplay = this.$store.state.transactions.filter(x => x.accountId == this.account.id).map(x => {
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
		async sendAccount() {
			const accountData = {
				name: this.account.name,
				defaultCurrency: this.account.defaultCurrency.id,
				tagIds: Array.isArray(this.account.tagIds) && typeof this.account.tagIds[0] == "number" ? this.account.tagIds : undefined
			}
			console.log(accountData)
			if(typeof this.account.id == "number") {
				await this.$axios.$put(`/api/v1/accounts/${this.account.id}`, accountData);
			} else {
				await this.$axios.$post("/api/v1/accounts", accountData);
			}

			this.$emit("back");
		},
		
		tagUpdate(selected) {
			this.account.tagIds = selected;
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
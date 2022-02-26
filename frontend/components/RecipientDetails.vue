<template>
	<div id="wrapper">
		<button @click="$emit('back')">Back</button>

		<div id="details">
			<label for="id">ID:</label>
			<input type="text" id="id" disabled v-model="recipient.id">
			<br>
			<label for="name">Name:</label>
			<input type="text" id="name" v-model="recipient.name">
			<CustomSelect
				:selectData="selectData"
				v-on:update="tagUpdate"
			/>
			<br>
			<button class="green" @click="sendRecipient(true)">Save</button>
			<button class="red" @click="$emit('back')">Cancel</button>	
			<button class="green" @click="sendRecipient(false)">Save and New</button>
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
		recipient: Object
	},

	created() {
		this.recipient.tagIds = Array.isArray(this.recipient.tagIds) ? [...this.recipient.tagIds] : [null]
		this.selectData = {
			options: [...this.$store.state.tags.map(x => ({id: x.id, name: x.name}))],
			selected: this.recipient.tagIds ? [...this.recipient.tagIds] : undefined,
			label: "Tags:"
		}

		const transactionsForDisplay = this.$store.state.transactions.filter(x => x.recipientId == this.recipient.id).map(x => {
			x.account = this.$store.state.accounts.filter(a => a.id == x.accountId)[0];
			x.currency = this.$store.state.currencies.filter(c => c.id == x.currencyId)[0];
			x.recipient = this.$store.state.recipients.filter(r => r.id == x.recipientId)[0];
			return x;
		});

		this.tableData = {
			multiSelect: false,
			defaultSort: {
				column: 0,
				sort: "asc"
			},
			columns: [
				{name: "ID", type: "number"},
				{name: "Account", type: "choice", options: [...new Set(this.$store.state.accounts.map(x => x.name))]},
				{name: "Recipient", type: "choice", options: [...new Set(this.$store.state.recipients.map(x => x.name))]},
				{name: "Timestamp", type: "date"},
				{name: "Amount", type: "number"},
				{name: "Comment", type: "string"},
				{name: "Tags", type: "choice", options: [...new Set(this.$store.state.tags.map(x => x.name))]}
			],
			rows: transactionsForDisplay.map(x => ([
				x.id,
				x.account.name,
				x.recipient.name,
				new Date(x.timestamp).toISOString().substring(0, 10),
				`${x.amount / x.currency.minorinmayor}${x.currency.symbol}`,
				x.comment,
				this.$store.state.tags.filter(y => x.tagIds?.includes(y.id)).map(y => y.name).join(", ")
			]))
		}
	},

	methods: {
		async sendRecipient(goBack) {
			const recipientData = {
				id: this.recipient.id,
				name: this.recipient.name,
				tagIds: Array.isArray(this.recipient.tagIds) && typeof this.recipient.tagIds[0] == "number" ? this.recipient.tagIds : undefined
			}

			if(typeof this.recipient.id == "number") {
				await this.$axios.$put(`/api/v1/recipients/${this.recipient.id}`, recipientData);
			} else {
				await this.$axios.$post("/api/v1/recipients", recipientData);
			}

			if(goBack) {
				this.$emit("back");
			} else {
				this.recipient = {
					id: "",
					name: ""
				}
			}
		},

		tagUpdate(selected) {
			this.recipient.tagIds = selected;
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
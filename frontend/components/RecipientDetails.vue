<template>
	<div id="wrapper">
		<DetailsPage
			:config="config"
			v-on:back="$emit('back')"
		/>
	</div>	
</template>

<script>
export default {
	data: () => ({
		config: {}
	}),

	props: {
		recipient: Object
	},

	created() {
		this.recipient.tagIds = Array.isArray(this.recipient.tagIds) ? [...this.recipient.tagIds] : [null]

		const transactionsForDisplay = this.$store.state.transactions.filter(x => x.recipientId == this.recipient.id).map(x => {
			x.account = this.$store.state.accounts.filter(a => a.id == x.accountId)[0];
			x.currency = this.$store.state.currencies.filter(c => c.id == x.currencyId)[0];
			x.recipient = this.$store.state.recipients.filter(r => r.id == x.recipientId)[0];
			return x;
		});

		this.config = {
			...this.$detailPageConfig.recipient,
			data: this.recipient,
			tableData: {
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
		}
	}
}
</script>
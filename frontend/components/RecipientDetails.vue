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
		this.recipient.tag_ids = Array.isArray(this.recipient.tag_ids) ? [...this.recipient.tag_ids] : [null]

		const transactionsForDisplay = this.$store.state.transactions.filter(x => x.recipient_id == this.recipient.id).map(x => {
			x.account = this.$store.state.accounts.filter(a => a.id == x.account_id)[0];
			x.currency = this.$store.state.currencies.filter(c => c.id == x.currency_id)[0];
			x.recipient = this.$store.state.recipients.filter(r => r.id == x.recipient_id)[0];
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
					`${x.amount / x.currency.minor_in_mayor}${x.currency.symbol}`,
					x.comment,
					this.$store.state.tags.filter(y => x.tag_ids?.includes(y.id)).map(y => y.name).join(", ")
				]))
			}
		}
	}
}
</script>
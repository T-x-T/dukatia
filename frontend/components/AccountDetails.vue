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
		account: Object
	},

	created() {
		this.account.defaultCurrencyId = this.account.defaultCurrency.id;
		this.account.tagIds = Array.isArray(this.account.tagIds) ? [...this.account.tagIds] : [null];

		const transactionsForDisplay = this.$store.state.transactions.filter(x => x.accountId == this.account.id).map(x => {
			x.account = this.$store.state.accounts.filter(a => a.id == x.accountId)[0];
			x.currency = this.$store.state.currencies.filter(c => c.id == x.currencyId)[0];
			x.recipient = this.$store.state.recipients.filter(r => r.id == x.recipientId)[0];
			return x;
		});

		this.config = {
			...this.$detailPageConfig.account,
			data: this.account,
			resetDefaultCurrencyId: true,
			tableData : {
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
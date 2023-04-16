<template>
	<div id="wrapper">
		<DetailsPage
			:config="config"
			v-on:back="$emit('back')"
		/>
	</div>	
</template>

<script lang="ts">
export default {
	data: () => ({
		config: {},
		tags: [],
		recipients: [],
		accounts: [],
		transactions: [],
		currencies: []
	}),

	props: {
		recipient: Object
	},

	async created() {
		this.tags = await $fetch("/api/v1/tags/all");
		this.recipients = await $fetch("/api/v1/recipients/all");
		this.accounts = await $fetch("/api/v1/accounts/all");
		this.transactions = await $fetch("/api/v1/transactions/all");
		this.currencies = await $fetch("/api/v1/currencies/all");
		(this as any).recipient.tag_ids = Array.isArray((this as any).recipient.tag_ids) ? [...(this as any).recipient.tag_ids] : [null]

		const transactionsForDisplay = this.transactions.filter((x: any) => x.recipient_id == (this as any).recipient.id).map((x: any) => {
			x.account = this.accounts.filter((a: any) => a.id == x.account_id)[0];
			x.currency = this.currencies.filter((c: any) => c.id == x.currency_id)[0];
			x.recipient = this.recipients.filter((r: any) => r.id == x.recipient_id)[0];
			return x;
		});

		this.config = {
			...this.$detailPageConfig().recipient,
			data: this.recipient,
			tableData: {
				multiSelect: false,
				defaultSort: {
					column: 0,
					sort: "asc"
				},
				columns: [
					{name: "ID", type: "number"},
					{name: "Account", type: "choice", options: [...new Set(this.accounts.map((x: any) => x.name))]},
					{name: "Recipient", type: "choice", options: [...new Set(this.recipients.map((x: any) => x.name))]},
					{name: "Timestamp", type: "date"},
					{name: "Amount", type: "number"},
					{name: "Comment", type: "string"},
					{name: "Tags", type: "choice", options: [...new Set(this.tags.map((x: any) => x.name))]}
				],
				rows: transactionsForDisplay.map(x => ([
					x.id,
					x.account.name,
					x.recipient.name,
					new Date(x.timestamp).toISOString().substring(0, 10),
					`${x.amount / x.currency.minor_in_mayor}${x.currency.symbol}`,
					x.comment,
					this.tags.filter((y: any) => x.tag_ids?.includes(y.id)).map((y: any) => y.name).join(", ")
				]))
			}
		}
	}
}
</script>
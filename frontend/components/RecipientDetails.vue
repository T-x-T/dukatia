<template>
	<div id="wrapper">
		<DetailsPage
			v-if="Object.keys(config).length > 0"
			:config="config"
			v-on:back="$emit('back')"
		/>
	</div>	
</template>

<script lang="ts">
export default {
	data: () => ({
		config: {} as DetailFormConfig,
		tags: [] as Tag[],
		recipients: [] as Recipient[],
		accounts: [] as Account[],
		transactions: [] as Transaction[],
		currencies: [] as Currency[],
	}),

	props: {
		recipient: {
			type: Object as PropType<Recipient>,
			required: true,
		}
	},

	async created() {
		this.tags = await $fetch("/api/v1/tags/all");
		this.recipients = await $fetch("/api/v1/recipients/all");
		this.accounts = await $fetch("/api/v1/accounts/all");
		this.transactions = await $fetch("/api/v1/transactions/all");
		this.currencies = await $fetch("/api/v1/currencies/all");
		this.recipient.tag_ids = Array.isArray(this.recipient.tag_ids) ? [...this.recipient.tag_ids] : []

		const transactionsForDisplay = this.transactions.filter(x => x.recipient_id == this.recipient.id).map(x => {
			x.account = this.accounts.filter(a => a.id == x.account_id)[0];
			x.currency = this.currencies.filter(c => c.id == x.currency_id)[0];
			x.recipient = this.recipients.filter(r => r.id == x.recipient_id)[0];
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
					{name: "Account", type: "choice", options: [...new Set(this.accounts.map(x => x.name))]},
					{name: "Recipient", type: "choice", options: [...new Set(this.recipients.map(x => x.name))]},
					{name: "Timestamp", type: "date"},
					{name: "Amount", type: "number"},
					{name: "Comment", type: "string"},
					{name: "Tags", type: "choice", options: [...new Set(this.tags.map(x => x.name))]}
				],
				rows: transactionsForDisplay.map(x => {
					if(!x.account || !x.recipient || !x.currency) return [];
					return [
						x.id,
						x.account.name,
						x.recipient.name,
						new Date(x.timestamp).toISOString().substring(0, 10),
						`${x.amount / x.currency.minor_in_mayor}${x.currency.symbol}`,
						x.comment,
						this.tags.filter(y => x.tag_ids?.includes(y.id ? y.id : -1)).map(y => y.name).join(", ")
					];
				})
			}
		}
	}
}
</script>
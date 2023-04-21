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
		config: {}
	}),

	props: {
		account: {
			type: Object as PropType<Account>,
			required: true,
		}
	},

	async created() {
		const transactions = await $fetch("/api/v1/transactions/all") as Transaction[];
		const accounts = await $fetch("/api/v1/accounts/all") as Account[];
		const currencies = await $fetch("/api/v1/currencies/all") as Currency[];
		const recipients = await $fetch("/api/v1/recipients/all") as Recipient[];
		const tags = await $fetch("/api/v1/tags/all") as Tag[];

		this.account.tag_ids = Array.isArray(this.account.tag_ids) ? [...this.account.tag_ids] : [];

		const transactionsForDisplay = transactions.filter(x => x.account_id == this.account.id).map(x => {
			x.account = accounts.filter(a => a.id == x.account_id)[0];
			x.currency = currencies.filter(c => c.id == x.currency_id)[0];
			x.recipient = recipients.filter(r => r.id == x.recipient_id)[0];
			return x;
		});

		this.config = {
			...this.$detailPageConfig().account,
			data: this.account,
			resetdefault_currency_id: true,
			tableData : {
				multiSelect: false,
				defaultSort: {
					column: 0,
					sort: "asc"
				},
				columns: [
					{name: "ID", type: "number"},
					{name: "Account", type: "choice", options: [...new Set(accounts.map(x => x.name))]},
					{name: "Recipient", type: "choice", options: [...new Set(recipients.map(x => x.name))]},
					{name: "Timestamp", type: "date"},
					{name: "Amount", type: "number"},
					{name: "Comment", type: "string"},
					{name: "Tags", type: "choice", options: [...new Set(tags.map(x => x.name))]}
				],
				rows: transactionsForDisplay.map(x => {
					if(!x.account || !x.recipient || !x.currency) return;
					return [
						x.id,
						x.account.name,
						x.recipient.name,
						new Date(x.timestamp).toISOString().substring(0, 10),
						`${x.amount / x.currency.minor_in_mayor}${x.currency.symbol}`,
						x.comment,
						tags.filter(y => x.tag_ids?.includes(y.id ? y.id : -1)).map(y => y.name).join(", ")
					]
				})
			}
		}
	}
}
</script>